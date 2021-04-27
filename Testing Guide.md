## Testing Guide

### Install Everlasting Parachain Node

To use the ELP node, you could follow the [README.md](https://github.com/CycanTech/ELP-runtime-node) to install and run the ELP node, which supports our oracle that ELC contracts needed. 

You can also deploy ELC contracts on other substrate nodes such as canvas node, but you need an oracle pallet to feed price to ELC's oracle contract.

### Install redspot

```shell
npm install redspot
```

### Check the rust and ink version

```shell
rustup show
```
If it show as next：

```shell
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/wx/.rustup


installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.51.0 (2fd73fabe 2021-03-23)
```
It is proper configure for build.


### Compile all contracts

```shell
cd ELC
npx redspot compile
```

### Get  patractlabs/store-contracts swap contract 


```shell
cd ..
git clone https://github.com/patractlabs/store-contracts.git
cd patractlabs/artifacts
cp exchange2.contract ../ELC/artifacts
cp lpt.contract ../ELC/artifacts
```

### Deploy

```shell
cd ELC
npx redspot run scripts/elc.deploy.ts --no-compile
```

### Test Project  
Run test:

```shell
cd ELC
npx redspot test
```

### Start front-end

```shell
cd substrate-front-end-template
yarn install
yarn start
```

## Acknowledgements

Thanks existing projects & standards:

patractlabs/store-contracts https://github.com/patractlabs/store-contracts

A swap contract of the patractlabs/store-contracts is used to exchange tokens.

## Upstream
All contracts code is developed by the project team, not forked project.


