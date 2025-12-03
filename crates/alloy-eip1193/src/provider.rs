//! Provider layer for smart EIP-1193 routing
//!
//! Provides `WalletLayer` that can be added to any provider to route wallet operations
//! through EIP-1193 while keeping RPC reads on the original transport.

use alloy::providers::{Provider, ProviderBuilder, ProviderLayer, RootProvider, PendingTransactionBuilder, SendableTx};
use alloy::network::{Network, TransactionBuilder};
use alloy::transports::TransportResult;
use alloy::primitives::TxHash;
use std::marker::PhantomData;
use wasm_bindgen::JsValue;
use crate::{Eip1193Transport, Eip1193Error};

/// Layer that adds EIP-1193 wallet routing to any provider
///
/// Routes wallet operations to browser wallet, everything else to original transport
pub struct WalletLayer {
    ethereum: JsValue,
}

impl WalletLayer {
    /// Create new wallet layer
    pub fn new(ethereum: JsValue) -> Self {
        Self { ethereum }
    }

    /// Create from window.ethereum
    pub fn from_window() -> Result<Self, Eip1193Error> {
        let ethereum = Eip1193Transport::get_ethereum()?;
        Ok(Self::new(ethereum))
    }
}

impl<P, N> ProviderLayer<P, N> for WalletLayer
where
    P: Provider<N>,
    N: Network,
{
    type Provider = WalletProvider<P, N>;

    fn layer(&self, inner: P) -> Self::Provider {
        WalletProvider {
            inner,
            wallet_transport: Eip1193Transport::new(self.ethereum.clone()),
            _phantom: PhantomData,
        }
    }
}

/// Provider with EIP-1193 wallet routing
pub struct WalletProvider<P, N> {
    inner: P,
    wallet_transport: Eip1193Transport,
    _phantom: PhantomData<N>,
}

impl<P: Clone, N> Clone for WalletProvider<P, N> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            wallet_transport: self.wallet_transport.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<P, N> std::fmt::Debug for WalletProvider<P, N>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalletProvider")
            .field("inner", &self.inner)
            .finish()
    }
}

#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
impl<P, N> Provider<N> for WalletProvider<P, N>
where
    P: Provider<N>,
    N: Network,
{
    fn root(&self) -> &RootProvider<N> {
        self.inner.root()
    }

    /// Override send_transaction to route through EIP-1193 wallet
    ///
    /// This ensures transactions are sent via `eth_sendTransaction` through the browser wallet
    /// instead of `eth_sendRawTransaction` on the RPC provider. This allows the wallet to:
    /// - Request user approval for the transaction
    /// - Sign the transaction with the user's private key
    /// - Broadcast it through the wallet's preferred RPC
    async fn send_transaction_internal(
        &self,
        tx: SendableTx<N>,
    ) -> TransportResult<PendingTransactionBuilder<N>> {
        match tx {
            SendableTx::Builder(mut tx_request) => {
                // Prepare transaction for submission (set gas, nonce, etc. if needed)
                TransactionBuilder::prep_for_submission(&mut tx_request);

                // Send via EIP-1193 eth_sendTransaction
                // The transport layer will handle JSON serialization via serde_json + JSON.parse
                let tx_hash: TxHash = self.wallet_transport
                    .request("eth_sendTransaction", vec![&tx_request])
                    .await
                    .map_err(|e| alloy::transports::TransportErrorKind::custom_str(&format!("Transaction rejected: {:?}", e)))?;

                // Return a pending transaction builder
                Ok(PendingTransactionBuilder::new(self.root().clone(), tx_hash))
            }
            SendableTx::Envelope(envelope) => {
                // If we have a signed envelope, forward to inner provider
                // (this shouldn't happen in normal wallet flow)
                self.inner.send_transaction_internal(SendableTx::Envelope(envelope)).await
            }
        }
    }
}

impl<P, N> WalletProvider<P, N> {
    /// Get reference to wallet transport
    pub fn wallet_transport(&self) -> &Eip1193Transport {
        &self.wallet_transport
    }

    /// Get reference to inner provider
    pub fn inner(&self) -> &P {
        &self.inner
    }
}

/// Convenience function to create provider with wallet layer
///
/// # Example
/// ```rust,ignore
/// use alloy_eip1193::provider::with_wallet;
///
/// let provider = with_wallet(
///     ethereum_js,
///     "https://eth.llamarpc.com".parse()?,
/// )?;
/// ```
pub fn with_wallet(ethereum: JsValue, rpc_url: url::Url) -> Result<impl Provider, Eip1193Error> {
    let layer = WalletLayer::new(ethereum);
    let provider = ProviderBuilder::new()
        .layer(layer)
        .connect_http(rpc_url);
    Ok(provider)
}

// WASM is single-threaded
unsafe impl<P, N> Send for WalletProvider<P, N> {}
unsafe impl<P, N> Sync for WalletProvider<P, N> {}
