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

            # Rust toolchain with rust-analyzer, clippy, and wasm32 target
            ((rust-bin.selectLatestNightlyWith
              (toolchain: toolchain.default)).override {
                extensions = [ "rust-analyzer" "rust-src" ];
                targets = [ "wasm32-unknown-unknown" ];
              })

            # Development tools
            cargo-watch       # Auto-rebuild on file changes
            cargo-edit        # Cargo add/rm/upgrade commands
            trunk             # WASM bundler for Leptos
            wasm-bindgen-cli  # WASM bindings generator
            just              # Justfile task runner

            # CSS tooling (no Node.js required)
            tailwindcss_4     # Tailwind CSS v4 standalone CLI
          ];

          # Environment variables for Rust development
          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)}/lib/rustlib/src/rust/library";

          shellHook = ''
            # Add ~/.cargo/bin to PATH for cargo-installed tools (leptosfmt, etc.)
            export PATH="$HOME/.cargo/bin:$PATH"

            # Configure Zed editor for Nix-managed Rust toolchain
            mkdir -p .zed

            # Create stable symlink to rust-analyzer
            ln -sf $(command -v rust-analyzer) .zed/rust-analyzer

            # Generate Zed settings.json if it doesn't exist
            if [ ! -f .zed/settings.json ]; then
              cat > .zed/settings.json <<'EOF'
            {
              "lsp": {
                "rust-analyzer": {
                  "binary": {
                    "path": ".zed/rust-analyzer"
                  }
                }
              }
            }
            EOF
            fi

            # Add .zed/ to .gitignore if not already present
            if [ -f .gitignore ] && ! grep -q '^\.zed/' .gitignore; then
              echo '.zed/' >> .gitignore
            fi

            # Install leptosfmt if not already installed
            if ! command -v leptosfmt &> /dev/null; then
              echo "📦 Installing leptosfmt..."
              cargo install leptosfmt
            fi

            echo "🌈 Leptos-RainbowKit development environment loaded"
            echo ""
            echo "🦀 Rust:       $(rustc --version)"
            echo "🎨 Tailwind:   $(tailwindcss --version)"
            echo "📦 Trunk:      $(trunk --version)"
            echo ""
            echo "💡 Quick start:"
            echo "   - List tasks:      just --list"
            echo "   - Dev server:      just dev"
            echo "   - Run example:     just example"
            echo "   - Build library:   just build"
            echo "   - Test:            just test"
            echo "   - Format:          just fmt"
          '';
        };
      }
    );
}
