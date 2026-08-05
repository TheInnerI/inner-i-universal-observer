usage:
  just dev           # Start all development services
  just build         # Build everything
  just test          # Run all tests
  just lint          # Lint all code
  just clean         # Clean build artifacts
  just mobile-ios    # Run iOS mobile app
  just mobile-android # Run Android mobile app
  just node-start    # Start Observer Node
  just control-center # Start Control Center
  just relay-start   # Start Relay service
  just demo          # Run the Wake-Up Demo
  just proto         # Regenerate protobuf stubs
  just release       # Build release artifacts

# ---- Top-level ----

dev:
  @echo "Starting Inner I Universal Observer..."
  @echo ""
  cargo build -p observer-node &
  cd apps/control-center && pnpm dev &
  cd apps/relay && pnpm dev &
  wait

build:
  cargo build --release
  cd apps/control-center && pnpm build
  cd apps/mobile && pnpm build

test:
  cargo test
  cd apps/control-center && pnpm test
  cd apps/mobile && pnpm test

lint:
  cargo clippy -- -D warnings
  cd apps/control-center && pnpm lint
  cd apps/mobile && pnpm lint

clean:
  cargo clean
  rm -rf apps/mobile/dist apps/control-center/.next apps/relay/dist
  rm -rf target/

# ---- Individual services ----

node-start:
  cargo run -p observer-node

control-center:
  cd apps/control-center && pnpm dev

relay-start:
  cd apps/relay && pnpm dev

mobile-ios:
  cd apps/mobile && npx expo run:ios

mobile-android:
  cd apps/mobile && npx expo run:android

# ---- Protocol ----

proto:
  @echo "Generating protobuf stubs..."
  # Rust
  mkdir -p crates/iiop-engine/src/generated
  protoc --proto_path=protocol \
    --rust_out=crates/iiop-engine/src/generated \
    protocol/iiop.proto
  # TypeScript
  mkdir -p packages/iiop-types/src/generated
  protoc --proto_path=protocol \
    --ts_out=packages/iiop-types/src/generated \
    protocol/iiop.proto
  @echo "Done."

# ---- Demo ----

demo:
  @echo "=== Inner I Universal Observer — Wake-Up Demo ==="
  cargo run -p observer-node &
  sleep 2
  cargo run --example demo --manifest-path crates/observer-node/Cargo.toml

# ---- Release ----

release:
  cargo build --release
  @echo "Release binaries in target/release/"
  @echo "  observer-node"
  @echo "  observer-cli"
