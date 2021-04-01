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
    use factory::PatraFactory;
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_env::call::FromAccountId;
    use ink_prelude::vec::Vec;
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        lazy::Lazy,
    };

    #[ink(storage)]
    pub struct Pool {
        elcaim: u128,
        k: u128, //inflation factor
        reserve: Balance,
        risk_reserve: Balance,
        k_update_time: u128,
        last_expand_time: u128,
        last_contract_time: u128,
        expand_adj_num: u128,
        contract_adj_num: u128,
        expand_gap: Vec<OpGap>,
        contract_gap: Vec<OpGap>,
        elc_contract: Lazy<ELC>,
        relp_contract: Lazy<RELP>,
        oracle_contract: Lazy<Oracle>,
        factory_contract: Lazy<PatraFactory>,
        exchange_contract: Lazy<PatraExchange2>,
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
        gaptime: u128,
        #[ink(topic)]
        elc_amount: Balance,
    }

    #[ink(event)]
    pub struct ContractEvent {
        #[ink(topic)]
        gaptime: u128,
        #[ink(topic)]
        elp_amount: Balance,
    }

    pub type OpGap = (u128, u128);

    impl Pool {
        #[ink(constructor)]
        pub fn new(
            reserve: Balance,
            risk_reserve: Balance,
            elc_token: AccountId,
            relp_token: AccountId,
            oracle_addr: AccountId,
            factory_addr: AccountId,
        ) -> Self {
            let elc_contract: ELC = FromAccountId::from_account_id(elc_token);
            let relp_contract: RELP = FromAccountId::from_account_id(relp_token);
            let oracle_contract: Oracle = FromAccountId::from_account_id(oracle_addr);
            let factory_contract: PatraFactory = FromAccountId::from_account_id(factory_addr);
            let instance = Self {
                elcaim: 100,
                k: 5, //0.00005 * 100000
                reserve: reserve,
                risk_reserve: risk_reserve,
                k_update_time: Self::env().block_timestamp().into(),
                last_expand_time:  k_update_time,
                last_contract_time:  k_update_time,
                expand_adj_num: 100,  //around 100 $
                contract_adj_num: 5,
                expand_gap: Vec::new(),
                contract_gap: Vec::new(),
                oracle_contract: Lazy::new(oracle_contract),
                elc_contract: Lazy::new(elc_contract),
                relp_contract: Lazy::new(relp_contract),
                factory_contract: Lazy::new(factory_contract),
                exchange_contract：Default::default();
            };
            instance
        }

        /// 增加流动性(ELP)，返回rELP和ELC
        #[ink(message, payable)]
        pub fn add_liquidity(&mut self) -> (Balance, Balance) {
            self.update_elc_aim(); //首先更新ELCaim价格
            let caller: AccountId = self.env().caller();
            let elp_amount: Balance = self.env().transferred_balance();
            let (relp_tokens, elc_tokens) = self.compute_liquidity(elp_amount);
            if elc_tokens != 0 {
                assert!(self
                    .relp_contract
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
            let lr = self.liability_ratio(); //计算LR
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

        /// 退出流动性，发送ELP给用户,赎回只能使用rELP，
        #[ink(message)]
        pub fn remove_liquidity(&mut self, relp_amount: Balance) -> Balance {
            self.update_elc_aim(); //首先更新ELCaim价格
            let caller: AccountId= self.env().caller();
//            let elp_price: u128 = self.oracle_contract.elp_price();
            let lr = self.liability_ratio(); //计算LR
            let relp_balance = self.relp_contract.total_supply();
            let mut elp_amount: Balance = 0;
            assert!(relp_amount > 0);
            //burn relp
            assert!(self
                .relp_contract
                .burn(caller, relp_amount)
                .is_ok());

            //正向兑换rELP时 LR>30，ELP仅兑换rELP，反向兑亦然
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

        /// 持有rELP即可领取奖励
        #[ink(message)]
        pub fn get_reward(&mut self) -> Balance {
            let caller: AccountId= self.env().caller();
            let relp_amount = self.relp_contract.balance_of(caller);
//            assert!(relp_amount > 0);
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

        /// when price higher, call swap contract, swap elp for elc
        /// note: 扩张，swap选择交易所待定，提供给外部做市商调用，保证每次小量交易, 记录时间间隔
        #[ink(message)]
        pub fn expand_elc(&mut self) {
            let elc_price: u128 = self.oracle_contract.elc_price();
            let elcaim = self.elcaim;
            assert!(elc_price > elcaim * 102 / 100);

            //调用swap，卖出ELC，买入ELP
            if(self.exchange_contract == (&0)) {
                let to_token = Default::default();
                self.exchange_contract = self.factory_contract.get_exchange(elc_contract, to_token).unwrap_or(&0);
                assert!((self.exchange_contract) != (&0));
            }

            let exchange_info: ExchangeInfo = self.exchange_contract.exchange_info();
            let token_decimals = exchange_info.from_decimals;
            let base: u128 = 10;
            let adj_num = self.expand_adj_num;
            let adj_bignum = adj_num * (base.pow(token_decimals));
            let sold_amount = self.exchange_contract.swap_token_to_dot_input(adj_bignum);
            assert!(sold_amount);
            let buy_amount = self.exchange_contract.swap_token_to_dot_output(sold_amount);
            assert!(buy_amount);

            let block_time:u128 = self.env().block_timestamp().into();
            let gap: u128 = block_time - self.last_expand_time;
            self.last_expand_time = block_time;
            self.expand_gap.push((gap, adj_bignum));
            self.env().emit_event(ExpandEvent {
                gaptime: gap,
                elc_amount: adj_bignum,
            }

            ///zengfa


        }

        /// when price lower, call swap contract, swap elc for elp
        #[ink(message, payable)]
        pub fn contract_elc(&mut self){
            let elc_price: u128 = self.oracle_contract.elc_price();
            let elcaim = self.elcaim;
            assert!(elc_price < elcaim * 98 / 100);

            //调用swap，卖出ELP，买入ELC
            if(self.exchange_contract == (&0)) {
                let to_token = Default::default();
                self.exchange_contract = self.factory_contract.get_exchange(elc_contract, to_token).unwrap_or(&0);
                assert!((self.exchange_contract) != (&0));
            }

            let exchange_info: ExchangeInfo = self.exchange_contract.exchange_info();
            let token_decimals = exchange_info.to_decimals;
            let base: u128 = 10;
            let adj_num = self.contract_adj_num;
            let adj_bignum = adj_num * (base.pow(token_decimals));

            let send_amount: Balance = self.env().transferred_balance();
            ///判断符合小量交易
            assert!((send_amount > 0) && (send_amount <= adj_bignum));


            let block_time:u128 = self.env().block_timestamp().into();

            ///风险储备足够
            if(send_amount <= self.risk_reserve){
                let sold_amount = self.exchange_contract.swap_dot_to_token_input();
                assert!(sold_amount);
                let buy_amount = self.exchange_contract.swap_dot_to_token_output(sold_amount);
                assert!(buy_amount);

                self.risk_reserve -= send_amount;
                let gap: u128 = block_time - self.last_contract_time;
                self.last_contract_time = block_time;
                self.contract_gap.push((gap, send_amount));
                self.env().emit_event(ContractEvent {
                    gaptime: gap,
                    elp_amount: send_amount,
                }
            } else {
                ///一天内限制使用储备的2%
                if(self.start_tp > block_time - 86400000) {
                    assert!((self.day_used_reserve + send_amount - self.risk_reserve) <= (reserve * 2 / 100));
                    self.start_tp = block_time;
                    self.day_used_reserve += send_amount - self.risk_reserve;

                } else {
                    let sold_amount = self.exchange_contract.swap_dot_to_token_input();
                    assert!(sold_amount);
                    let buy_amount = self.exchange_contract.swap_dot_to_token_output(sold_amount);
                    assert!(buy_amount);

                    self.reserve -= send_amount - self.risk_reserve;
                    self.risk_reserve = 0;
                    self.start_tp = block_time;
                    let gap: u128 = block_time - self.last_contract_time;
                    self.last_contract_time = block_time;
                    self.contract_gap.push((gap, send_amount));
                    self.env().emit_event(ContractEvent {
                    gaptime: gap,
                    elp_amount: send_amount,

                }
                assert!((self.day_used_reserve + send_amount) < reserve * 2 / 100)
            }
        }

        ///设置扩张操作时每次小量交易的数值
        #[ink(message)]
        pub fn update_expand_adj(&mut self, expand_adj_num: u128) {
            assert!(expand_adj_num > 0);
            self.expand_adj_num = expand_adj_num;
        }

        ///设置收缩操作时每次小量交易的数值
        #[ink(message)]
        pub fn update_contract_adj(&mut self, contract_adj_num: u128) {
            assert!(contract_adj_num > 0);
            self.contract_adj_num = contract_adj_num;
        }

        ///计算通胀因子，如果通胀因子变动要更新, 出块速度为6秒/块，每隔10000个块将ELC目标价格调升K
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

        /// 返回系统负债率，调用时需要实时计算, 返回整数，以100为基数
        #[ink(message)]
        pub fn liability_ratio(&self) -> u128 {
            let elp_price: u128 = self.oracle_contract.elp_price();
            let elc_price: u128 = self.oracle_contract.elc_price();
            let elp_amount: Balance = self.reserve;
            let elc_amount: Balance = self.elc_contract.total_supply();
            let lr =  elc_amount * elc_price/(elp_price * elp_amount) * 100; //100为精度
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
