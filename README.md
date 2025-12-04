# Nexum-Kit 🔗

> **⚠️ WORK IN PROGRESS**: This repository is undergoing an active migration from TypeScript/React to Rust/Leptos. The Rust implementation is functional but not feature-complete. See [Migration Status](#migration-status) below.

**Type-safe Web3 wallet connections for Rust/Leptos**

Nexum-Kit is a Rust/WASM port of [RainbowKit](https://github.com/rainbow-me/rainbowkit), providing type-safe, developer-friendly wallet connection components for Leptos applications. Built on [Alloy](https://github.com/alloy-rs/alloy) and EIP-1193, Nexum-Kit brings Rust's type safety and performance to Web3 dApp development.

**nexum** (Latin): bond, connection, obligation

## Features

- 🦀 **Type-Safe**: Leverage Rust's type system for compile-time safety
- ⚡ **Performance**: WASM-compiled for near-native browser performance
- 🔌 **EIP-1193 Native**: First-class browser wallet integration
- 🎨 **Themeable**: Customizable UI components with multiple built-in themes
- 🌐 **Multi-Chain**: Support for Ethereum and EVM-compatible chains
- 📦 **Modular**: Use what you need - from full UI kit to low-level primitives

## Architecture

Nexum-Kit consists of two main crates:

### `nexum-kit` (AGPL-3.0)
Leptos component library providing:
- Wallet connection UI components
- Account management modals
- Chain switching interfaces
- Themed, customizable design system

### `alloy-eip1193` (MIT)
Low-level Alloy integration providing:
- EIP-1193 transport layer for browser wallets
- Type-safe error handling for wallet operations
- Provider and signer implementations
- Wallet discovery via EIP-6963

## Quick Start

### Installation

Add Nexum-Kit to your `Cargo.toml`:

```toml
[dependencies]
nexum-kit = { git = "https://github.com/nxm-rs/nexum-kit" }
alloy = { version = "1.1", features = ["provider-http", "signer-local"] }
leptos = "0.8"
```

### Basic Usage

```rust
use leptos::*;
use nexum_kit::prelude::*;
use nexum_kit::components::modals::ConnectModal;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConnectModal />
        <YourDappComponents />
    }
}
```

### Connect to a Wallet

```rust
use alloy_eip1193::prelude::*;
use alloy::providers::RootProvider;

// Create provider from browser wallet
let transport = Eip1193Transport::client_from_window()?;
let provider = RootProvider::new(transport);

// Request account access
let accounts = provider.request_accounts().await?;

// Send transactions
let tx = TransactionRequest::default()
    .to(address!("0x..."))
    .value(U256::from(1000000000000000000u64));

let pending = provider.send_transaction(tx).await?;
```

## Migration Status

### ✅ Completed
- [x] EIP-1193 transport layer and error handling
- [x] Basic wallet connection flow
- [x] Account display and management
- [x] EIP-6963 wallet discovery
- [x] Multi-chain support (chain switching)
- [x] Transaction sending with error handling
- [x] Theme system (Light, Dark, Midnight)

### 🚧 In Progress
- [ ] Complete component library parity with RainbowKit
- [ ] Wallet connection persistence
- [ ] Advanced chain configuration
- [ ] Sign-In with Ethereum (SIWE) integration
- [ ] Comprehensive documentation
- [ ] Example applications

### 📋 Planned
- [ ] Additional wallet connectors
- [ ] WalletConnect integration
- [ ] Mobile wallet support
- [ ] Testing suite
- [ ] Performance optimizations

## Examples

See the [`examples/basic/`](./examples/basic/) directory for a working example application.

To run the example:

```bash
cd examples/basic
trunk serve
```

Then open http://localhost:8080 in your browser with a Web3 wallet installed (MetaMask, Coinbase Wallet, etc.).

## Development

### Prerequisites

- Rust 1.75+
- wasm32-unknown-unknown target
- trunk for WASM development

### Building

```bash
# Install trunk
cargo install trunk

# Build all crates
cargo build --target wasm32-unknown-unknown

# Run example
cd examples/basic && trunk serve
```

## Dual Licensing

This repository contains code under multiple licenses:

- **`nexum-kit`** (Rust/Leptos components): **AGPL-3.0-or-later**
  - Ensures derivative works remain open source
  - See [`crates/nexum-kit/LICENSE-AGPL`](./crates/nexum-kit/LICENSE-AGPL)

- **`alloy-eip1193`** (Alloy integration): **MIT**
  - Enables broader adoption and upstream contributions
  - See [`crates/alloy-eip1193/LICENSE-MIT`](./crates/alloy-eip1193/LICENSE-MIT)

- **TypeScript/React packages**: **MIT** (Original RainbowKit)
  - In transition - not actively maintained

See [`LICENSE`](./LICENSE) for complete licensing details and [`NOTICE`](./NOTICE) for attribution.

## Attribution

Nexum-Kit is a derivative work based on [RainbowKit](https://github.com/rainbow-me/rainbowkit) by [Rainbow](https://rainbow.me). We are grateful to the Rainbow team for their excellent work on the original project. This Rust/Leptos port reimplements RainbowKit's functionality using Rust, WASM, and the Alloy library ecosystem.

## Contributing

Contributions are welcome! Please note the dual-licensing structure:

- Contributions to `nexum-kit/` fall under AGPL-3.0-or-later
- Contributions to `alloy-eip1193/` fall under MIT

See [CONTRIBUTING.md](./.github/CONTRIBUTING.md) for guidelines.

## Funding

Support development via Ethereum:

**0xC1FC64b34FA86D8fac48565E44882a32cC08EB97**

## Links

- **Original RainbowKit**: https://rainbowkit.com
- **Alloy**: https://github.com/alloy-rs/alloy
- **Leptos**: https://github.com/leptos-rs/leptos
- **Issues**: https://github.com/nxm-rs/nexum-kit/issues

## License

Dual-licensed - see [`LICENSE`](./LICENSE) for details.
