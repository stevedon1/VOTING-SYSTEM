
cargo build --target wasm32-unknown-unknown --release


near deploy --wasmFile target/wasm32-unknown-unknown/release/net_packages.wasm --accountId don.arthurkamau.testnet


near call don.arthurkamau.testnet register_voter '{"name":"andrew"}' --accountId don.arthurkamau.testnet

near call don.arthurkamau.testnet register_candidate '{"name":"steve", "position":"president"}' --accountId don.arthurkamau.testnet

near call don.arthurkamau.testnet get_voters --accountId don.arthurkamau.testnet