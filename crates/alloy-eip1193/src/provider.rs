//! Provider layer for smart EIP-1193 routing
//!
//! Provides `WalletLayer` that can be added to any provider to route wallet operations
//! through EIP-1193 while keeping RPC reads on the original transport.

use alloy::providers::{Provider, ProviderBuilder, ProviderLayer, RootProvider};
use alloy::network::{Network, Ethereum};
use alloy::transports::{Transport, TransportResult};
use alloy_json_rpc::{RequestPacket, ResponsePacket};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use std::marker::PhantomData;
use tower::{Service, Layer};
use wasm_bindgen::JsValue;
use crate::Eip1193Transport;

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
    pub fn from_window() -> Result<Self, JsValue> {
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

impl<P, N> Provider<N> for WalletProvider<P, N>
where
    P: Provider<N>,
    N: Network,
{
    fn root(&self) -> &RootProvider<N> {
        self.inner.root()
    }

    // Override methods that should use wallet
    // The WalletApi trait already provides these via extension,
    // so we don't need to override anything here!
    // The layer just provides access to wallet_transport if needed
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
pub fn with_wallet(ethereum: JsValue, rpc_url: url::Url) -> Result<impl Provider, JsValue> {
    let layer = WalletLayer::new(ethereum);
    let provider = ProviderBuilder::new()
        .layer(layer)
        .connect_http(rpc_url);
    Ok(provider)
}

// WASM is single-threaded
unsafe impl<P, N> Send for WalletProvider<P, N> {}
unsafe impl<P, N> Sync for WalletProvider<P, N> {}
