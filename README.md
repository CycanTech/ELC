# ELC — Everlasting Cash
## 1. Introduction

ELC is a decentralized anti-inflation stablecoin with a reserve system, a hybrid of a crypto-collateralized and algorithmic stablecoin mechanism, with the collateralized mechanism providing the underlying value guarantee, and the algorithmic mechanism incentivizing the participants of the collateralized mechanism on one hand and hedging the downside risk when the demand for the stablecoin is insufficient on the other. 

The Cycan Network (CYN) is an isomorphic parachain on the Polkadot network. The Everlasting Parachain (ELP), as Cycan’s canary network, is the isomorphic parachain on the Kusama network, which is Polkadot’s canary network. The Cycan Network aims to build a decentralized autonomous trust (DAT) protocol for everyone.The ELC protocol is a smart contract protocol based on the Cycan Network/Everlasting Parachain.

## 2. Overview

Based on the Polkadot/Kusama ecosystem,

1.Adopting a crypto-collateralized mechanism to ensure the basic value of ELC.

2.Using a reserve-based liquidity mining mechanism to issue additional ELC, ELC grows in an orderly manner with the expansion of demand.

3.Adopting the anti-inflation model and using the anti-inflation factor k to adjust the goal of ELC price control. The annual appreciation rate of ELC is roughly the same as the inflation rate of USD.

4.The buffer mechanism with reserves for price falls avoids the death loop trap of algorithmic stablecoins.


## 3. Setup

### Installing Node.js and redspot
We require node >=12.0, if not, you can go to the nodejs website and find out how to install or upgrade.
Or we recommend that you install Node using nvm. Windows users can use nvm-windows instead.

Install redspot 

```
npm i redspot
```

### Substrate Prerequisites
Follow the official installation steps from the Substrate Developer Hub Knowledge Base.
```
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```
### Installing a substrate node(support ink! 3.0)

Recommend using [ELP-runtime-node](https://github.com/CycanTech/ELP-runtime-node), there's an oracle-pallet, which can feed ELC and ELP price into oracle contract. You can follow CycanTech/ELP-runtime-node README.md install ELP node.

If you use other substrate node such as parity's canvas node, there's no oracle-pallet, you need add your orcale, if not, you still can deploy ELC contracts, but oracle contract will provide zero price.

```
cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --force --locked
```

### Run a local node

```
cargo run --release -- --dev
```

### Compile ELC contracts

compile all contracts 
```
npx redspot compile
```

### Deploy ELC contracts
```
npx redspot run scripts/elc.deploy.ts --no-compile
```

