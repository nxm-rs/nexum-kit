use alloy::hex;
use alloy::network::{TxSigner, NetworkWallet, Ethereum};
use alloy::primitives::{Address, Signature, B256, ChainId};
use alloy::signers::Signer;
use alloy::consensus::SignableTransaction;
use alloy::dyn_abi::eip712::TypedData;
use alloy::sol_types::SolStruct;
use alloy::dyn_abi::Eip712Domain;
use async_trait::async_trait;
use wasm_bindgen::prelude::*;

use super::eip1193::Eip1193Transport;
use super::request::{Eip1193Requester, EthSignParams, PersonalSignParams, SignTypedDataV4Params};

/// EIP-1193 signer that uses browser wallet for signing operations only.
///
/// This signer wraps `window.ethereum` to provide signing capabilities
/// without acting as a full RPC provider. It should be combined with an
/// HTTP transport provider for blockchain RPC operations.
#[derive(Clone, Debug)]
pub struct Eip1193Signer {
    /// Generic requester for making EIP-1193 RPC calls
    requester: Eip1193Requester,
    /// Cached address of the currently connected account
    address: Address,
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
            requester: Eip1193Requester::new(ethereum),
            address,
        }
    }

    /// Create a signer from the window.ethereum object.
    ///
    /// This will request account access if not already granted.
    pub async fn from_window() -> Result<Self, JsValue> {
        let ethereum = Eip1193Transport::get_ethereum()?;
        let requester = Eip1193Requester::new(ethereum.clone());

        // Request accounts to get the current address
        let empty_params: Vec<String> = Vec::new();
        let accounts: Vec<String> = requester.request("eth_requestAccounts", empty_params).await?;
        let address = accounts
            .first()
            .ok_or_else(|| JsValue::from_str("No accounts available"))?
            .parse()
            .map_err(|e| JsValue::from_str(&format!("Failed to parse address: {}", e)))?;

        Ok(Self::new(ethereum, address))
    }

    /// Get the ethereum provider object
    pub fn ethereum(&self) -> &JsValue {
        self.requester.ethereum()
    }
}

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
impl Signer<Signature> for Eip1193Signer {
    #[inline]
    async fn sign_hash(&self, hash: &B256) -> Result<Signature, alloy::signers::Error> {
        let params: EthSignParams = (
            format!("{:?}", self.address),
            format!("0x{}", hex::encode(hash)),
        );

        let sig_str: String = self.requester
            .request("eth_sign", params)
            .await
            .map_err(|e| alloy::signers::Error::other(format!("Sign hash failed: {:?}", e)))?;

        sig_str
            .parse()
            .map_err(|e| alloy::signers::Error::other(format!("Failed to parse signature: {}", e)))
    }

    #[inline]
    async fn sign_message(&self, message: &[u8]) -> Result<Signature, alloy::signers::Error> {
        let params: PersonalSignParams = (
            format!("0x{}", hex::encode(message)),
            format!("{:?}", self.address),
        );

        let sig_str: String = self.requester
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
        // Chain ID is not stored in the signer, as it's managed by the provider
        None
    }

    fn set_chain_id(&mut self, _chain_id: Option<ChainId>) {
        // Chain ID is managed by the wallet/provider, not the signer
        // This is a no-op
    }

    /// Sign EIP-712 typed data using the browser wallet
    ///
    /// This forwards the typed data to the wallet via `eth_signTypedData_v4`,
    /// which provides a better UX as wallets can display the structured data
    /// to users before they sign.
    #[inline]
    async fn sign_dynamic_typed_data(&self, payload: &TypedData) -> Result<Signature, alloy::signers::Error> {
        // Serialize the TypedData to a serde_json::Value
        let payload_json = serde_json::to_value(payload)
            .map_err(|e| alloy::signers::Error::other(format!("Failed to serialize TypedData: {}", e)))?;

        let params: SignTypedDataV4Params = (
            format!("{:?}", self.address),
            payload_json,
        );

        let sig_str: String = self.requester
            .request("eth_signTypedData_v4", params)
            .await
            .map_err(|e| alloy::signers::Error::other(format!("Sign typed data failed: {:?}", e)))?;

        sig_str
            .parse()
            .map_err(|e| alloy::signers::Error::other(format!("Failed to parse signature: {}", e)))
    }

    /// Sign statically-typed EIP-712 data
    ///
    /// This is a convenience method for compile-time known types that implement SolStruct.
    /// It constructs a TypedData and forwards to sign_dynamic_typed_data.
    #[inline]
    async fn sign_typed_data<T: SolStruct + Send + Sync>(
        &self,
        payload: &T,
        domain: &Eip712Domain,
    ) -> Result<Signature, alloy::signers::Error>
    where
        Self: Sized,
    {
        // Use the default implementation which computes the hash and calls sign_hash
        // This is simpler than reconstructing TypedData from SolStruct
        self.sign_hash(&payload.eip712_signing_hash(domain)).await
    }
}

/// Implement TxSigner for transaction signing
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
impl TxSigner<Signature> for Eip1193Signer {
    fn address(&self) -> Address {
        self.address
    }

    async fn sign_transaction(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> Result<Signature, alloy::signers::Error> {
        // Encode the transaction for signing
        let mut tx_encoded = Vec::new();
        tx.encode_for_signing(&mut tx_encoded);

        // Sign the transaction hash using eth_sign
        // Note: MetaMask and most wallets will show a scary warning for eth_sign
        // In production, you might want to use eth_sendTransaction and extract the signature
        // or use a different signing method
        let tx_hash = alloy::primitives::keccak256(&tx_encoded);

        self.sign_hash(&tx_hash).await
    }
}

/// Implement NetworkWallet for Ethereum network
/// This allows the signer to be used with ProviderBuilder
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
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
                return Err(alloy::signers::Error::other("Sender address does not match signer address"));
            }

            // Sign the transaction
            let signature = TxSigner::sign_transaction(self, &mut tx).await?;

            // Build the signed transaction envelope and convert to TxEnvelope
            Ok(tx.into_signed(signature).into())
        }
    }
}
