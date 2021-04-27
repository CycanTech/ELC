#![cfg_attr(not(feature = "std"), no_std)]
pub use self::oracle::Oracle;
use ink_lang as ink;

#[ink::contract]
mod oracle {
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Oracle {
        /// Stores a single `bool` value on the storage.
        elp_price: u128,  //价格乘以100，避免小数
        elc_price: u128,
        block_timestamp_last: u128,
        owner: AccountId,
    }

    impl Oracle {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                elp_price: 0,
                elc_price: 0,
                block_timestamp_last: 0,
                owner: caller,
            }
        }

        /// 每小时更新一次价格，精度100
        #[ink(message)]
        pub fn update(&mut self, elp_price: u128, elc_price: u128) -> bool {
            self.only_owner();
            self.elp_price = elp_price;
            self.elc_price =  elc_price;
            self.block_timestamp_last = 0;
            true
        }

        #[ink(message)]
        pub fn elp_price(&self) -> u128 { self.elp_price }

        #[ink(message)]
        pub fn elc_price(&self) -> u128 { self.elc_price }

        fn only_owner(&self) {
            assert_eq!(self.env().caller(), self.owner);
        }

        /// Contract owner.
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        /// transfer contract ownership to new owner.
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            self.only_owner();
            self.owner = new_owner;
        }
    }
}
