set -e

RUSTFLAG='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p out
cp target/wasm32-unknown-unknown/release/*.wasm out/staking-contract.wasm