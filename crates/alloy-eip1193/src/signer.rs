use alloy::hex;
use alloy::network::{TxSigner, NetworkWallet, Ethereum};
use alloy::primitives::{Address, Signature, B256, ChainId};
use alloy::signers::Signer;
use alloy::consensus::SignableTransaction;
use alloy::dyn_abi::eip712::TypedData;
use alloy::providers::RootProvider;
use async_trait::async_trait;
use wasm_bindgen::prelude::*;

use crate::transport::Eip1193Transport;
use crate::ext::Eip1193 as Eip1193Ext;

/// EIP-1193 signer that uses browser wallet for signing operations only.
///
/// This signer wraps `window.ethereum` to provide signing capabilities
/// without acting as a full RPC provider. It should be combined with an
/// HTTP transport provider for blockchain RPC operations.
#[derive(Clone, Debug)]
pub struct Eip1193Signer {
    /// Transport for making EIP-1193 RPC calls
    transport: Eip1193Transport,
    /// Cached address of the currently connected account
    address: Address,
    /// Chain ID for EIP-155 transaction signing
    chain_id: Option<ChainId>,
}

// WASM is single-threaded, so Send/Sync are safe
unsafe impl Send for Eip1193Signer {}
unsafe impl Sync for Eip1193Signer {}

impl Eip1193Signer {
    /// Create a new EIP-1193 signer from the browser's ethereum provider.
    ///
    /// # Arguments
    /// * `ethereum` - The `window.ethereum` JavaScript object
    /// * `address` - The currently connected account address
    pub fn new(ethereum: JsValue, address: Address) -> Self {
        Self {
            transport: Eip1193Transport::new(ethereum),
            address,
            chain_id: None,
        }
    }

    /// Create a new EIP-1193 signer with a specific chain ID.
    ///
    /// # Arguments
    /// * `ethereum` - The `window.ethereum` JavaScript object
    /// * `address` - The currently connected account address
    /// * `chain_id` - The chain ID for EIP-155 signing
    pub fn new_with_chain_id(ethereum: JsValue, address: Address, chain_id: ChainId) -> Self {
        Self {
            transport: Eip1193Transport::new(ethereum),
            address,
            chain_id: Some(chain_id),
        }
    }

    /// Create a signer from the window.ethereum object.
    ///
    /// This will request account access if not already granted and fetch the current chain ID.
    /// Uses the modern RpcClient + RootProvider + ext::Eip1193 pattern.
    pub async fn from_window() -> Result<Self, JsValue> {
        let ethereum = Eip1193Transport::get_ethereum()?;
        let transport = Eip1193Transport::new(ethereum.clone());

        // Create RpcClient and RootProvider to use the Eip1193 trait extension
        let client = transport.clone().into_client();
        let provider = RootProvider::<Ethereum>::new(client);

        // Use the Eip1193 trait extension to request accounts
        let accounts = provider.request_accounts().await
            .map_err(|e| JsValue::from_str(&format!("Failed to request accounts: {:?}", e)))?;

        let address = accounts
            .first()
            .copied()
            .ok_or_else(|| JsValue::from_str("No accounts available"))?;

        // Fetch the current chain ID from the wallet
        let chain_id_hex: String = transport.request("eth_chainId", Vec::<String>::new()).await?;
        let chain_id = u64::from_str_radix(chain_id_hex.trim_start_matches("0x"), 16)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse chain ID: {}", e)))?;

        Ok(Self::new_with_chain_id(ethereum, address, chain_id))
    }

    /// Get the ethereum provider object
    pub fn ethereum(&self) -> &JsValue {
        self.transport.ethereum()
    }


    /// Refresh the chain ID from the wallet
    ///
    /// This queries the wallet's current chain via `eth_chainId` and updates
    /// the internal chain_id field.
    pub async fn refresh_chain_id(&mut self) -> Result<ChainId, JsValue> {
        let chain_id_hex: String = self.transport
            .request("eth_chainId", Vec::<String>::new())
            .await?;

        let chain_id = u64::from_str_radix(chain_id_hex.trim_start_matches("0x"), 16)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse chain ID: {}", e)))?;

        self.chain_id = Some(chain_id);
        Ok(chain_id)
    }

    /// Validate that the signer's chain ID matches the expected chain
    ///
    /// Returns an error if there's a mismatch between the wallet's current chain
    /// and the expected chain. Call `refresh_chain_id()` first to ensure the
    /// chain ID is up to date.
    pub fn validate_chain_id(&self, expected: ChainId) -> Result<(), JsValue> {
        if let Some(current) = self.chain_id {
            if current != expected {
                return Err(JsValue::from_str(&format!(
                    "Chain ID mismatch: wallet is on chain {}, expected chain {}",
                    current, expected
                )));
            }
        }
        Ok(())
    }

}

