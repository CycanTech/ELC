## Substrate Prerequisites

Here is how to configure for the Ubuntu users, other system users please follow the official [installation](https://substrate.dev/docs/en/knowledgebase/getting-started/) steps from the Substrate Developer Hub Knowledge Base.  

```
sudo apt update  
sudo apt install -y git clang curl libssl-dev llvm libudev-dev
```

```
curl https://sh.rustup.rs -sSf | sh  
source ~/.cargo/env
```

```
rustup default stable  
rustup update  
rustup update nightly  
rustup component add rust-src --toolchain nightly  
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## Installing The Canvas Node

We need to use a Canvas node with the built-in Contracts module. For  this workshop we'll use the pre-designed Substrate node client.

```bash
cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --tag v0.1.8 --force --locked
```

## Install binaryen

binaryen(version >= 99) is used to optimize the WebAssembly bytecode of the contract.

[download a binary release directly](https://github.com/WebAssembly/binaryen/releases)

```bash
tar -xvf binaryen-version_101-x86_64-linux.tar.gz  
sudo cp binaryen-version_101/include/* /usr/include/  
sudo cp binaryen-version_101/lib64/* /usr/lib64/  
sudo cp binaryen-version_101/bin/* /usr/bin/
```

## Install cargo-contract

cargo-contract is the ink! command line utility which will make setting up Substrate smart contract projects easier.

```bash
cargo install cargo-contract --vers ^0.12 --force --locked
```



