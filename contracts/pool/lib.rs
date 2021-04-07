#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]
#![allow(unused_mut)]

pub use self::pool::Pool;
use ink_lang as ink;

#[ink::contract]
mod pool {
    use elc::ELC;
    use relp::RELP;
    use oracle::Oracle;
    use exchange2::PatraExchange as PatraExchange2;
//    #[cfg(not(feature = "ink-as-dependency"))]
//    use factory::PatraFactory;
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_env::call::FromAccountId;
    use ink_prelude::vec::Vec;
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        lazy::Lazy,
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_prelude::string::String;


    #[ink(storage)]
    pub struct Pool {
        elcaim: u128,
        k: u128, //inflation factor
        reserve: Balance,
        risk_reserve: Balance,
        k_update_time: u128,
        last_expand_time: u128,
        last_contract_time: u128,
        adjust_gap: u128,
        elc_contract: Lazy<ELC>,
        elc_accountid: AccountId,
        relp_contract: Lazy<RELP>,
        relp_accountid: AccountId,
        oracle_contract: Lazy<Oracle>,
//        factory_contract: Lazy<PatraFactory>,
        exchange_contract: Lazy<PatraExchange2>,
        exchange_accountid: AccountId,
    }

    #[ink(event)]
    pub struct AddLiquidity {
        #[ink(topic)]
        sender: AccountId,
        #[ink(topic)]
        elp_amount: Balance,
        #[ink(topic)]
        relp_amount: Balance,
        #[ink(topic)]
        elc_amount: Balance,
    }

    #[ink(event)]
    pub struct RemoveLiquidity {
        #[ink(topic)]
        sender: AccountId,
        #[ink(topic)]
        relp_amount: Balance,
        #[ink(topic)]
        elp_amount: Balance,
    }

    #[ink(event)]
    pub struct ExpandEvent {
        #[ink(topic)]
        expand_type: String,
        #[ink(topic)]
        expand_amount: Balance,
        #[ink(topic)]
        elp_amount: Balance,
    }

    #[ink(event)]
    pub struct ContractEvent {
        #[ink(topic)]
        contract_amount: Balance,
        #[ink(topic)]
        risk_reserve_consumed: Balance,
        reserve_consumed: Balance,
    }

    impl Pool {
        #[ink(constructor)]
        pub fn new(
            risk_reserve: Balance,
            elc_token: AccountId,
            relp_token: AccountId,
            oracle_addr: AccountId,
            exchange_account: AccountId,
        ) -> Self {
            let elc_contract: ELC = FromAccountId::from_account_id(elc_token);
            let relp_contract: RELP = FromAccountId::from_account_id(relp_token);
            let oracle_contract: Oracle = FromAccountId::from_account_id(oracle_addr);
            let exchange_contract: PatraExchange2 = FromAccountId::from_account_id(exchange_account);
            let blocktime = Self::env().block_timestamp().into();
            let instance = Self {
                elcaim: 100,
                k: 5, //0.00005 * 100000
                reserve: 0,
                risk_reserve: risk_reserve,
                k_update_time: blocktime,
                last_expand_time:  blocktime,
                last_contract_time:  blocktime,
                adjust_gap: 3600, // one hour
                oracle_contract: Lazy::new(oracle_contract),
                elc_contract: Lazy::new(elc_contract),
                elc_accountid: elc_token,
                relp_contract: Lazy::new(relp_contract),
                relp_accountid: relp_token,
//                factory_contract: Lazy::new(factory_contract),
                exchange_contract: Lazy::new(exchange_contract),
                exchange_accountid: exchange_account,
            };
            instance
        }

        /// add liquidity for ELP，returns rELP and ELC
        #[ink(message, payable)]
        pub fn add_liquidity(&mut self) -> (Balance, Balance) {
            self.update_elc_aim();
            let caller: AccountId = self.env().caller();
            let elp_amount: Balance = self.env().transferred_balance();
            let (relp_tokens, elc_tokens) = self.compute_liquidity(elp_amount);
            if elc_tokens != 0 {
                assert!(self
                    .elc_contract
                    .mint(caller, elc_tokens)
                    .is_ok());
            }
            assert!(self
                .relp_contract
                .mint(caller, relp_tokens)
                .is_ok());
            self.env().emit_event(AddLiquidity {
                sender: caller,
                elp_amount: elp_amount,
                relp_amount: relp_tokens,
                elc_amount: elc_tokens,
            });
            (relp_tokens, elc_tokens)
        }

