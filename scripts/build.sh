#!/bin/bash
set -eox pipefail

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build -p sender --target wasm32-unknown-unknown --profile=contract

cp ./target/wasm32-unknown-unknown/contract/sender.wasm res/asi.wasm
