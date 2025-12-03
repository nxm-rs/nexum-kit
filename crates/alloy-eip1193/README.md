# alloy-eip1193

EIP-1193 provider and signer implementation for [Alloy](https://github.com/alloy-rs/alloy) in WebAssembly environments.

## Features

- **`Eip1193Transport`**: Tower Service implementation for JSON-RPC requests via browser wallets
- **`Eip1193Signer`**: Signer implementation that delegates signing to browser wallets
- **`Eip1193Requester`**: Generic typed request handler for EIP-1193 providers
- Type-safe API with compile-time guarantees
- Zero-copy serialization where possible
- Minimal allocations for optimal WASM performance

## Usage

### As a Transport (for read operations)

Use this when you need to make RPC calls through the browser wallet's provider:

```rust
use alloy_eip1193::Eip1193Transport;
use alloy::providers::ProviderBuilder;

let ethereum = Eip1193Transport::get_ethereum()?;
let transport = Eip1193Transport::new(ethereum);
let provider = ProviderBuilder::new().on_transport(transport);

// Now you can use the provider for read operations
let block_number = provider.get_block_number().await?;
```

### As a Signer (for write operations)

Use this when you need the browser wallet to sign transactions or messages:

```rust
use alloy_eip1193::Eip1193Signer;
use alloy::providers::ProviderBuilder;

let signer = Eip1193Signer::from_window().await?;
let provider = ProviderBuilder::new()
    .with_signer(signer)
    .on_http("https://eth.llamarpc.com".parse()?);

// Now you can sign transactions
let tx = TransactionRequest::default()
    .to(address)
    .value(U256::from(1000));
let receipt = provider.send_transaction(tx).await?;
```

### Combined (Transport + Signer)

For the best of both worlds - use the wallet's RPC for all operations:

```rust
use alloy_eip1193::{Eip1193Transport, Eip1193Signer};
use alloy::providers::ProviderBuilder;

let ethereum = Eip1193Transport::get_ethereum()?;
let transport = Eip1193Transport::new(ethereum.clone());

// Request accounts and create signer
let accounts = transport.request_accounts().await?;
let address = accounts[0].parse()?;
let signer = Eip1193Signer::new(ethereum, address);

// Create provider with both transport and signer
let provider = ProviderBuilder::new()
    .with_signer(signer)
    .on_transport(transport);
```

### Direct EIP-1193 Requests

For advanced use cases, you can use the `Eip1193Requester` directly:

```rust
use alloy_eip1193::{Eip1193Requester, Eip1193Transport};

let ethereum = Eip1193Transport::get_ethereum()?;
let requester = Eip1193Requester::new(ethereum);

// Make typed requests
let accounts: Vec<String> = requester
    .request("eth_requestAccounts", Vec::<()>::new())
    .await?;
```

### Wallet-Specific Operations

Use `WalletOperations` for wallet management tasks:

```rust
use alloy_eip1193::{WalletOperations, ChainConfig};
use alloy_chains::NamedChain;

let ethereum = Eip1193Transport::get_ethereum()?;
let wallet = WalletOperations::new(ethereum);

// Switch chains
wallet.switch_chain(137).await?; // Switch to Polygon

// Use the builder pattern for an ergonomic API
let config = ChainConfig::builder()
    .chain(NamedChain::Polygon)  // Auto-derives chain ID, name, and currency symbol
    .rpc_url("https://polygon-rpc.com")
    .rpc_url("https://polygon-backup.com")  // Multiple RPCs for redundancy
    .block_explorer("https://polygonscan.com")
    .build();
wallet.add_chain(config).await?;

// Or use a chain ID directly
let config = ChainConfig::builder()
    .chain(137u64)  // Also auto-derives metadata
    .rpc_url("https://polygon-rpc.com")
    .block_explorer("https://polygonscan.com")
    .build();
wallet.add_chain(config).await?;

// Override currency details if needed
let config = ChainConfig::builder()
    .chain(NamedChain::Gnosis)
    .rpc_url("https://rpc.gnosischain.com")
    .block_explorer("https://gnosisscan.io")
    .currency_name("xDAI Token")  // Custom name
    .currency_decimals(18)
    .build();
wallet.add_chain(config).await?;
```

## Architecture

This crate is designed to work seamlessly with Alloy's provider architecture:

- **Transport Layer** (`transport.rs`): `Eip1193Transport` implements Tower's `Service` trait for JSON-RPC requests
- **Signer Layer** (`signer.rs`): `Eip1193Signer` implements Alloy's `Signer`, `TxSigner`, and `NetworkWallet` traits
- **Request Layer** (`request.rs`): `Eip1193Requester` provides a low-level, type-safe API for making EIP-1193 requests
- **Wallet Operations** (`wallet.rs`): `WalletOperations` provides high-level methods for wallet management (switch chains, add chains)
- **Chain Configuration** (`chain.rs`): `ChainConfig` for configuring chains with auto-derived metadata from `alloy-chains`

All components are optimized for WebAssembly and work with any EIP-1193 compliant browser wallet (MetaMask, Coinbase Wallet, etc.).

## WASM-Only

This crate is designed exclusively for WebAssembly environments and requires:
- `wasm-bindgen`
- Browser with `window.ethereum` object

## License

MIT