        /// compute add-liquidity threshold for internal and external call
        #[ink(message)]
        pub fn compute_liquidity(&self, elp_amount_deposit: Balance) -> (Balance, Balance) {
            let elp_price: u128 = self.oracle_contract.elp_price();
            let elc_price: u128 = self.oracle_contract.elc_price();
            let elc_amount: Balance = self.elc_contract.total_supply();
            let mut relp_tokens: Balance = 0;
            let mut elc_tokens: Balance = 0;
            let mut relp_price = self.relp_price();
            let lr = self.liability_ratio();
            if lr < 30 {
                // compute elp amount make LR >= 30
                let elp_amount_threshold: Balance  = elc_amount * elc_price * 100 / (elp_price * 30);
                if elp_amount_deposit < elp_amount_threshold {
                    relp_tokens = elp_price * elp_amount_deposit / relp_price;
                    elc_tokens = elp_price * elp_amount_deposit * (lr/100000) / relp_price;
                } else {
                    relp_tokens = elp_price * elp_amount_threshold / relp_price +
                        elp_price * (elp_amount_deposit - elp_amount_threshold) * (1- lr/100000)/ relp_price;
                    elc_tokens = elp_price * elp_amount_threshold * (lr/100000) / relp_price;
                }
            } else {
                relp_tokens = elp_price * elp_amount_deposit * (1- lr/100000)/ relp_price;
            };
            (relp_tokens, elc_tokens)
        }

        /// remove liquidity, user can get back their ELP by burn rELP and get their reward
        #[ink(message)]
        pub fn remove_liquidity(&mut self, relp_amount: Balance) -> Balance {
            self.update_elc_aim();
            let caller: AccountId= self.env().caller();
//            let elp_price: u128 = self.oracle_contract.elp_price();
            let lr = self.liability_ratio();
            let relp_balance = self.relp_contract.total_supply();
            let mut elp_amount: Balance = 0;
            assert!(relp_amount > 0);
            //burn relp
            assert!(self
                .relp_contract
                .burn(caller, relp_amount)
                .is_ok());

            //when LR>30，only consider rELP price, else consider rELP and ELC price
            if lr > 30 {
                //compute ELP amount
                //△Amount(ELP) = △Amount(rELP) * p(rELP) / p(ELP)
                // △Amount(ELP) = △Amount(rELP)*Amount(ELP)/Amount(rELP)
                elp_amount = relp_amount * self.reserve / relp_balance;
            } else {
                //compute ELP amount
                //△Amount(ELP) = △Amount(rELP) * p(rELP) / (p(ELP) * (1-LR))
                // △Amount(ELP) = △Amount(rELP)*Amount(ELP)/Amount(rELP) / (1-LR))
                elp_amount =  relp_amount * self.reserve / relp_balance / (1 - lr/100000);
            }

            //redeem ELP
            assert!(self.env().transfer(caller, elp_amount).is_ok());

            self.env().emit_event(RemoveLiquidity {
                sender: caller,
                relp_amount: relp_amount,
                elp_amount: elp_amount,
            });

            //give reward
            self.get_reward();
            elp_amount
        }

        /// anyone hold rELP can get reward
        #[ink(message)]
        pub fn get_reward(&mut self) -> Balance {
            let caller: AccountId= self.env().caller();
            let relp_amount = self.relp_contract.balance_of(caller);
            assert!(relp_amount > 0);
            let now_time: u128 = self.env().block_timestamp().into();
            let (hold_time, hold_realtime) = self.relp_contract.hold_time(caller, now_time);
            let hold_time_all: u128 = self.relp_contract.hold_time_all(now_time);
            //6 seconds per block, every block reward, reward assume reward is 5, decimal is 10^12
            let elp_amount: u128 = hold_time / hold_time_all * (hold_realtime/6) * 5 * 10^12 ;
            if self.risk_reserve > 0 {
                assert!(self.env().transfer(caller, elp_amount).is_ok());
                self.risk_reserve -= elp_amount;
            }
            self.relp_contract.update_hold_time_for_reward(caller, relp_amount, now_time);
            //return elp amount
            elp_amount
        }

