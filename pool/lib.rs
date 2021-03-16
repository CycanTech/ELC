#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod pool {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_env::call::FromAccountId;
    use ink_prelude::string::String;
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::Lazy;

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct PoolState {
        pub from_symbol: String,
        pub from_decimals: u8,
        pub to_symbol: String,
        pub to_decimals: u8,
        pub from_token_pool: Balance,
        pub to_token_pool: Balance,
        pub lp_token_supply: Balance,
        pub own_lp_token: Balance,
    }

    #[ink(storage)]
    pub struct ELCaim {
        aimprice: Option<u8>,
        inflationfactor: Option<u8>,
    }

    #[ink(event)]
    pub struct AddLiquidity {
        #[ink(topic)]
        sender: AccountId,
        #[ink(topic)]
        from_amount: Balance,
        #[ink(topic)]
        to_amount: Balance,
    }

    #[ink(event)]
    pub struct RemoveLiquidity {
        #[ink(topic)]
        sender: AccountId,
        #[ink(topic)]
        from_amount: Balance,
        #[ink(topic)]
        to_amount: Balance,
    }

    impl Pool {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
