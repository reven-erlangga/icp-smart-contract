#!/usr/bin/env bash

function generate_did() {
  local canister=$1
  canister_root="src/$canister"

  echo "Building canister: $canister"
  echo "Canister root: $canister_root"

  cargo build --manifest-path="$canister_root/Cargo.toml" \
      --target wasm32-unknown-unknown \
      --release --package "$canister" \

  echo "Build complete, generating did file..."

  candid-extractor "target/wasm32-unknown-unknown/release/$canister.wasm" > "$canister_root/$canister.did"
}

CANISTERS=icp_rust_boilerplate_backend

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    generate_did "$canister"
done
