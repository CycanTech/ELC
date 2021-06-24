## Testing Guide

### Install dependency Library

```shell
git clone https://github.com/CycanTech/ELC.git
cd ELC
npm i
```

### Run a local node in another terminal

```
canvas --dev --tmp
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

stable-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu (default)

active toolchain
----------------

nightly-x86_64-unknown-linux-gnu (default)
rustc 1.51.0 (2fd73fabe 2021-03-23)
```
It is proper configure for build.

### Compile all contracts

```shell
cd ELC
npx redspot compile
```

### Deploy all contracts

- Save contract addresses for testing:

  After successful deployment, all contract addresses are printed on the terminal. Replace the five contract addresses in the file( `tests/pub.parameter.ts`).

```
npx redspot run scripts/elc.deploy.ts --no-compile 
```

### Test Project

Run test:

```shell
npx redspot test
```

## Acknowledgements

Thanks existing projects & standards:

patractlabs/store-contracts https://github.com/patractlabs/store-contracts

A swap contract of the patractlabs/store-contracts is used to exchange tokens.

## Upstream

All contracts code is developed by the project team, not forked project.

