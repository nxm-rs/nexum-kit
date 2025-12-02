# RainbowKit → Leptos-RainbowKit Migration Plan

**Version:** 1.0
**Date:** 2025-12-02
**Status:** Planning Phase

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Technology Stack Transition](#technology-stack-transition)
3. [Architecture Analysis](#architecture-analysis)
4. [Proposed Rust Crate Structure](#proposed-rust-crate-structure)
5. [Migration Strategy](#migration-strategy)
6. [Development Environment Setup](#development-environment-setup)
7. [Phase 1: Foundation](#phase-1-foundation)
8. [Phase 2: Core Components](#phase-2-core-components)
9. [Phase 3: Wallet Integration](#phase-3-wallet-integration)
10. [Phase 4: Advanced Features](#phase-4-advanced-features)
11. [Phase 5: Polish & Distribution](#phase-5-polish--distribution)
12. [Key Technical Challenges](#key-technical-challenges)
13. [Testing Strategy](#testing-strategy)
14. [Success Criteria](#success-criteria)
15. [Risk Assessment](#risk-assessment)

---

## Executive Summary

This document outlines a comprehensive plan to migrate **RainbowKit** (a TypeScript/React wallet connection library) to **Leptos-RainbowKit** (a Rust/Leptos component library). The migration will:

- **Replace TypeScript with Rust** for type safety and performance
- **Replace React with Leptos** for fine-grained reactivity and CSR efficiency
- **Replace wagmi/viem with Alloy** for blockchain primitives and RPC interactions
- **Replace Vanilla Extract with Tailwind CSS** for styling
- **Maintain API parity** with the original RainbowKit where possible
- **Create a production-ready Leptos component library** for web3 wallet connections

### Key Goals

1. Build a client-side rendered (CSR) Leptos component library
2. Support all major Ethereum wallets (MetaMask, WalletConnect, Coinbase, etc.)
3. Provide theming system (light/dark/custom themes)
4. Support internationalization (i18n)
5. Transaction tracking and management
6. Responsive design (mobile & desktop)
7. Type-safe blockchain interactions via Alloy

---

## Technology Stack Transition

### Before (RainbowKit)

| Layer | Technology |
|-------|-----------|
| **Language** | TypeScript |
| **UI Framework** | React 18+ |
| **Build Tool** | esbuild (custom build.js) |
| **Styling** | Vanilla Extract (CSS-in-TS) |
| **Blockchain** | wagmi v2 + viem 2.x |
| **State Management** | React Context + @tanstack/react-query |
| **Routing** | N/A (component library) |
| **QR Codes** | cuer (React components) |
| **Package Manager** | pnpm |
| **Monorepo** | pnpm workspaces |

### After (Leptos-RainbowKit)

| Layer | Technology |
|-------|-----------|
| **Language** | Rust (MSRV 1.88+) |
| **UI Framework** | Leptos 0.8+ (CSR mode) |
| **Build Tool** | Trunk + cargo |
| **Styling** | Tailwind CSS v4 |
| **Blockchain** | Alloy v1.0+ (WASM-compatible) |
| **State Management** | Leptos Signals + Resources |
| **Routing** | leptos_router (optional for examples) |
| **QR Codes** | qrcode crate |
| **Package Manager** | cargo |
| **Monorepo** | cargo workspace |

---

## Architecture Analysis

### Current RainbowKit Architecture

```
RainbowKit (TypeScript/React)
│
├─ Core Components
│  ├─ ConnectButton (main entry point)
│  ├─ ConnectModal (wallet selection)
│  ├─ AccountModal (account management)
│  └─ ChainModal (network switching)
│
├─ Wallet System
│  ├─ 73+ wallet connectors (MetaMask, WC, Coinbase, etc.)
│  ├─ EIP-6963 auto-discovery
│  ├─ Adaptive connector strategies (injected/QR/deep-link)
│  └─ Recent wallets (localStorage)
│
├─ Theming
│  ├─ Theme contract (CSS variables)
│  ├─ 3 built-in themes (light/dark/midnight)
│  ├─ Customizable (accent color, border radius, font, blur)
│  └─ Vanilla Extract sprinkles (atomic CSS)
│
├─ State Management
│  ├─ 11+ nested React contexts
│  ├─ Transaction store (observable pattern)
│  ├─ Authentication adapter (SIWE)
│  └─ wagmi integration (hooks)
│
└─ Features
   ├─ i18n (22 languages)
   ├─ Responsive breakpoints (768px)
   ├─ Transaction tracking
   ├─ ENS name resolution
   ├─ Avatar support (custom/ENS)
   └─ Cool mode (confetti)
```

### Target Leptos Architecture

```
Leptos-RainbowKit (Rust/Leptos)
│
├─ Core Components (Leptos CSR)
│  ├─ ConnectButton (main entry point)
│  ├─ ConnectModal (wallet selection)
│  ├─ AccountModal (account management)
│  └─ ChainModal (network switching)
│
├─ Wallet System
│  ├─ Wallet registry (declarative wallet metadata)
│  ├─ EIP-6963 via window.ethereum (wasm-bindgen)
│  ├─ WalletConnect SDK (WASM-compatible client)
│  ├─ Connector abstraction (trait-based)
│  └─ Recent wallets (localStorage via web_sys)
│
├─ Theming (Tailwind-based)
│  ├─ CSS variables for theme tokens
│  ├─ 3 built-in themes (Rust config → CSS)
│  ├─ Tailwind utility classes
│  └─ Dark mode support (class-based)
│
├─ State Management (Leptos Signals)
│  ├─ Modal state (RwSignal<ModalState>)
│  ├─ Wallet connection (Resource<Connection>)
│  ├─ Transaction store (StoredValue + LocalStorage)
│  ├─ Auth adapter (trait-based)
│  └─ Alloy provider integration
│
└─ Features
   ├─ i18n (macro-based translations)
   ├─ Responsive (Tailwind breakpoints)
   ├─ Transaction tracking (Alloy subscriptions)
   ├─ ENS resolution (Alloy providers)
   ├─ Avatar support
   └─ Cool mode (canvas API)
```

---

## Proposed Rust Crate Structure

We'll use a **cargo workspace** to mirror the original monorepo structure:

```
rainbowkit/
├─ Cargo.toml                      # Workspace root
├─ flake.nix                       # Nix development environment
├─ Trunk.toml                      # Trunk build configuration
├─ tailwind.config.js              # Tailwind CSS configuration
├─ .gitignore
│
├─ crates/
│  │
│  ├─ leptos-rainbowkit/           # Main library crate
│  │  ├─ Cargo.toml                # Core dependencies (leptos, alloy, etc.)
│  │  ├─ src/
│  │  │  ├─ lib.rs                 # Public API exports
│  │  │  ├─ prelude.rs             # Common imports
│  │  │  │
│  │  │  ├─ components/            # UI components
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ connect_button.rs   # Main connect button
│  │  │  │  ├─ wallet_button.rs    # Single wallet button
│  │  │  │  ├─ modals/
│  │  │  │  │  ├─ mod.rs
│  │  │  │  │  ├─ connect.rs       # Connect modal
│  │  │  │  │  ├─ account.rs       # Account modal
│  │  │  │  │  └─ chain.rs         # Chain switcher modal
│  │  │  │  ├─ primitives/
│  │  │  │  │  ├─ box.rs           # Base box component
│  │  │  │  │  ├─ text.rs          # Typography
│  │  │  │  │  ├─ dialog.rs        # Modal primitive
│  │  │  │  │  └─ qr_code.rs       # QR code generator
│  │  │  │  └─ provider.rs         # RainbowKitProvider
│  │  │  │
│  │  │  ├─ wallets/               # Wallet connectors
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ wallet.rs           # Wallet trait & types
│  │  │  │  ├─ connector.rs        # Connector abstraction
│  │  │  │  ├─ registry.rs         # Wallet registry
│  │  │  │  ├─ eip6963.rs          # EIP-6963 discovery
│  │  │  │  └─ connectors/
│  │  │  │     ├─ mod.rs
│  │  │  │     ├─ metamask.rs
│  │  │  │     ├─ walletconnect.rs
│  │  │  │     ├─ coinbase.rs
│  │  │  │     ├─ rainbow.rs
│  │  │  │     └─ injected.rs      # Generic injected wallet
│  │  │  │
│  │  │  ├─ provider/              # Alloy integration
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ client.rs           # Alloy HTTP provider (WASM)
│  │  │  │  ├─ signer.rs           # Browser wallet signer
│  │  │  │  ├─ chains.rs           # Chain configurations
│  │  │  │  └─ network.rs          # Network state
│  │  │  │
│  │  │  ├─ state/                 # State management
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ connection.rs       # Connection state
│  │  │  │  ├─ modal.rs            # Modal state
│  │  │  │  ├─ transaction.rs      # Transaction store
│  │  │  │  └─ auth.rs             # Authentication adapter
│  │  │  │
│  │  │  ├─ theme/                 # Theming system
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ types.rs            # Theme types
│  │  │  │  ├─ light.rs            # Light theme
│  │  │  │  ├─ dark.rs             # Dark theme
│  │  │  │  ├─ midnight.rs         # Midnight theme
│  │  │  │  └─ provider.rs         # Theme context
│  │  │  │
│  │  │  ├─ i18n/                  # Internationalization
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ macros.rs           # i18n macro
│  │  │  │  └─ locales/
│  │  │  │     ├─ en_us.rs
│  │  │  │     ├─ es_es.rs
│  │  │  │     └─ ...
│  │  │  │
│  │  │  ├─ hooks/                 # Custom hooks (signals)
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ use_wallet.rs       # Wallet connection hook
│  │  │  │  ├─ use_balance.rs      # Balance fetching
│  │  │  │  ├─ use_ens.rs          # ENS resolution
│  │  │  │  └─ use_transaction.rs  # Transaction tracking
│  │  │  │
│  │  │  └─ utils/                 # Utilities
│  │  │     ├─ mod.rs
│  │  │     ├─ storage.rs          # LocalStorage helpers
│  │  │     ├─ format.rs           # Address/balance formatting
│  │  │     └─ responsive.rs       # Responsive utilities
│  │  │
│  │  └─ styles/                   # Tailwind styles
│  │     └─ tailwind.css           # Base + components + utilities
│  │
│  ├─ leptos-rainbow-button/       # Standalone rainbow button
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     └─ lib.rs
│  │
│  └─ create-leptos-rainbowkit/    # CLI scaffolding tool
│     ├─ Cargo.toml
│     ├─ src/
│     │  └─ main.rs
│     └─ templates/
│        └─ trunk-app/             # CSR template
│
├─ examples/                       # Example applications
│  ├─ basic/                       # Basic CSR example
│  │  ├─ Cargo.toml
│  │  ├─ Trunk.toml
│  │  ├─ index.html
│  │  └─ src/
│  │     ├─ main.rs
│  │     └─ lib.rs
│  │
│  └─ advanced/                    # Advanced features demo
│     └─ ...
│
├─ docs/                           # Documentation site (optional)
│  └─ ...
│
└─ tests/                          # Integration tests
   └─ ...
```

---

## Migration Strategy

### Phased Approach

We'll migrate in **5 distinct phases**, each building upon the previous:

1. **Foundation** - Project setup, build system, dev environment
2. **Core Components** - Basic UI components without wallet logic
3. **Wallet Integration** - Alloy + wallet connectors + state management
4. **Advanced Features** - Transactions, ENS, i18n, theming
5. **Polish & Distribution** - Testing, docs, examples, crates.io release

### Principles

- **In-place migration**: Use `git mv` and `git rm` to replace files directly
- **No history preservation**: Git history is already preserved in the existing repo
- **Incremental validation**: Test each phase thoroughly before proceeding
- **Feature parity**: Match original RainbowKit functionality where feasible
- **Rust idioms**: Embrace Rust patterns (traits, enums, Result types)

---

## Development Environment Setup

### Prerequisites

- Rust 1.88+ (nightly or stable)
- Node.js 22+ (for Tailwind CLI)
- Nix (optional, for reproducible environment)

### flake.nix Configuration

Based on `/code/nullisLabs/shorturl/flake.nix`, create:

```nix
{
  description = "Leptos-RainbowKit - Web3 wallet connection library for Leptos";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            # System dependencies
            openssl
            pkg-config

            # Rust toolchain with wasm32 target
            ((rust-bin.selectLatestNightlyWith
              (toolchain: toolchain.default)).override {
                extensions = [ "rust-analyzer" "rust-src" ];
                targets = [ "wasm32-unknown-unknown" ];
              })

            # Development tools
            cargo-watch        # Auto-rebuild
            cargo-edit         # cargo add/rm/upgrade
            trunk              # WASM bundler
            wasm-bindgen-cli   # WASM bindings
            just               # Task runner

            # CSS tooling
            tailwindcss_4      # Tailwind CSS v4
          ];

          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)}/lib/rustlib/src/rust/library";

          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"

            # Configure editor
            mkdir -p .zed
            ln -sf $(command -v rust-analyzer) .zed/rust-analyzer

            # Install additional tools
            command -v leptosfmt &> /dev/null || cargo install leptosfmt
            command -v cargo-leptos &> /dev/null || cargo install cargo-leptos

            echo "🦀 Leptos-RainbowKit development environment loaded"
            echo ""
            echo "📦 Rust:     $(rustc --version)"
            echo "🎨 Tailwind: $(tailwindcss --version)"
            echo "📦 Trunk:    $(trunk --version)"
            echo ""
            echo "💡 Quick start:"
            echo "   - List tasks:      just --list"
            echo "   - Dev server:      just dev"
            echo "   - Run example:     just example"
            echo "   - Build library:   just build"
            echo "   - Test:            just test"
          '';
        };
      }
    );
}
```

### Justfile (Task Runner)

Create a `Justfile` for common tasks:

```justfile
# List all available commands
default:
  @just --list

# Run development server (example app)
dev:
  cd examples/basic && trunk serve --open

# Build the library
build:
  cargo build --release --target wasm32-unknown-unknown

# Run tests
test:
  cargo test --all-features

# Format code
fmt:
  cargo fmt --all
  leptosfmt **/*.rs

# Check code (clippy + fmt)
check:
  cargo clippy --all-features -- -D warnings
  cargo fmt --all -- --check

# Clean build artifacts
clean:
  cargo clean
  rm -rf examples/*/dist

# Update dependencies
update:
  cargo update

# Generate documentation
docs:
  cargo doc --all-features --no-deps --open

# Run basic example
example NAME="basic":
  cd examples/{{NAME}} && trunk serve --open
```

---

## Phase 1: Foundation

**Goal:** Set up the Rust workspace, build system, and basic project structure.

### Tasks

#### 1.1 Initialize Workspace

```bash
# Create workspace Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
members = [
  "crates/leptos-rainbowkit",
  "crates/leptos-rainbow-button",
  "crates/create-leptos-rainbowkit",
  "examples/basic",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["RainbowKit Contributors"]
license = "MIT"
repository = "https://github.com/rainbow-me/rainbowkit"

[workspace.dependencies]
leptos = { version = "0.8", features = ["csr"] }
leptos_meta = "0.8"
leptos_router = "0.8"
alloy = { version = "1.0", features = ["full"], default-features = false }
alloy-core = { version = "0.8", default-features = false }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "0.2"
web-sys = "0.3"
js-sys = "0.3"
console_log = "1"
log = "0.4"
console_error_panic_hook = "0.1"
qrcode = "0.14"
getrandom = { version = "0.3", features = ["js"] }

[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"
EOF
```

#### 1.2 Remove TypeScript Infrastructure

```bash
# Remove TypeScript build artifacts
git rm -rf packages/rainbowkit/build.js
git rm -rf packages/rainbowkit/tsconfig.json
git rm -rf packages/rainbowkit/dist
git rm -rf packages/rainbowkit/node_modules

# Remove pnpm workspace files
git rm pnpm-lock.yaml
git rm pnpm-workspace.yaml

# Remove package.json files (keep root for scripts if needed)
git rm packages/*/package.json
```

#### 1.3 Create Core Library Crate

```bash
# Create directory structure
mkdir -p crates/leptos-rainbowkit/src/{components,wallets,provider,state,theme,i18n,hooks,utils}
mkdir -p crates/leptos-rainbowkit/styles

# Initialize Cargo.toml
cat > crates/leptos-rainbowkit/Cargo.toml << 'EOF'
[package]
name = "leptos-rainbowkit"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Leptos component library for connecting Ethereum wallets"
keywords = ["leptos", "ethereum", "web3", "wallet", "blockchain"]
categories = ["wasm", "web-programming", "gui"]

[dependencies]
leptos.workspace = true
leptos_meta.workspace = true
leptos_router.workspace = true
alloy = { workspace = true, features = ["provider-http", "signer-wallet"] }
alloy-core.workspace = true
serde.workspace = true
serde_json.workspace = true
wasm-bindgen.workspace = true
web-sys = { workspace = true, features = [
  "Window",
  "Document",
  "Storage",
  "EventTarget",
  "MessageEvent",
  "CustomEvent",
  "Crypto",
] }
js-sys.workspace = true
console_log.workspace = true
log.workspace = true
console_error_panic_hook.workspace = true
qrcode.workspace = true
getrandom.workspace = true

[features]
default = []
ssr = []

[lib]
crate-type = ["cdylib", "rlib"]
EOF
```

#### 1.4 Setup Tailwind CSS

```bash
# Create Tailwind config
cat > tailwind.config.js << 'EOF'
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./crates/*/src/**/*.rs",
    "./examples/*/src/**/*.rs",
    "./examples/*/index.html",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        'rk-accent': 'var(--rk-colors-accent)',
        'rk-accent-hover': 'var(--rk-colors-accent-hover)',
        'rk-bg': 'var(--rk-colors-bg)',
        'rk-bg-secondary': 'var(--rk-colors-bg-secondary)',
        'rk-text': 'var(--rk-colors-text)',
        'rk-text-secondary': 'var(--rk-colors-text-secondary)',
      },
      borderRadius: {
        'rk': 'var(--rk-radii-base)',
      },
      backdropBlur: {
        'rk': 'var(--rk-blur-overlay)',
      },
    },
  },
  plugins: [],
}
EOF

# Create base styles
cat > crates/leptos-rainbowkit/styles/tailwind.css << 'EOF'
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    /* Color tokens - Light theme */
    --rk-colors-accent: 0 95% 63%;
    --rk-colors-accent-hover: 0 95% 58%;
    --rk-colors-bg: 0 0% 100%;
    --rk-colors-bg-secondary: 0 0% 98%;
    --rk-colors-text: 0 0% 7%;
    --rk-colors-text-secondary: 0 0% 38%;
    --rk-colors-border: 0 0% 87%;

    /* Structural tokens */
    --rk-radii-base: 12px;
    --rk-blur-overlay: 0px;
    --rk-font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }

  .dark {
    /* Color tokens - Dark theme */
    --rk-colors-accent: 0 95% 63%;
    --rk-colors-accent-hover: 0 95% 58%;
    --rk-colors-bg: 0 0% 7%;
    --rk-colors-bg-secondary: 0 0% 11%;
    --rk-colors-text: 0 0% 98%;
    --rk-colors-text-secondary: 0 0% 67%;
    --rk-colors-border: 0 0% 20%;
  }

  [data-rk] {
    font-family: var(--rk-font-family);
  }
}

@layer components {
  .rk-modal-overlay {
    @apply fixed inset-0 bg-black/50 backdrop-blur-rk z-50;
  }

  .rk-modal-content {
    @apply bg-rk-bg rounded-rk border border-rk-border shadow-xl;
  }

  .rk-button {
    @apply px-4 py-2 rounded-rk font-medium transition-colors;
  }

  .rk-button-primary {
    @apply rk-button bg-rk-accent text-white hover:bg-rk-accent-hover;
  }

  .rk-button-secondary {
    @apply rk-button bg-rk-bg-secondary text-rk-text hover:bg-rk-border;
  }
}
EOF
```

#### 1.5 Create Basic Example App

```bash
# Create example directory
mkdir -p examples/basic/src
mkdir -p examples/basic/public

# Create Cargo.toml
cat > examples/basic/Cargo.toml << 'EOF'
[package]
name = "leptos-rainbowkit-example"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos.workspace = true
leptos_meta.workspace = true
leptos_router.workspace = true
leptos-rainbowkit = { path = "../../crates/leptos-rainbowkit" }
console_log.workspace = true
log.workspace = true
console_error_panic_hook.workspace = true

[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"
EOF

# Create Trunk.toml
cat > examples/basic/Trunk.toml << 'EOF'
[build]
target = "index.html"
release = false
dist = "dist"
public_url = "/"

[serve]
addresses = ["127.0.0.1"]
port = 3000
open = true

[watch]
watch = ["../../crates/leptos-rainbowkit/src"]
ignore = []
EOF

# Create index.html
cat > examples/basic/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Leptos RainbowKit Example</title>
  <link data-trunk rel="rust" data-wasm-opt="z" data-weak-refs />
  <link data-trunk rel="tailwind-css" href="../../crates/leptos-rainbowkit/styles/tailwind.css" />
</head>
<body>
  <div id="app"></div>
</body>
</html>
EOF

# Create main.rs
cat > examples/basic/src/main.rs << 'EOF'
use leptos::prelude::*;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100 dark:bg-gray-900 flex items-center justify-center">
            <div class="text-center">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
                    "Leptos RainbowKit"
                </h1>
                <p class="text-gray-600 dark:text-gray-400">
                    "Migration in progress..."
                </p>
            </div>
        </div>
    }
}
EOF
```

#### 1.6 Create Development Scripts

```bash
# Add flake.nix (from earlier section)
# Add Justfile (from earlier section)

# Create .gitignore
cat >> .gitignore << 'EOF'

# Rust
target/
Cargo.lock
**/*.rs.bk
*.pdb

# Trunk
dist/
.trunk/

# IDE
.zed/
.vscode/
.idea/

# Nix
result
result-*

# Tailwind
node_modules/
EOF
```

#### 1.7 Validation

```bash
# Enter nix shell (if using Nix)
nix develop

# Build workspace
cargo build

# Run example
cd examples/basic && trunk serve

# Verify browser opens to localhost:3000 with "Migration in progress..."
```

---

## Phase 2: Core Components

**Goal:** Build basic UI components (primitives, buttons, modals) without wallet logic.

### Tasks

#### 2.1 Create lib.rs Entry Point

**File:** `crates/leptos-rainbowkit/src/lib.rs`

```rust
//! Leptos-RainbowKit - Web3 wallet connection library for Leptos
//!
//! This library provides a set of components for connecting Ethereum wallets
//! in Leptos applications with a focus on CSR (client-side rendering).

// Module declarations
pub mod components;
pub mod provider;
pub mod state;
pub mod theme;
pub mod wallets;
pub mod hooks;
pub mod utils;
pub mod i18n;
pub mod prelude;

// Re-exports
pub use components::{
    ConnectButton,
    RainbowKitProvider,
};

pub use theme::{Theme, LightTheme, DarkTheme, MidnightTheme};
```

#### 2.2 Create Prelude Module

**File:** `crates/leptos-rainbowkit/src/prelude.rs`

```rust
//! Common imports for Leptos-RainbowKit applications

pub use crate::components::{ConnectButton, RainbowKitProvider};
pub use crate::theme::{Theme, LightTheme, DarkTheme};
pub use crate::hooks::{use_wallet, use_balance, use_ens};
pub use leptos::prelude::*;
```

#### 2.3 Implement Theme System

**File:** `crates/leptos-rainbowkit/src/theme/mod.rs`

```rust
pub mod types;
pub mod light;
pub mod dark;
pub mod midnight;
pub mod provider;

pub use types::{Theme, ThemeConfig, AccentColor, BorderRadius, FontStack, OverlayBlur};
pub use light::LightTheme;
pub use dark::DarkTheme;
pub use midnight::MidnightTheme;
pub use provider::{ThemeProvider, use_theme};
```

**File:** `crates/leptos-rainbowkit/src/theme/types.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccentColor {
    Blue,
    Green,
    Orange,
    Pink,
    Purple,
    Red,
    Custom { h: u16, s: u8, l: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderRadius {
    None,
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStack {
    System,
    Rounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayBlur {
    None,
    Small,
    Large,
}

#[derive(Debug, Clone)]
pub struct ThemeConfig {
    pub accent_color: AccentColor,
    pub border_radius: BorderRadius,
    pub font_stack: FontStack,
    pub overlay_blur: OverlayBlur,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            accent_color: AccentColor::Blue,
            border_radius: BorderRadius::Large,
            font_stack: FontStack::Rounded,
            overlay_blur: OverlayBlur::None,
        }
    }
}

pub trait Theme {
    fn name(&self) -> &'static str;
    fn to_css_vars(&self, config: &ThemeConfig) -> String;
}
```

#### 2.4 Implement Primitive Components

**File:** `crates/leptos-rainbowkit/src/components/primitives/box.rs`

```rust
use leptos::prelude::*;

#[component]
pub fn Box(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class data-rk="">
            {children()}
        </div>
    }
}
```

**File:** `crates/leptos-rainbowkit/src/components/primitives/dialog.rs`

```rust
use leptos::prelude::*;
use leptos::portal::Portal;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

#[component]
pub fn Dialog(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    children: Children,
) -> impl IntoView {
    // Handle ESC key
    let handle_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" {
            on_close.call(());
        }
    };

    // Handle backdrop click
    let handle_backdrop_click = move |ev: Event| {
        if let Some(target) = ev.target() {
            if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                if element.class_list().contains("rk-modal-overlay") {
                    on_close.call(());
                }
            }
        }
    };

    view! {
        <Show when=move || open.get()>
            <Portal>
                <div
                    class="rk-modal-overlay"
                    on:click=handle_backdrop_click
                    on:keydown=handle_keydown
                >
                    <div class="flex items-center justify-center min-h-screen p-4">
                        <div class="rk-modal-content p-6 max-w-md w-full">
                            {children()}
                        </div>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}
```

#### 2.5 Implement Modal State Management

**File:** `crates/leptos-rainbowkit/src/state/modal.rs`

```rust
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalType {
    None,
    Connect,
    Account,
    Chain,
}

#[derive(Debug, Clone, Copy)]
pub struct ModalState {
    current: RwSignal<ModalType>,
}

impl ModalState {
    pub fn new() -> Self {
        Self {
            current: RwSignal::new(ModalType::None),
        }
    }

    pub fn open_connect(&self) {
        self.current.set(ModalType::Connect);
    }

    pub fn open_account(&self) {
        self.current.set(ModalType::Account);
    }

    pub fn open_chain(&self) {
        self.current.set(ModalType::Chain);
    }

    pub fn close(&self) {
        self.current.set(ModalType::None);
    }

    pub fn is_open(&self, modal_type: ModalType) -> Signal<bool> {
        Signal::derive(move || self.current.get() == modal_type)
    }
}

// Context provider
pub fn provide_modal_state() -> ModalState {
    let state = ModalState::new();
    provide_context(state);
    state
}

pub fn use_modal_state() -> ModalState {
    expect_context::<ModalState>()
}
```

#### 2.6 Implement RainbowKitProvider

**File:** `crates/leptos-rainbowkit/src/components/provider.rs`

```rust
use leptos::prelude::*;
use crate::theme::{Theme, ThemeConfig, ThemeProvider};
use crate::state::modal::provide_modal_state;

#[component]
pub fn RainbowKitProvider<T: Theme + 'static>(
    #[prop(optional)] theme: Option<T>,
    #[prop(optional)] theme_config: Option<ThemeConfig>,
    children: Children,
) -> impl IntoView {
    // Provide modal state
    provide_modal_state();

    // Provide theme
    let config = theme_config.unwrap_or_default();

    view! {
        <ThemeProvider theme=theme config=config>
            {children()}
        </ThemeProvider>
    }
}
```

#### 2.7 Implement Basic ConnectButton

**File:** `crates/leptos-rainbowkit/src/components/connect_button.rs`

```rust
use leptos::prelude::*;
use crate::state::modal::use_modal_state;

#[component]
pub fn ConnectButton() -> impl IntoView {
    let modal_state = use_modal_state();

    let handle_click = move |_| {
        modal_state.open_connect();
    };

    view! {
        <button
            class="rk-button-primary"
            on:click=handle_click
        >
            "Connect Wallet"
        </button>
    }
}
```

#### 2.8 Implement Connect Modal (Skeleton)

**File:** `crates/leptos-rainbowkit/src/components/modals/connect.rs`

```rust
use leptos::prelude::*;
use crate::components::primitives::dialog::Dialog;
use crate::state::modal::{use_modal_state, ModalType};

#[component]
pub fn ConnectModal() -> impl IntoView {
    let modal_state = use_modal_state();
    let is_open = modal_state.is_open(ModalType::Connect);
    let on_close = Callback::new(move |_| modal_state.close());

    view! {
        <Dialog open=is_open on_close=on_close>
            <h2 class="text-2xl font-bold mb-4 text-rk-text">
                "Connect Wallet"
            </h2>
            <p class="text-rk-text-secondary">
                "Wallet selection will be implemented in Phase 3"
            </p>
        </Dialog>
    }
}
```

#### 2.9 Update Example App

**File:** `examples/basic/src/main.rs`

```rust
use leptos::prelude::*;
use leptos_rainbowkit::prelude::*;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <RainbowKitProvider>
            <div class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col items-center justify-center gap-4">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-white">
                    "Leptos RainbowKit"
                </h1>
                <ConnectButton />
                <leptos_rainbowkit::components::modals::ConnectModal />
            </div>
        </RainbowKitProvider>
    }
}
```

#### 2.10 Validation

```bash
# Build and run
just dev

# Test:
# 1. Click "Connect Wallet" button
# 2. Modal should open with title
# 3. ESC key should close modal
# 4. Clicking backdrop should close modal
```

---

## Phase 3: Wallet Integration

**Goal:** Integrate Alloy for blockchain interactions and implement wallet connectors.

### Tasks

#### 3.1 Setup Alloy Provider

**File:** `crates/leptos-rainbowkit/src/provider/client.rs`

```rust
use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::Address;
use alloy::transports::http::{Client, Http};
use wasm_bindgen::JsValue;

pub type HttpProvider = Provider<Http<Client>>;

pub async fn create_http_provider(rpc_url: &str) -> Result<HttpProvider, JsValue> {
    ProviderBuilder::new()
        .on_http(rpc_url.parse().map_err(|e| JsValue::from_str(&format!("{:?}", e)))?)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}
```

**Note:** Alloy's WASM support is limited for some features. We may need to use `wasm-bindgen` to interface with browser wallet providers (window.ethereum) directly and use Alloy for type definitions and RPC calls where possible.

#### 3.2 Implement EIP-6963 Discovery

**File:** `crates/leptos-rainbowkit/src/wallets/eip6963.rs`

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Event, CustomEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EIP6963ProviderInfo {
    pub uuid: String,
    pub name: String,
    pub icon: String,
    pub rdns: String,
}

#[derive(Debug, Clone)]
pub struct EIP6963Provider {
    pub info: EIP6963ProviderInfo,
    pub provider: JsValue,
}

/// Listen for EIP-6963 "eip6963:announceProvider" events
pub fn discover_wallets() -> Vec<EIP6963Provider> {
    let window = web_sys::window().expect("no window");

    // Storage for discovered providers
    let providers = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

    // Create event listener
    let providers_clone = providers.clone();
    let closure = Closure::wrap(Box::new(move |event: Event| {
        if let Ok(custom_event) = event.dyn_into::<CustomEvent>() {
            if let Ok(detail) = custom_event.detail().into_serde::<serde_json::Value>() {
                // Parse provider info from event detail
                // Implementation details omitted for brevity
            }
        }
    }) as Box<dyn FnMut(_)>);

    window.add_event_listener_with_callback(
        "eip6963:announceProvider",
        closure.as_ref().unchecked_ref()
    ).ok();

    closure.forget(); // Keep listener alive

    // Request providers
    window.dispatch_event(
        &CustomEvent::new("eip6963:requestProvider").unwrap()
    ).ok();

    // Return discovered providers (async handling needed in practice)
    providers.borrow().clone()
}
```

#### 3.3 Implement Wallet Trait

**File:** `crates/leptos-rainbowkit/src/wallets/wallet.rs`

```rust
use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetadata {
    pub id: String,
    pub name: String,
    pub icon_url: String,
    pub icon_background: String,
    pub download_urls: Option<DownloadUrls>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrls {
    pub android: Option<String>,
    pub ios: Option<String>,
    pub chrome: Option<String>,
    pub firefox: Option<String>,
    pub edge: Option<String>,
}

#[async_trait::async_trait(?Send)]
pub trait WalletConnector {
    fn metadata(&self) -> &WalletMetadata;

    async fn connect(&self) -> Result<Address, JsValue>;

    async fn disconnect(&self) -> Result<(), JsValue>;

    fn is_installed(&self) -> bool;

    fn get_provider(&self) -> Option<JsValue>;
}
```

#### 3.4 Implement MetaMask Connector

**File:** `crates/leptos-rainbowkit/src/wallets/connectors/metamask.rs`

```rust
use super::super::wallet::{WalletConnector, WalletMetadata};
use alloy::primitives::Address;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Array, Reflect};

pub struct MetaMaskConnector {
    metadata: WalletMetadata,
}

impl MetaMaskConnector {
    pub fn new() -> Self {
        Self {
            metadata: WalletMetadata {
                id: "metamask".to_string(),
                name: "MetaMask".to_string(),
                icon_url: "data:image/svg+xml;base64,...".to_string(),
                icon_background: "#fff".to_string(),
                download_urls: Some(/* ... */),
            },
        }
    }

    fn get_ethereum(&self) -> Option<JsValue> {
        let window = web_sys::window()?;
        Reflect::get(&window, &"ethereum".into()).ok()
    }
}

#[async_trait::async_trait(?Send)]
impl WalletConnector for MetaMaskConnector {
    fn metadata(&self) -> &WalletMetadata {
        &self.metadata
    }

    async fn connect(&self) -> Result<Address, JsValue> {
        let ethereum = self.get_ethereum()
            .ok_or_else(|| JsValue::from_str("MetaMask not installed"))?;

        // Request accounts
        let method = JsValue::from_str("eth_requestAccounts");
        let params = Array::new();

        let request_args = js_sys::Object::new();
        Reflect::set(&request_args, &"method".into(), &method)?;
        Reflect::set(&request_args, &"params".into(), &params)?;

        let promise = Reflect::get(&ethereum, &"request".into())?;
        let promise = js_sys::Function::from(promise);
        let result = promise.call1(&ethereum, &request_args)?;

        let accounts = JsFuture::from(js_sys::Promise::from(result)).await?;
        let accounts = Array::from(&accounts);

        let address_str = accounts.get(0).as_string()
            .ok_or_else(|| JsValue::from_str("No accounts found"))?;

        address_str.parse::<Address>()
            .map_err(|e| JsValue::from_str(&format!("Invalid address: {:?}", e)))
    }

    async fn disconnect(&self) -> Result<(), JsValue> {
        Ok(())
    }

    fn is_installed(&self) -> bool {
        self.get_ethereum().is_some()
    }

    fn get_provider(&self) -> Option<JsValue> {
        self.get_ethereum()
    }
}
```

#### 3.5 Implement Connection State

**File:** `crates/leptos-rainbowkit/src/state/connection.rs`

```rust
use leptos::prelude::*;
use alloy::primitives::Address;
use crate::wallets::wallet::WalletConnector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub status: RwSignal<ConnectionStatus>,
    pub address: RwSignal<Option<Address>>,
    pub connector_id: RwSignal<Option<String>>,
}

impl ConnectionState {
    pub fn new() -> Self {
        Self {
            status: RwSignal::new(ConnectionStatus::Disconnected),
            address: RwSignal::new(None),
            connector_id: RwSignal::new(None),
        }
    }

    pub async fn connect<C: WalletConnector>(&self, connector: &C) -> Result<(), JsValue> {
        self.status.set(ConnectionStatus::Connecting);

        match connector.connect().await {
            Ok(address) => {
                self.address.set(Some(address));
                self.connector_id.set(Some(connector.metadata().id.clone()));
                self.status.set(ConnectionStatus::Connected);
                Ok(())
            }
            Err(e) => {
                self.status.set(ConnectionStatus::Disconnected);
                Err(e)
            }
        }
    }

    pub async fn disconnect(&self) -> Result<(), JsValue> {
        self.address.set(None);
        self.connector_id.set(None);
        self.status.set(ConnectionStatus::Disconnected);
        Ok(())
    }
}

pub fn provide_connection_state() -> ConnectionState {
    let state = ConnectionState::new();
    provide_context(state.clone());
    state
}

pub fn use_connection_state() -> ConnectionState {
    expect_context::<ConnectionState>()
}
```

#### 3.6 Implement use_wallet Hook

**File:** `crates/leptos-rainbowkit/src/hooks/use_wallet.rs`

```rust
use leptos::prelude::*;
use crate::state::connection::{use_connection_state, ConnectionStatus};
use alloy::primitives::Address;

pub struct WalletInfo {
    pub address: Signal<Option<Address>>,
    pub is_connected: Signal<bool>,
    pub is_connecting: Signal<bool>,
    pub connector_id: Signal<Option<String>>,
}

pub fn use_wallet() -> WalletInfo {
    let state = use_connection_state();

    let address = Signal::derive(move || state.address.get());
    let is_connected = Signal::derive(move || state.status.get() == ConnectionStatus::Connected);
    let is_connecting = Signal::derive(move || state.status.get() == ConnectionStatus::Connecting);
    let connector_id = Signal::derive(move || state.connector_id.get());

    WalletInfo {
        address,
        is_connected,
        is_connecting,
        connector_id,
    }
}
```

#### 3.7 Update ConnectModal with Wallet List

**File:** `crates/leptos-rainbowkit/src/components/modals/connect.rs`

```rust
use leptos::prelude::*;
use crate::components::primitives::dialog::Dialog;
use crate::state::modal::{use_modal_state, ModalType};
use crate::state::connection::use_connection_state;
use crate::wallets::connectors::metamask::MetaMaskConnector;

#[component]
pub fn ConnectModal() -> impl IntoView {
    let modal_state = use_modal_state();
    let connection_state = use_connection_state();

    let is_open = modal_state.is_open(ModalType::Connect);
    let on_close = Callback::new(move |_| modal_state.close());

    // Create wallet connectors
    let metamask = MetaMaskConnector::new();

    let handle_metamask_click = move |_| {
        spawn_local(async move {
            match connection_state.connect(&metamask).await {
                Ok(_) => {
                    modal_state.close();
                }
                Err(e) => {
                    log::error!("Failed to connect: {:?}", e);
                }
            }
        });
    };

    view! {
        <Dialog open=is_open on_close=on_close>
            <h2 class="text-2xl font-bold mb-4 text-rk-text">
                "Connect Wallet"
            </h2>
            <div class="space-y-2">
                <button
                    class="w-full rk-button-secondary flex items-center gap-3"
                    on:click=handle_metamask_click
                >
                    <span>"MetaMask"</span>
                </button>
                // More wallets to be added
            </div>
        </Dialog>
    }
}
```

#### 3.8 Update ConnectButton to Show Address

**File:** `crates/leptos-rainbowkit/src/components/connect_button.rs`

```rust
use leptos::prelude::*;
use crate::state::modal::use_modal_state;
use crate::hooks::use_wallet::use_wallet;
use crate::utils::format::format_address;

#[component]
pub fn ConnectButton() -> impl IntoView {
    let modal_state = use_modal_state();
    let wallet = use_wallet();

    let handle_click = move |_| {
        if wallet.is_connected.get() {
            modal_state.open_account();
        } else {
            modal_state.open_connect();
        }
    };

    let button_text = move || {
        if wallet.is_connecting.get() {
            "Connecting...".to_string()
        } else if let Some(addr) = wallet.address.get() {
            format_address(&addr)
        } else {
            "Connect Wallet".to_string()
        }
    };

    view! {
        <button
            class="rk-button-primary"
            on:click=handle_click
            disabled=move || wallet.is_connecting.get()
        >
            {button_text}
        </button>
    }
}
```

#### 3.9 Implement Address Formatting

**File:** `crates/leptos-rainbowkit/src/utils/format.rs`

```rust
use alloy::primitives::Address;

pub fn format_address(address: &Address) -> String {
    let addr_str = format!("{:?}", address);
    format!("{}...{}", &addr_str[0..6], &addr_str[addr_str.len()-4..])
}

pub fn format_balance(balance: u128, decimals: u8) -> String {
    let divisor = 10u128.pow(decimals as u32);
    let whole = balance / divisor;
    let fractional = balance % divisor;

    format!("{}.{:04}", whole, fractional / (divisor / 10000))
}
```

#### 3.10 Validation

```bash
# Install MetaMask browser extension (if not installed)
# Run example
just dev

# Test:
# 1. Click "Connect Wallet"
# 2. Select MetaMask
# 3. Approve connection in MetaMask popup
# 4. Button should show shortened address (0x1234...5678)
# 5. Clicking button again should open account modal (skeleton)
```

---

## Phase 4: Advanced Features

**Goal:** Implement transactions, ENS, balance fetching, i18n, and complete theming.

### Tasks

#### 4.1 Implement Balance Fetching

**File:** `crates/leptos-rainbowkit/src/hooks/use_balance.rs`

```rust
use leptos::prelude::*;
use alloy::primitives::Address;
use alloy::providers::Provider;
use crate::provider::client::HttpProvider;

#[derive(Clone)]
pub struct BalanceInfo {
    pub value: Signal<Option<u128>>,
    pub formatted: Signal<String>,
    pub is_loading: Signal<bool>,
}

pub fn use_balance(
    address: Signal<Option<Address>>,
    provider: HttpProvider,
) -> BalanceInfo {
    let (balance, set_balance) = signal(None::<u128>);
    let (is_loading, set_is_loading) = signal(false);

    Effect::new(move || {
        if let Some(addr) = address.get() {
            set_is_loading.set(true);

            spawn_local(async move {
                match provider.get_balance(addr).await {
                    Ok(bal) => {
                        set_balance.set(Some(bal));
                    }
                    Err(e) => {
                        log::error!("Failed to fetch balance: {:?}", e);
                    }
                }
                set_is_loading.set(false);
            });
        }
    });

    let formatted = Signal::derive(move || {
        balance.get()
            .map(|b| crate::utils::format::format_balance(b, 18))
            .unwrap_or_default()
    });

    BalanceInfo {
        value: balance.into(),
        formatted,
        is_loading: is_loading.into(),
    }
}
```

#### 4.2 Implement ENS Resolution

**File:** `crates/leptos-rainbowkit/src/hooks/use_ens.rs`

```rust
use leptos::prelude::*;
use alloy::primitives::Address;
// use alloy::providers::Provider; // ENS support via Alloy

pub fn use_ens_name(address: Signal<Option<Address>>) -> Signal<Option<String>> {
    let (ens_name, set_ens_name) = signal(None::<String>);

    Effect::new(move || {
        if let Some(_addr) = address.get() {
            // TODO: Implement ENS lookup when Alloy WASM support improves
            // For now, return None
            set_ens_name.set(None);
        }
    });

    ens_name.into()
}
```

#### 4.3 Implement Transaction Store

**File:** `crates/leptos-rainbowkit/src/state/transaction.rs`

```rust
use leptos::prelude::*;
use alloy::primitives::{Address, TxHash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: TxHash,
    pub status: TransactionStatus,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct TransactionStore {
    transactions: RwSignal<HashMap<Address, Vec<Transaction>>>,
}

impl TransactionStore {
    pub fn new() -> Self {
        // Load from localStorage if available
        let initial = Self::load_from_storage();

        Self {
            transactions: RwSignal::new(initial),
        }
    }

    pub fn add_transaction(&self, address: Address, tx: Transaction) {
        self.transactions.update(|txs| {
            txs.entry(address)
                .or_insert_with(Vec::new)
                .push(tx);
        });

        self.save_to_storage();
    }

    pub fn get_pending(&self, address: Address) -> Vec<Transaction> {
        self.transactions.with(|txs| {
            txs.get(&address)
                .map(|list| {
                    list.iter()
                        .filter(|tx| matches!(tx.status, TransactionStatus::Pending))
                        .cloned()
                        .collect()
                })
                .unwrap_or_default()
        })
    }

    fn load_from_storage() -> HashMap<Address, Vec<Transaction>> {
        // Implementation using web_sys::Storage
        HashMap::new()
    }

    fn save_to_storage(&self) {
        // Implementation using web_sys::Storage
    }
}

pub fn provide_transaction_store() -> TransactionStore {
    let store = TransactionStore::new();
    provide_context(store.clone());
    store
}

pub fn use_transaction_store() -> TransactionStore {
    expect_context::<TransactionStore>()
}
```

#### 4.4 Implement i18n System

**File:** `crates/leptos-rainbowkit/src/i18n/mod.rs`

```rust
pub mod locales;

use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    EnUs,
    EsEs,
    FrFr,
    // Add more as needed
}

pub struct I18n {
    locale: RwSignal<Locale>,
    translations: HashMap<Locale, HashMap<&'static str, &'static str>>,
}

impl I18n {
    pub fn new(locale: Locale) -> Self {
        let mut translations = HashMap::new();
        translations.insert(Locale::EnUs, locales::en_us::translations());
        translations.insert(Locale::EsEs, locales::es_es::translations());

        Self {
            locale: RwSignal::new(locale),
            translations,
        }
    }

    pub fn t(&self, key: &str) -> String {
        let locale = self.locale.get();
        self.translations
            .get(&locale)
            .and_then(|t| t.get(key))
            .map(|s| s.to_string())
            .unwrap_or_else(|| key.to_string())
    }

    pub fn set_locale(&self, locale: Locale) {
        self.locale.set(locale);
    }
}

pub fn provide_i18n(locale: Locale) -> I18n {
    let i18n = I18n::new(locale);
    provide_context(i18n.clone());
    i18n
}

pub fn use_i18n() -> I18n {
    expect_context::<I18n>()
}
```

**File:** `crates/leptos-rainbowkit/src/i18n/locales/en_us.rs`

```rust
use std::collections::HashMap;

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    map.insert("connect_wallet.label", "Connect Wallet");
    map.insert("connect_wallet.title", "Connect a Wallet");
    map.insert("account.title", "Account");
    map.insert("account.disconnect", "Disconnect");
    map.insert("chain.title", "Switch Networks");

    map
}
```

#### 4.5 Implement Account Modal

**File:** `crates/leptos-rainbowkit/src/components/modals/account.rs`

```rust
use leptos::prelude::*;
use crate::components::primitives::dialog::Dialog;
use crate::state::modal::{use_modal_state, ModalType};
use crate::state::connection::use_connection_state;
use crate::hooks::use_wallet::use_wallet;
use crate::utils::format::format_address;

#[component]
pub fn AccountModal() -> impl IntoView {
    let modal_state = use_modal_state();
    let connection_state = use_connection_state();
    let wallet = use_wallet();

    let is_open = modal_state.is_open(ModalType::Account);
    let on_close = Callback::new(move |_| modal_state.close());

    let handle_disconnect = move |_| {
        spawn_local(async move {
            let _ = connection_state.disconnect().await;
            modal_state.close();
        });
    };

    view! {
        <Dialog open=is_open on_close=on_close>
            <Show when=move || wallet.is_connected.get()>
                <div class="space-y-4">
                    <h2 class="text-2xl font-bold text-rk-text">
                        "Account"
                    </h2>

                    <div class="p-4 bg-rk-bg-secondary rounded-rk">
                        <p class="text-sm text-rk-text-secondary mb-1">
                            "Connected Address"
                        </p>
                        <p class="font-mono text-rk-text">
                            {move || wallet.address.get().map(|a| format_address(&a))}
                        </p>
                    </div>

                    <button
                        class="w-full rk-button-secondary"
                        on:click=handle_disconnect
                    >
                        "Disconnect"
                    </button>
                </div>
            </Show>
        </Dialog>
    }
}
```

#### 4.6 Implement QR Code Component

**File:** `crates/leptos-rainbowkit/src/components/primitives/qr_code.rs`

```rust
use leptos::prelude::*;
use qrcode::{QrCode, render::svg};

#[component]
pub fn QrCode(
    #[prop(into)] data: String,
    #[prop(default = 256)] size: usize,
) -> impl IntoView {
    let svg_data = move || {
        QrCode::new(data.clone())
            .ok()
            .map(|code| {
                code.render()
                    .min_dimensions(size as u32, size as u32)
                    .build()
            })
    };

    view! {
        <div class="flex justify-center">
            {move || {
                svg_data().map(|svg| {
                    view! {
                        <div inner_html=svg />
                    }
                })
            }}
        </div>
    }
}
```

#### 4.7 Complete Theme Implementation

**File:** `crates/leptos-rainbowkit/src/theme/light.rs`

```rust
use super::types::{Theme, ThemeConfig, AccentColor};

pub struct LightTheme;

impl Theme for LightTheme {
    fn name(&self) -> &'static str {
        "light"
    }

    fn to_css_vars(&self, config: &ThemeConfig) -> String {
        let accent = match config.accent_color {
            AccentColor::Blue => "0 95% 63%",
            AccentColor::Green => "142 76% 36%",
            AccentColor::Orange => "25 95% 53%",
            AccentColor::Pink => "330 81% 60%",
            AccentColor::Purple => "260 60% 60%",
            AccentColor::Red => "0 72% 51%",
            AccentColor::Custom { h, s, l } => return format!("{} {}% {}%", h, s, l),
        };

        format!(r#"
            --rk-colors-accent: {};
            --rk-colors-bg: 0 0% 100%;
            --rk-colors-bg-secondary: 0 0% 98%;
            --rk-colors-text: 0 0% 7%;
            --rk-colors-text-secondary: 0 0% 38%;
            --rk-colors-border: 0 0% 87%;
        "#, accent)
    }
}
```

(Similar implementations for `dark.rs` and `midnight.rs`)

#### 4.8 Validation

```bash
# Run example
just dev

# Test all features:
# 1. Connect wallet
# 2. View account modal with address
# 3. Disconnect wallet
# 4. Toggle theme (if theme switcher added)
# 5. Check localStorage for transactions (if any)
```

---

## Phase 5: Polish & Distribution

**Goal:** Complete remaining wallets, testing, documentation, and prepare for release.

### Tasks

#### 5.1 Implement Remaining Wallets

- WalletConnect (via WC SDK WASM)
- Coinbase Wallet
- Rainbow Wallet
- Trust Wallet
- Ledger (via WebHID)
- ...and all 73+ original wallets

Each follows the pattern established in `metamask.rs`.

#### 5.2 Implement Chain Switching Modal

**File:** `crates/leptos-rainbowkit/src/components/modals/chain.rs`

Similar to `AccountModal`, list available chains and allow switching via `wallet_switchEthereumChain` RPC call.

#### 5.3 Add Comprehensive Tests

```rust
// tests/wallet_connection.rs
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_format_address() {
        // ...
    }

    #[wasm_bindgen_test]
    async fn test_metamask_connector() {
        // ...
    }
}
```

#### 5.4 Generate Documentation

```bash
# Generate docs
cargo doc --all-features --no-deps

# Add doc comments to all public APIs
```

#### 5.5 Create Documentation Site

Use Leptos SSR or static site generator for docs similar to `site/` in original repo.

#### 5.6 Prepare for crates.io

```toml
# Update Cargo.toml with metadata
[package]
description = "Leptos component library for connecting Ethereum wallets"
documentation = "https://docs.rs/leptos-rainbowkit"
homepage = "https://rainbowkit.com"
readme = "README.md"
keywords = ["leptos", "ethereum", "web3", "wallet", "blockchain"]
categories = ["wasm", "web-programming", "gui"]
```

#### 5.7 Create Release Workflow

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      - run: cargo publish -p leptos-rainbowkit
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}
```

#### 5.8 Final Validation

```bash
# Run all checks
just check
just test
just build

# Test all examples
just example basic
just example advanced

# Verify documentation
just docs
```

---

## Key Technical Challenges

### 1. Alloy WASM Limitations

**Challenge:** Alloy's WASM support is not complete for all features (especially signers).

**Solution:**
- Use `wasm-bindgen` to directly interface with `window.ethereum`
- Use Alloy for types, RPC calls, and ABI encoding where supported
- For signing, delegate to browser wallet's native `eth_signTransaction`

### 2. EIP-6963 in WASM

**Challenge:** Listening to `window` events in WASM requires careful closure management.

**Solution:**
- Use `wasm-bindgen::closure::Closure::forget()` for persistent listeners
- Implement cleanup on component unmount
- Store discovered providers in Leptos signals

### 3. Portal Rendering (Modals)

**Challenge:** Leptos has limited portal support compared to React.

**Solution:**
- Use `leptos::portal::Portal` component
- Render modals to `<body>` similar to React
- Ensure proper event handling for backdrop clicks

### 4. LocalStorage Persistence

**Challenge:** Saving transaction state and recent wallets.

**Solution:**
- Use `web_sys::Storage` API via `wasm-bindgen`
- Serialize/deserialize with `serde_json`
- Implement debounced writes to avoid performance issues

### 5. Responsive Design Without Media Queries

**Challenge:** The original uses TypeScript to determine screen size.

**Solution:**
- Use Tailwind's responsive classes (`sm:`, `md:`, `lg:`)
- Use `web_sys::window().inner_width()` for dynamic breakpoints if needed
- Leverage Leptos `Signal` reactivity for window resize events

### 6. Async Image Loading

**Challenge:** Loading wallet icons asynchronously.

**Solution:**
- Use `base64` encoded SVGs in Rust constants
- For dynamic images, use Leptos `Resource` with `fetch` API
- Implement loading states with `Suspense`

### 7. WalletConnect WASM Integration

**Challenge:** WalletConnect SDK may not have native Rust/WASM support.

**Solution:**
- Use WalletConnect JS SDK via `wasm-bindgen`
- Wrap JS API in Rust trait implementation
- Manage QR code display with our `QrCode` component

### 8. Theme Injection

**Challenge:** Vanilla Extract injects `<style>` tags dynamically.

**Solution:**
- Generate CSS at build time with Tailwind
- Inject theme CSS variables via `<style>` tag in `<head>`
- Use Leptos `leptos_meta::Style` component

### 9. i18n Without Runtime Overhead

**Challenge:** 22 locales with many translation keys.

**Solution:**
- Use const `HashMap` or `phf` crate for compile-time translations
- Implement macro for translation key lookups: `t!("key")`
- Only include selected locale in final bundle (code splitting)

### 10. Testing WASM Components

**Challenge:** Testing browser-specific code.

**Solution:**
- Use `wasm-bindgen-test` for unit tests
- Use `wasm-pack test --headless --chrome` for CI
- Mock `window.ethereum` for connector tests

---

## Testing Strategy

### Unit Tests

```bash
# Run all unit tests
cargo test --all-features

# Run WASM tests
wasm-pack test --headless --chrome
```

### Integration Tests

- Test wallet connection flows end-to-end
- Test modal open/close behavior
- Test theme switching
- Test transaction tracking

### Visual Regression Testing

- Use Playwright or similar for screenshot comparisons
- Test across browsers (Chrome, Firefox, Safari)
- Test responsive layouts

### Manual Testing Checklist

- [ ] Connect MetaMask on desktop
- [ ] Connect WalletConnect on mobile
- [ ] Disconnect wallet
- [ ] Switch networks
- [ ] View balance
- [ ] View ENS name (if available)
- [ ] Track pending transaction
- [ ] Toggle theme (light/dark)
- [ ] Change language (i18n)
- [ ] Test on mobile viewport
- [ ] Test keyboard navigation
- [ ] Test screen reader (accessibility)

---

## Success Criteria

### Functionality

- [ ] All original RainbowKit components implemented
- [ ] Support for top 10 wallets (MetaMask, WC, Coinbase, etc.)
- [ ] Theming system fully functional
- [ ] i18n for English + 2 additional languages
- [ ] Transaction tracking operational
- [ ] ENS resolution working

### Performance

- [ ] Initial bundle size < 500KB (gzipped)
- [ ] Time to interactive < 2s (on 3G)
- [ ] Lighthouse score > 90

### Quality

- [ ] >80% test coverage
- [ ] Zero TypeScript errors (removed)
- [ ] Zero Clippy warnings
- [ ] Documentation for all public APIs
- [ ] Working examples for all features

### Distribution

- [ ] Published to crates.io
- [ ] CI/CD pipeline operational
- [ ] Documentation site live
- [ ] Migration guide available

---

## Risk Assessment

### High Risk

1. **Alloy WASM Compatibility** - May require workarounds or alternative libraries
   - Mitigation: Prototype core features early, fallback to `wasm-bindgen` + ethers.js patterns

2. **WalletConnect Integration** - JS SDK may be difficult to wrap
   - Mitigation: Research existing Rust WASM WC implementations, consider iframe approach

### Medium Risk

3. **Performance Regressions** - WASM bundle size could be large
   - Mitigation: Use `wasm-opt`, code splitting, lazy loading

4. **Browser Compatibility** - Some wallets may not work in all browsers
   - Mitigation: Extensive cross-browser testing, fallback strategies

### Low Risk

5. **Theme System Complexity** - CSS variable injection may have edge cases
   - Mitigation: Thorough testing, use Tailwind's built-in dark mode

6. **i18n Maintenance** - Keeping 22 locales in sync
   - Mitigation: Only implement en-US initially, community contributions for others

---

## Execution Checklist

### Phase 1: Foundation
- [ ] Create flake.nix
- [ ] Setup workspace Cargo.toml
- [ ] Remove TypeScript files
- [ ] Initialize core library crate
- [ ] Setup Tailwind CSS
- [ ] Create basic example
- [ ] Create Justfile
- [ ] Validate: Example runs

### Phase 2: Core Components
- [ ] Implement lib.rs + prelude
- [ ] Implement theme types
- [ ] Implement Box primitive
- [ ] Implement Dialog primitive
- [ ] Implement modal state
- [ ] Implement RainbowKitProvider
- [ ] Implement basic ConnectButton
- [ ] Implement ConnectModal skeleton
- [ ] Update example app
- [ ] Validate: Modal opens/closes

### Phase 3: Wallet Integration
- [ ] Setup Alloy provider
- [ ] Implement EIP-6963 discovery
- [ ] Implement Wallet trait
- [ ] Implement MetaMask connector
- [ ] Implement connection state
- [ ] Implement use_wallet hook
- [ ] Update ConnectModal with wallet list
- [ ] Update ConnectButton with address
- [ ] Implement address formatting
- [ ] Validate: MetaMask connects

### Phase 4: Advanced Features
- [ ] Implement balance fetching
- [ ] Implement ENS resolution
- [ ] Implement transaction store
- [ ] Implement i18n system
- [ ] Implement AccountModal
- [ ] Implement QR code component
- [ ] Complete theme implementation
- [ ] Validate: All features work

### Phase 5: Polish & Distribution
- [ ] Implement remaining wallets (70+)
- [ ] Implement chain switching
- [ ] Add comprehensive tests
- [ ] Generate documentation
- [ ] Create documentation site
- [ ] Prepare crates.io metadata
- [ ] Create release workflow
- [ ] Final validation

---

## Conclusion

This migration plan provides a systematic approach to converting RainbowKit from TypeScript/React to Rust/Leptos. By following the phased approach and addressing key technical challenges early, we can deliver a production-ready Leptos component library for web3 wallet connections.

The resulting library will offer:
- **Superior performance** via WASM and fine-grained reactivity
- **Type safety** throughout the stack
- **Smaller bundle sizes** compared to React
- **Native Rust ecosystem integration** for blockchain tooling

Next steps: Begin Phase 1 and validate the foundation before proceeding to wallet integration.
