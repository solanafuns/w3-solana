sbf:
	cargo build-sbf --manifest-path=./w3/Cargo.toml --sbf-out-dir=dist/w3

deploy-remote-local: sbf
	solana config set --url http://127.0.0.1:8899
	solana program deploy dist/w3/w3.so

deploy-remote-dev: sbf
	solana config set --url devnet
	solana program deploy dist/contract/contract.so

deploy-remote-main: sbf
	solana config set --url mainnet-beta
	solana program deploy dist/contract/contract.so
