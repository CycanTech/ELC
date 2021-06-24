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

### Substrate and ink! Prerequisites

see the file [InkConfiguration.md](./InkConfiguration.md)

### Installing Node.js and dependency library

We require node >=14.0, if not, you can go to the nodejs website and find out how to install or upgrade.
Or we recommend that you install Node using nvm. Windows users can use nvm-windows instead.

install node for ubuntu users

```
# install node
sudo apt install npm

# check node version, if node version < 14.0, do the following steps to upgrade.
node -v

# install node version management tool `n`
sudo npm install n -g

# install the latest lts version
sudo n lts

# check node version again
node -v
```

Install dependency Library

```
git clone https://github.com/CycanTech/ELC.git
cd ELC
npm i
```

### Compile ELC contracts

compile all contracts: 

- set the default rust version to `nightly-x86_64-unknown-linux-gnu`

- run `rustup show` command to see if the previous step is successful：

  if success, it will show following infos:

  ...

  active toolchain

  nightly-x86_64-unknown-linux-gnu (default)
  rustc 1.55.0-nightly (6a758ea7e 2021-06-22)

- compiling all contracts

```
rustup default nightly-x86_64-unknown-linux-gnu
rustup show  
npx redspot compile
```

### Run a local node in another terminal

```
canvas --dev --tmp
```

### Deploy

- Save contract addresses for testing:

  After successful deployment, all contract addresses are printed on the terminal. Replace the five contract addresses in the file `tests/pub.parameter.ts`.

```
npx redspot run scripts/elc.deploy.ts --no-compile 
```

### Test Project

Run test:

```shell
npx redspot test
```