        /// when price higher:
        /// 1.call swap contract, swap elc for elp
        /// 2.raise ELC
        #[ink(message)]
        pub fn expand_elc(&mut self) {
            let elc_price: u128 = self.oracle_contract.elc_price();
            let elp_price: u128 = self.oracle_contract.elp_price();
            let lr = self.liability_ratio();
            let elcaim_deviation = self.elcaim * 102 / 100;
            assert!(elc_price > elcaim_deviation);

            //assert time > adjust duration
            let block_time:u128 = self.env().block_timestamp().into();
            let gap: u128 = block_time - self.last_expand_time;
            assert!(gap >= self.adjust_gap);

            let elc_balance = self.elc_contract.balance_of(self.env().account_id());
            let base: u128 = 10;

            // estimate ELC value: value per ELC in swap
            let elc_decimals = self.elc_contract.token_decimals().unwrap_or(0);
            let elp_amount_per_elc = self.exchange_contract.get_token_to_dot_input_price(base.pow(elc_decimals.into()));
            let value_per_elc = elp_amount_per_elc * elp_price;
            assert!(value_per_elc > self.elcaim * (base.pow(12))); //ELP decimals is 12, use elcaim price

            let price_impact_for_swap = (value_per_elc - elcaim_deviation * (base.pow(12))) * 100 / value_per_elc;
            let price_impact_for_expand = (elc_price - self.elcaim) * 100 / self.elcaim ;
            let elc_amount: Balance = self.elc_contract.total_supply();
            let expand_amount = price_impact_for_expand * elc_amount / 100;
            let mut elp_amount:u128 = 0;
            if elc_balance > expand_amount {
                //swap elc for elp
//                if(self.exchange_contract == (&0)) {
//                    self.exchange_contract = self.factory_contract.get_exchange(elc_contract, to_token).unwrap_or(&0);
//                    assert!((self.exchange_contract) != (&0));
//                }
//                let adj_num = self.expand_adj_num;
//                let adj_bignum = adj_num * (base.pow(token_decimals));
                assert!(self.elc_contract.approve(self.exchange_accountid, expand_amount).is_ok());
                elp_amount = self.exchange_contract.swap_token_to_dot_input(expand_amount);
                assert!(elp_amount > 0);
                self.env().emit_event(ExpandEvent {
                    expand_type: String::from("swap"),
                    expand_amount: expand_amount,
                    elp_amount: elp_amount,
                });
            } else {
                //raise ELC
                if lr > 70 {
                    // 95 allocate to ELC holders
                    let mint_to_holders_amount:u128 = expand_amount * 95 / 100;
                    let mint_to_reserve_amount:u128 = expand_amount * 5 / 100;
                    assert!(self.elc_contract.mint(self.relp_accountid, mint_to_holders_amount).is_ok());
                    assert!(self.relp_contract.mint_to_holders(mint_to_holders_amount).is_ok());

                    // 5% allocate to ELP reserve
                    let self_account = self.env().account_id();
                    assert!(self.elc_contract.mint(self_account, mint_to_reserve_amount).is_ok());
                    assert!(self.elc_contract.approve(self.exchange_accountid, expand_amount).is_ok());
                    elp_amount = self.exchange_contract.swap_token_to_dot_input(mint_to_reserve_amount);
                    assert!(elp_amount > 0);
                    self.env().emit_event(ExpandEvent {
                        expand_type: String::from("raise"),
                        expand_amount: expand_amount,
                        elp_amount: elp_amount,
                    });
                }
            }
            self.last_expand_time = block_time;
            self.risk_reserve += elp_amount;
        }

