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

# Run specific example
example NAME="basic":
  cd examples/{{NAME}} && trunk serve --open

# Build example for production
build-example NAME="basic":
  cd examples/{{NAME}} && trunk build --release

# Watch and rebuild library on changes
watch:
  cargo watch -x 'build --target wasm32-unknown-unknown'
