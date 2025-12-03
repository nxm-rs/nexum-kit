# alloy-eip1193

EIP-1193 provider and signer implementation for [Alloy](https://github.com/alloy-rs/alloy) in WebAssembly environments.

> **Note**: This crate is WASM-only. To build or test, use:
> ```bash
> cargo build --target wasm32-unknown-unknown
> ```

## Features

- **`Eip1193Transport`**: Tower Service implementation for JSON-RPC requests via browser wallets
- **`WalletLayer`**: Provider layer for smart request routing
- **`ext::Eip1193`**: Trait extension for EIP-1193 mandated wallet operations (automatically available on any provider)
- **`Eip1193Signer`**: Signer implementation (⚠️ uses `eth_sign`, shows warnings)
- **`ChainConfig`**: Type-safe chain configuration with builder pattern
- Type-safe API with compile-time guarantees
- Zero-copy serialization where possible
- Minimal allocations for optimal WASM performance

## Architecture

This crate provides **three usage patterns** with different trade-offs:

### Pattern 1: Smart Routing with WalletLayer (Recommended)

Use `WalletLayer` to add wallet operations to any provider. The layer provides access to the wallet transport while keeping RPC reads on your configured transport (HTTP/WebSocket).

```rust
use alloy::providers::ProviderBuilder;
use alloy_eip1193::{WalletLayer, ext::Eip1193};

// Create wallet layer from window.ethereum
let wallet_layer = WalletLayer::from_window()?;

// Add to any provider (RPC reads go to HTTP, wallet ops to browser wallet)
let provider = ProviderBuilder::new()
    .layer(wallet_layer)
    .on_http("https://eth.llamarpc.com".parse()?);

// Eip1193 trait methods automatically available!
let accounts = provider.request_accounts().await?;
provider.switch_chain(137).await?; // Switch to Polygon

// Standard provider methods work normally
let block = provider.get_block_number().await?;
```

**Benefits:**
- ✅ Clean separation: wallet operations via EIP-1193, reads via HTTP/WS
- ✅ `ext::Eip1193` trait extension provides ergonomic API
- ✅ No blind signing warnings
- ✅ Follows Alloy's layer pattern

### Pattern 2: Eip1193 Extension with Any Provider

The `ext::Eip1193` trait is automatically available on **any** Alloy provider. Wallet methods use `client().request()` internally, routing through whatever transport you configured.

```rust
use alloy::providers::ProviderBuilder;
use alloy_eip1193::ext::Eip1193;

// Create provider with ANY transport
let provider = ProviderBuilder::new()
    .on_http("https://eth.llamarpc.com".parse()?);

// EIP-1193 methods automatically available!
// (These will call through the HTTP transport)
let accounts = provider.request_accounts().await?;
provider.switch_chain(137).await?;
```

**Benefits:**
- ✅ Minimal setup - no explicit wallet configuration
- ✅ Works with any transport
- ⚠️ All requests (including wallet ops) go through configured transport

### Pattern 3: Eip1193Signer (⚠️ Shows Warnings)

Use `Eip1193Signer` when you need full `NetworkWallet` compatibility. Note that this uses `eth_sign` internally, which shows scary warnings in MetaMask.

```rust
use alloy::providers::ProviderBuilder;
use alloy_eip1193::Eip1193Signer;

let signer = Eip1193Signer::from_window().await?;

// ⚠️ WARNING: Uses eth_sign internally
// MetaMask will show scary warnings to users
let provider = ProviderBuilder::new()
    .wallet(signer)
    .on_http("https://eth.llamarpc.com".parse()?);
```

**When to use:**
- Only use if you specifically need `NetworkWallet` trait
- For better UX, prefer Pattern 1 (WalletLayer)

**Caveats:**
- ⚠️ Uses `eth_sign` which MetaMask is deprecating
- ⚠️ Shows "dangerous" warning to users
- ⚠️ Blind signing security concerns

## Wallet-Specific Operations

Use `ext::Eip1193` trait extension for wallet management:

```rust
use alloy_eip1193::{ext::Eip1193, ChainConfig};
use alloy_chains::NamedChain;

// Switch chains
provider.switch_chain(137).await?; // Polygon

// Add a new chain with builder pattern
let config = ChainConfig::builder()
    .chain(NamedChain::Polygon)  // Auto-derives metadata
    .rpc_url("https://polygon-rpc.com")
    .block_explorer("https://polygonscan.com")
    .build();
provider.add_chain(config).await?;

// Watch an asset
provider.watch_asset(token_address, "USDC", 6).await?;
```


## Chain Management

The `Eip1193Signer` includes chain ID tracking and validation:

```rust
let mut signer = Eip1193Signer::from_window().await?;

// Refresh chain ID from wallet
let chain_id = signer.refresh_chain_id().await?;

// Validate expected chain
signer.validate_chain_id(1)?; // Mainnet

// Get current chain
if let Some(id) = signer.chain_id() {
    println!("Current chain: {}", id);
}
```

## Caveats

### Signer Uses eth_sign

The `Eip1193Signer` implementation uses `eth_sign` for transaction signing, which:
- Shows warnings in MetaMask ("This is a dangerous operation")
- Is being deprecated by wallet providers
- Involves blind signing (security risk)

**Recommendation:** Use `WalletLayer` (Pattern 1) for production applications.

### WASM-Only

This crate is designed exclusively for WebAssembly environments and requires:
- `wasm-bindgen`
- Browser with `window.ethereum` object

## License

MIT