        // when price lower, call swap contract, swap elc for elp
        #[ink(message, payable)]
        pub fn contract_elc(&mut self){
            let elc_price: u128 = self.oracle_contract.elc_price();
            let elp_price: u128 = self.oracle_contract.elp_price();
            let elcaim_deviation = self.elcaim * 98 / 100;
            assert!(elc_price < elcaim_deviation);

            //assert time > adjust duration
            let block_time:u128 = self.env().block_timestamp().into();
            let gap: u128 = block_time - self.last_contract_time;
            assert!(gap >= self.adjust_gap);

            // estimate ELC value: value per ELC in swap
            let base: u128 = 10;
            let elp_amount_per_elc = self.exchange_contract.get_token_to_dot_input_price(base.pow(
                self.elc_contract.token_decimals().unwrap_or(0).into()
            ));
            let value_per_elc = elp_amount_per_elc * elp_price;
            assert!(value_per_elc < self.elcaim * (base.pow(12))); //ELP decimals is 12, use elcaim price

            let price_impact_for_expand = (self.elcaim - elc_price) * 100 / self.elcaim;
            let elc_amount: Balance = self.elc_contract.total_supply();
            let contract_amount = price_impact_for_expand * elc_amount / 100;

            // elp decimals need to be same with elc decimals
            let elp_needed = self.exchange_contract.get_dot_to_token_output_price(contract_amount);
//            if(self.exchange_contract == (&0)) {
//                let to_token = Default::default();
//                self.exchange_contract = self.factory_contract.get_exchange(elc_contract, to_token).unwrap_or(&0);
//                assert!((self.exchange_contract) != (&0));
//            }
            if self.risk_reserve > elp_needed {
                assert!(self.env().transfer(self.exchange_accountid, elp_needed).is_ok());
                let elc_amount = self.exchange_contract.swap_dot_to_token_input();
                assert!(elc_amount > 0);
                self.env().emit_event(ContractEvent {
                    contract_amount: elc_amount,
                    risk_reserve_consumed: elp_needed,
                    reserve_consumed: 0,
                });
                self.risk_reserve -= elp_needed;
            } else {
                //if risk reserve not enough, then use self.risk_reserve + reserve * 2% per day
                if (self.risk_reserve + self.reserve * 2 / 100) > elp_needed {
                    assert!(gap >= (24 * 60 * 60)); // one day later can call this
                    assert!(self.env().transfer(self.exchange_accountid, elp_needed).is_ok());
                    let elc_amount = self.exchange_contract.swap_dot_to_token_input();
                    assert!(elc_amount > 0);
                    self.env().emit_event(ContractEvent {
                        contract_amount: elc_amount,
                        risk_reserve_consumed: self.risk_reserve,
                        reserve_consumed: elp_needed - self.risk_reserve,
                    });
                    self.risk_reserve -= self.risk_reserve;
                }
            }
            self.last_contract_time = block_time;
        }

        ///compute inflation factor, 6 seconds per block, every 10000 adjust ELC aim price
        #[ink(message)]
        pub fn update_elc_aim(&mut self) {
            let block_time:u128 = self.env().block_timestamp().into();
            let elcaim:u128 = self.elcaim;
            let last_update_time = self.k_update_time;
            let k: u128 = (block_time - self.k_update_time) / 6 / 10000;
            if k > 0 {
                self.elcaim = elcaim * (100000 + k) / 100000;
                self.k_update_time = last_update_time + (k * 10000 * 6);
            }
        }

        /// compute liability ratio
        #[ink(message)]
        pub fn liability_ratio(&self) -> u128 {
            let elp_price: u128 = self.oracle_contract.elp_price();
            let elc_price: u128 = self.oracle_contract.elc_price();
            let elp_amount: Balance = self.reserve;
            let elc_amount: Balance = self.elc_contract.total_supply();
            let lr =  elc_amount * elc_price  * 100 /(elp_price * elp_amount); //100 as base
            lr
        }

        ///compute internal relp price for query
        #[ink(message)]
        pub fn relp_price(&self) -> u128 {
            let elp_price: u128 = self.oracle_contract.elp_price();
            let relp_balance = self.relp_contract.total_supply();
            //p(rELP) = p(ELP)*Amount(ELP)/Amount(rELP)
            let relp_price = elp_price * self.reserve / relp_balance;
            relp_price
        }

        #[ink(message)]
        pub fn elp_reserve(&self) -> Balance { self.reserve }

        #[ink(message)]
        pub fn elp_risk_reserve(&self) -> Balance { self.risk_reserve }

        /// define a struct returns all pool states
        #[ink(message)]
        pub fn pool_state(&self)  {

        }
    }
}
