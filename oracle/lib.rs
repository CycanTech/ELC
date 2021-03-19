#![cfg_attr(not(feature = "std"), no_std)]
pub use self::oracle::Oracle;
use ink_lang as ink;

#[ink::contract]
mod oracle {
    use ownership::Ownable;
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

    impl Ownable for Oracle {
        #[ink(constructor)]
        fn new() -> Self {
            unimplemented!()
        }

        /// Contract owner.
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            Some(self.owner)
        }

        /// transfer contract ownership to new owner.
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: Option<AccountId>) {
            self.only_owner();
            if let Some(owner) = new_owner {
                self.owner = owner;
            }
        }
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
        pub fn update(&mut self, elp_price: u128, elc_price: u128)  {
            self.only_owner();
            self.elp_price = elp_price;
            self.elc_price =  elc_price;
            self.block_timestamp_last = 0;
        }

        #[ink(message)]
        pub fn elp_price(&self) -> u128 { self.elp_price }

        #[ink(message)]
        pub fn elc_price(&self) -> u128 { self.elc_price }

        fn only_owner(&self) {
            assert_eq!(self.env().caller(), self.owner);
        }

    }
}
