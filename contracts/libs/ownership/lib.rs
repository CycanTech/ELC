#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
use ink_lang as ink;

#[ink::trait_definition]
pub trait Ownable {
    /// Initializes the contract setting the deployer as the initial owner.
    #[ink(constructor)]
    fn new() -> Self;

    /// Returns the account id of the current owner.
    #[ink(message)]
    fn owner(&self) -> Option<AccountId>;

    /// Transfer ownership to new owner.
    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: Option<AccountId>);
}
