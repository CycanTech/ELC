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

## 3.Milestone 1

### Contracts

#### ELC token contract

standard ERC20 token, ELC Token ELC token is the stablecoin within the ELC contract system, with the initial pegged rate of 1 USD value in the year of 2021. The pegged rate varies over time based on inflation and depends on whether its supply expands or shrinks based on the system LR.

##### functions

- `pub fn transfer_ownership(&mut self, new_owner: AccountId)` :  When deployed, transfer owner to pool contract.
- `pub fn mint(&mut self, user: AccountId, amount: Balance) -> Result<()>` : Pool contract mint amount of ELC to liquidity provider.
- ` pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()>`: Holders transfer amount of ELC.

#### rELP Token contract

rELP token is a risk asset in the ELC system (an equity in the system). Participants are able to add rELP liquidity to the multifunction pool, which could be realized by sending ELP to pool contract. The system encourages user to do long-term liquidity mining so as to keep the reserve pool more stable with liquidity.

##### functions

- `pub fn transfer_ownership(&mut self, new_owner: AccountId)` :  When deployed, transfer owner to pool contract.
- `pub fn mint(&mut self, user: AccountId, amount: Balance) -> Result<()>` : Pool contract mint amount of rELP to liquidity provider.
- `pub fn burn(&mut self, user: AccountId, amount: Balance) -> Result<()>`: Pool contract burn amount of rELP when user redeem liquidity
- ` pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()>`: Holders transfer amount of rELP.
- `pub fn hold_time(&self, user: AccountId, now_time: u128) -> (u128, u128)`: Compute rELP holders holder time share and holder time
- `pub fn hold_time_all(&self, now_time: u128) -> u128 `: Compute rELP holders all share
- `pub fn mint_to_holders(&mut self, expand_amount:u128) -> Result<()> `: When expand ELC, 5% amount of ELC transfer to holders.

#### Multifunction Pool contract

##### functions

- `pub fn add_liquidity(&mut self) -> (Balance, Balance)` :  Reserve Participants can send ELPs to the ELC system in exchange for both ELC and rELP, or rELP alone, depending on what the system liability ratios are. When the system LR is less than 30%, participants who send ELPs to the ELC system can obtain a certain amount of ELC and rELP minted and the LR of the system remains unchanged. When the system LR is higher than 30%, participants who send ELPs to the ELC system will only get rELP, and this can decrease LR.

- `pub fn remove_liquidity(&mut self, relp_amount: Balance) -> Balance` : Users remove their amount of ELP liquidity, burn amount of rELP

- ` pub fn expand_elc(&mut self)`: When ELC is higher than 102% of ELCaim, if the multifunction pool has ELC, it swaps amount of ELCs for amount of ELPs, increasing ELC supply. When ELC is lower than 98% of ELCaim, the multifuncitonal pool swaps amount of ELPs for amount of ELCs.

  When ELC is higher than 102% of ELCaim and LR is <= 0.7, when min (weighted average price of ELC 24 hours, weighted average price of ELC 1 hour) > $1, additional ELCs are issued, additional ELC will allocate to rELP liquidity provider (95% of issued ELC) and the multifunction pool (5% of issued ELC).

- `pub fn contract_elc(&mut self)`: when ELP price is lower than 98% of ELCaim, the ELC system will swap ELP for ELC until all ELP in the pool are swapped into ELC to keep the price back to ELCaim.

- `pub fn update_elc_aim(&mut self)`: Every liquidity operate update ELC aim price.

  The generation time of ELP is 6 seconds per block. The ELC aim price rises every 10,000 blocks in K, K is an anti-inflation factor. The inflation factor K can be adjusted through the ELP governance mechanism. rELP holders can vote for the adjustment of the anti-inflation factor K when the USD inflation goes into hyperinflation.

  ELCaim(after adjustment) = ELCaim(before adjustment) * (1+K)

  initial K is 0.00005

- `pub fn get_reward(&mut self) -> Balance `: Liquidity Mining Participants who have deposited ELPs to the reserve pool contract, can obtain rELP and use rELP to participate in the liquid mining. In order to maintain the size of reserves, participants are encouraged to do long-term liquidity mining. Participants who join liquidity mining will earn more tokens if they hold the tokens longer.

- `pub fn liability_ratio(&self) -> u128`：compute LR:  LR = Value/P(ELP) * Amount(ELP)

  Where:

  - Value is the value of the ELCs that have been issued
  - P is the price of ELP
  - Amount is the number of ELP in reserve pool

Risk Reserve Fund The risk reserve fund includes two assets (ELPs and ELCs). The purpose of the Risk Reserve Fund is to keep the ELC prices floating between 98% and 102% of the ELC aim price (ELCaim). For example, 1 million ELPs out of the overall 21 million of ELPs initial issuance are used as initial risk reserves. Such risk reserve fund is used as follows: when ELC price is lower than 98% of ELCaim, the ELC system will swap ELP for ELC until all ELP in the pool are swapped into ELC to keep the price back to ELCaim. When ELP price is higher than 102% of ELCaim, the ELC system will swap ELC to ELP until all ELCs in the pool are swapped into ELP to keep the price back to ELCaim.

Pool storage

```rust
pub struct Pool {
        elcaim: u128,    // elc aim price
        k: u128, //inflation factor
        reserve: Balance,  //ELP reserve funds
        risk_reserve: Balance, //ELP risk reserve funds
        elc_risk_reserve_source: u128, //ELP risk reserve swapped ELC funds
        elc_reserve_source: u128,//ELP reserve funds swapped ELC funds
        k_update_time: u128, // inflation factor update time
        last_expand_time: u128, // last expand trigger time
        last_contract_time: u128, // last contract trigger time
        adjust_gap: u128, // 24 hour
        elc_contract: Lazy<ELC>, 
        elc_accountid: AccountId,
        relp_contract: Lazy<RELP>,
        relp_accountid: AccountId,
        oracle_contract: Lazy<Oracle>,
        exchange_contract: Lazy<PatraExchange2>,
        exchange_accountid: AccountId,  // ELC and ELP swap pair address
    }
```



#### Oracle Contract

The oracle of reserve asset ELP and stablecoin ELC is implemented by [ELP-runtime-node](https://github.com/CycanTech/ELP-runtime-node), price-fetch pallet,  Which can feed ELP and ELC price th oracle contract.

##### functions

- `pub fn update(&mut self, elp_price: u128, elc_price: u128) -> bool`：price-fetch pallet feed price to oracle contract every hour.
- `pub fn elp_price(&self) -> u128`: pool contract query ELP price.
- `pub fn elc_price(&self) -> u128`: pool contract query ELC price.



## 4. Setup

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

### Compile all contracts

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

