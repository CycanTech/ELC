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

        #[ink(message)]
        pub fn liability_ratio(&self) -> u8 {
            0
        }

        #[ink(message)]
        pub fn elp_reserve(&self) -> Balance {
            self.env().balance().saturating_sub(self.reserve)
        }

        #[ink(message)]
        pub fn elp_risk_reserve(&self) -> Balance {
            self.env().balance().saturating_sub(self.risk_reserve)
        }
    }
}
