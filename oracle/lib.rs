#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod oracle {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Oracle {
        /// Stores a single `bool` value on the storage.
        elp_price: u8,  //价格乘以100，避免小数
        elc_price: u8,
        block_timestamp_last: u8,
    }

    impl Oracle {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                elp_price: Some(0),
                elc_price: Some(0),
                block_timestamp_last:
            }
        }

        /// 每小时更新一次价格
        #[ink(message)]
        pub fn update(&mut self, elp_price: Option<u8>, elc_price: Option<u8>) -> Option<u8> {
            Self {
                elp_price: elp_price,
                elc_price: elc_price,
                block_timestamp_last:
            }
        }

        #[ink(message)]
        pub fn elp_price(&self) -> Option<u8> { self.elp_price }

        #[ink(message)]
        pub fn elc_price(&self) -> Option<u8> { self.elc_price }
    }
}