#[cfg(target_family = "wasm")]
#[async_trait(?Send)]
impl Signer<Signature> for Eip1193Signer {
    #[inline]
    async fn sign_hash(&self, hash: &B256) -> Result<Signature, alloy::signers::Error> {
        // eth_sign params: [address, message_hash]
        // Following Alloy's pattern of using tuples for RPC params
        let params = (
            format!("{:?}", self.address),
            format!("0x{}", hex::encode(hash)),
        );

        let sig_str: String = self.transport
            .request("eth_sign", params)
            .await
            .map_err(|e| alloy::signers::Error::other(format!("Sign hash failed: {:?}", e)))?;

        sig_str
            .parse()
            .map_err(|e| alloy::signers::Error::other(format!("Failed to parse signature: {}", e)))
    }

    #[inline]
    async fn sign_message(&self, message: &[u8]) -> Result<Signature, alloy::signers::Error> {
        // personal_sign params: [message, address]
        // Note: personal_sign uses a different order than eth_sign
        let params = (
            format!("0x{}", hex::encode(message)),
            format!("{:?}", self.address),
        );

        let sig_str: String = self.transport
            .request("personal_sign", params)
            .await
            .map_err(|e| alloy::signers::Error::other(format!("Sign message failed: {:?}", e)))?;

        sig_str
            .parse()
            .map_err(|e| alloy::signers::Error::other(format!("Failed to parse signature: {}", e)))
    }

    fn address(&self) -> Address {
        self.address
    }

    fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
    }

    fn set_chain_id(&mut self, chain_id: Option<ChainId>) {
        self.chain_id = chain_id;
    }

    /// Sign EIP-712 typed data using the browser wallet
    ///
    /// This forwards the typed data to the wallet via `eth_signTypedData_v4`,
    /// which provides a better UX as wallets can display the structured data
    /// to users before they sign.
    #[inline]
    async fn sign_dynamic_typed_data(&self, payload: &TypedData) -> Result<Signature, alloy::signers::Error> {
        // eth_signTypedData_v4 params: [address, typed_data_json]
        // Serialize the TypedData to a serde_json::Value
        let payload_json = serde_json::to_value(payload)
            .map_err(|e| alloy::signers::Error::other(format!("Failed to serialize TypedData: {}", e)))?;

        let params = (
            format!("{:?}", self.address),
            payload_json,
        );

        let sig_str: String = self.transport
            .request("eth_signTypedData_v4", params)
            .await
            .map_err(|e| alloy::signers::Error::other(format!("Sign typed data failed: {:?}", e)))?;

        sig_str
            .parse()
            .map_err(|e| alloy::signers::Error::other(format!("Failed to parse signature: {}", e)))
    }
}

/// Implement TxSigner for transaction signing
#[cfg(target_family = "wasm")]
#[async_trait(?Send)]
impl TxSigner<Signature> for Eip1193Signer {
    fn address(&self) -> Address {
        self.address
    }

    async fn sign_transaction(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> Result<Signature, alloy::signers::Error> {
        // CAVEAT: This uses eth_sign which shows warnings in MetaMask and most wallets
        // For production use, prefer Eip1193Provider which uses eth_sendTransaction
        // This fallback implementation is provided for API compatibility in edge cases

        log::warn!(
            "Using eth_sign for transaction signing. \
             MetaMask and other wallets will show security warnings. \
             For better UX, use Eip1193Provider with send_transaction override."
        );

        // Encode the transaction for signing
        let mut tx_encoded = Vec::new();
        tx.encode_for_signing(&mut tx_encoded);

        // Sign the transaction hash using eth_sign (will show scary warning)
        let tx_hash = alloy::primitives::keccak256(&tx_encoded);

        self.sign_hash(&tx_hash).await
    }
}

/// Implement NetworkWallet for Ethereum network
///
/// This allows the signer to be used with ProviderBuilder.
/// The implementation delegates to `TxSigner::sign_transaction` and wraps the result.
#[cfg(target_family = "wasm")]
#[async_trait(?Send)]
impl NetworkWallet<Ethereum> for Eip1193Signer {
    fn default_signer_address(&self) -> Address {
        self.address
    }

    fn has_signer_for(&self, address: &Address) -> bool {
        address == &self.address
    }

    fn signer_addresses(&self) -> impl Iterator<Item = Address> {
        std::iter::once(self.address)
    }

    #[allow(refining_impl_trait)]
    fn sign_transaction_from<'a>(
        &'a self,
        sender: Address,
        mut tx: <Ethereum as alloy::network::Network>::UnsignedTx,
    ) -> impl std::future::Future<Output = Result<<Ethereum as alloy::network::Network>::TxEnvelope, alloy::signers::Error>> + 'a {
        async move {
            if sender != self.address {
                return Err(alloy::signers::Error::other(
                    format!("Sender {} does not match signer address {}", sender, self.address)
                ));
            }

            // Delegate to TxSigner::sign_transaction
            let signature = TxSigner::sign_transaction(self, &mut tx).await?;

            // Wrap in envelope
            Ok(tx.into_signed(signature).into())
        }
    }
}
