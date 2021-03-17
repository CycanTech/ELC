#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pool::Pool;
use ink_lang as ink;

#[ink::contract]
mod pool {
    #[ink(storage)]
    pub struct Pool {
        elcaim: u8,
        k: u8, //inflation factor
        reserve: Balance,
        risk_reserve: Balance,
        oracle: AccountId,
        k_update_time: u64,
    }

    impl Pool {
        #[ink(constructor)]
        pub fn new(reserve: Balance, risk_reserve: Balance, oracle_addr: AccountId) -> Self {
            let instance = Self {
                elcaim: 1,
                k: 5, //0.00005 * 100000
                reserve: reserve,
                risk_reserve: risk_reserve,
                oracle: oracle_addr,
                k_update_time: Self::env().block_timestamp(),
            };
            instance
        }

        /// 增加流动性(ELP)，返回rELP和ELC
        #[ink(message, payable)]
        pub fn add_liquidity(&mut self, from_tokens: Balance) -> (Balance, Balance) {
            //首先更新ELCaim价格
            self.update_ELCaim();
            
            let LR = self.liability_ratio();
            if LR > 30 {
                //返回用户rELP和0
            } else {
                //返回用户ELC和rELP数量
            }
            (from_tokens, from_tokens)
        }

        /// 退出流动性，发送ELP给用户
        #[ink(message)]
        pub fn remove_liquidity(&mut self, rELP_amount: Balance) -> (Balance) {
            assert!(rELP_amount > 0);


            //返回ELP数量
            rELP_amount
        }

        /// 单独领取奖励
        #[ink(message)]
        pub fn get_reward(&mut self, rELP_amount: Balance) -> (Balance) {
            assert!(rELP_amount > 0);
            //返回ELP数量
            rELP_amount
        }

        //计算通胀因子，如果通胀因子变动要更新, 出块速度为6秒/块，每隔10000个块将ELC目标价格调升K
        #[ink(message)]
        pub fn update_ELCaim(&self) {
            let block_time = self.env().block_timestamp();
            let elcaim = self.elcaim;
            let last_update_time = self.k_update_time;
            let k = (block_time - self.k_update_time) / 6 / 10000;
            if k > 0 {
                *self.elcaim = elcaim * (100000 + k) / 100000;
                *self.k_update_time = last_update_time + (k * 10000 * 6);
            }
        }

        /// 返回系统负债率，调用时需要实时计算, 返回整数，以100为基数
        #[ink(message)]
        pub fn liability_ratio(&self) -> u8 {
            0
        }

        #[ink(message)]
        pub fn elp_reserve(&self) -> Balance { self.reserve }

        #[ink(message)]
        pub fn elp_risk_reserve(&self) -> Balance { self.risk_reserve }
    }
}
