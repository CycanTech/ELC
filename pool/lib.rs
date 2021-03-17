#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pool::Pool;
use ink_lang as ink;

#[ink::contract]
mod pool {
    #[ink(storage)]
    pub struct Pool {
        aimprice: Option<u8>,
        inflation: Option<u8>,
        reserve: Balance,
        risk_reserve: Balance,
    }

    impl Pool {
        #[ink(constructor)]
        pub fn new(reserve: Balance, risk_reserve: Balance) -> Self {
            let aim_price: Option<u8> = Some(1);
            let inflation: Option<u8> = Some(5); //0.00005
            let instance = Self {
                aimprice: aim_price,
                inflation: inflation,
                reserve: reserve,
                risk_reserve: risk_reserve,
            };
            instance
        }

        /// 增加流动性(ELP)，返回rELP和ELC
        #[ink(message, payable)]
        pub fn add_liquidity(&mut self, from_tokens: Balance) -> (Balance, Balance) {
            let caller = self.env().caller();
            let LR = self.liability_ratio();
            if LR > 30 {
                //返回用户rELP和0
            } else {
                //返回用户ELC和rELP数量
            }
        }

        /// 退出流动性，发送ELP给用户
        #[ink(message)]
        pub fn remove_liquidity(&mut self, rELP_amount: Balance) -> (Balance) {
            assert!(rELP_amount > 0);
            //返回ELP数量
        }

        /// 单独领取奖励
        #[ink(message)]
        pub fn get_reward(&mut self, rELP_amount: Balance) -> (Balance) {
            assert!(rELP_amount > 0);
            //返回ELP数量
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
