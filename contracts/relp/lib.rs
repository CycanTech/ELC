#![cfg_attr(not(feature = "std"), no_std)]

pub use self::relp::RELP;
use ink_lang as ink;

#[ink::contract]
mod relp {
    use elc::ELC;
    use ink_prelude::{string::String};
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_prelude::{vec, vec::Vec};
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };
    use ink_storage::{traits::{PackedLayout, SpreadLayout}};
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_env::call::FromAccountId;

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        InsufficientSupply,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
        OnlyOwnerAccess,
        InvalidNewOwner,
        InvalidAmount,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// struct that represents a transfer time log
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    struct Transferlog {
        amount: u128,
        timelog: u128,
    }

    #[ink(storage)]
    pub struct RELP {
        /// Total token supply.
        total_supply: Lazy<Balance>,
        /// Mapping from owner to number of owned token.
        balances: StorageHashMap<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
        /// Name of the token
        name: Option<String>,
        /// Symbol of the token
        symbol: Option<String>,
        /// Decimals of the token
        decimals: Option<u8>,
        /// The contract owner, provides basic authorization control
        /// functions, this simplifies the implementation of "user permissions".
        owner: AccountId,
        /// AccountId -> average hold time
        transferlogs: StorageHashMap<AccountId, Vec<Transferlog>>,
        /// record all relp holders
        holders: Vec<AccountId>,
        elc_contract: Lazy<ELC>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    impl RELP {
        #[ink(constructor)]
        pub fn new(
            elc_token: AccountId,
        ) -> Self {
            let caller = Self::env().caller();
            let name: Option<String> = Some(String::from("Risk Reserve of ELP"));
            let symbol: Option<String> = Some(String::from("rELP"));
            let decimals: Option<u8> = Some(8);
            let elc_contract: ELC = FromAccountId::from_account_id(elc_token);
            let instance = Self {
                total_supply: Lazy::new(0),
                balances: StorageHashMap::new(),
                allowances: StorageHashMap::new(),
                name,
                symbol,
                decimals,
                owner: caller,
                transferlogs: StorageHashMap::new(),
                holders: Vec::new(),
                elc_contract: Lazy::new(elc_contract),
            };
            instance
        }

        /// Returns the token name.
        #[ink(message)]
        pub fn token_name(&self) -> Option<String> {
            self.name.clone()
        }

        /// Returns the token symbol.
        #[ink(message)]
        pub fn token_symbol(&self) -> Option<String> {
            self.symbol.clone()
        }

        /// Returns the token decimals.
        #[ink(message)]
        pub fn token_decimals(&self) -> Option<u8> {
            self.decimals
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).copied().unwrap_or(0)
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set `0`.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).copied().unwrap_or(0)
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the the account balance of `from`.
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(from, to, value)?;
            self.allowances.insert((from, caller), allowance - value);
            Ok(())
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        /// Mint a new amount of tokens
        /// these tokens are deposited into the owner address
        #[ink(message)]
        pub fn mint(&mut self, user: AccountId, amount: Balance) -> Result<()> {
            self.only_owner();
            assert_ne!(user, Default::default());
            if amount <= 0 {
                return Err(Error::InvalidAmount);
            }

            let user_balance = self.balance_of(user);
            self.balances.insert(user, user_balance + amount);
            *self.total_supply += amount;
            self.env().emit_event(Mint { user, amount });

            //deal with holders
            self.update_holders(user, user_balance);

            //deal with transferlog
            self.push_holder_log(user, amount);
            Ok(())
        }

        /// Burn tokens.
        /// These tokens are withdrawn from the owner address
        /// if the balance must be enough to cover the redeem
        /// or the call will fail.
        #[ink(message)]
        pub fn burn(&mut self, user: AccountId, amount: Balance) -> Result<()> {
            self.only_owner();
            if *self.total_supply < amount {
                return Err(Error::InsufficientSupply);
            }
            let user_balance = self.balance_of(user);
            if user_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(user, user_balance - amount);
            *self.total_supply -= amount;
            self.env().emit_event(Burn { user, amount });

            //deal with holders
            self.update_holders(user, user_balance - amount);

            //deal with transferlog
            self.take_holder_log(user);
            Ok(())
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            let from_balance_after = from_balance - value;
            self.balances.insert(from, from_balance_after);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);

            self.update_holders(from, from_balance_after);
            self.update_holders(to, to_balance + value);
            self.take_holder_log(from);
            self.push_holder_log(to, value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())
        }

        fn update_holders(&mut self, from: AccountId, balance: Balance) {
            if balance == 0 {
                //  delete
                let index = self.holders.iter().position(|x| *x == from).unwrap();
                self.holders.remove(index);
            } else {
                // push
                self.holders.push(from);
            }
        }

        ///if user is transfer sender, remove previous transferlog, then update
        fn take_holder_log(&mut self, from: AccountId) {
            let now_time: u128 = self.env().block_timestamp().into();
            let logs = self.transferlogs.get(&from).unwrap();
            let mut holdtime: u128 = 0;
            let mut hold_realtime: u128 = 0;
            for log in logs.iter() {
                holdtime = holdtime.saturating_add(log.amount.saturating_mul(now_time.saturating_sub(log.timelog)));
                hold_realtime = hold_realtime.saturating_add(now_time.saturating_sub(log.timelog));
            }
            let average_holder_time = holdtime / hold_realtime;
            let mut transferlog_from = Transferlog {
                amount: self.balances.get(&from).copied().unwrap_or(0),
                timelog: average_holder_time,
            };

            if let Some(from_log) = self.transferlogs.get_mut(&from) {
                self.transferlogs.take(&from);
                self.transferlogs.insert(from, vec![transferlog_from]);
            }
        }

        ///if user is transfer receiver, push new transferlog
        fn push_holder_log(&mut self, to: AccountId, value: Balance) {
            let now_time: u128 = self.env().block_timestamp().into();
            let mut transferlog_to = Transferlog {
                amount: value,
                timelog: now_time,
            };
            if let Some(to_log) = self.transferlogs.get_mut(&to) {
                to_log.push(transferlog_to);
            } else {
                self.transferlogs.insert(to, vec![transferlog_to]);
            }
        }

        #[ink(message)]
        pub fn update_hold_time_for_reward(&mut self, from: AccountId, value: Balance, now_time: u128)  {
            self.only_owner(); //owner only
            let mut transferlog_from = Transferlog {
                amount: self.balances.get(&from).copied().unwrap_or(0),
                timelog: now_time,
            };
            if let Some(from_log) = self.transferlogs.get_mut(&from) {
                self.transferlogs.take(&from);
                self.transferlogs.insert(from, vec![transferlog_from]);
            }
        }

        #[ink(message)]
        pub fn hold_time(&self, user: AccountId, now_time: u128) -> (u128, u128) {
            let mut holdtime: u128 = 0;
            let mut hold_realtime: u128 = 0;
            let logs = self.transferlogs.get(&user).unwrap();
            for log in logs.iter() {
                holdtime = holdtime.saturating_add(log.amount.saturating_mul(now_time.saturating_sub(log.timelog)));
                hold_realtime = hold_realtime.saturating_add(now_time.saturating_sub(log.timelog));
            }
            (holdtime, hold_realtime)
        }

        #[ink(message)]
        pub fn hold_time_all(&self, now_time: u128) -> u128 {
            let mut holdtime: u128 = 0;
            for holder in self.holders.iter() {
                let logs = self.transferlogs.get(&holder).unwrap();
                for log in logs.iter() {
                    holdtime = holdtime.saturating_add(log.amount.saturating_mul(now_time.saturating_sub(log.timelog)));
                }
            }
            holdtime
        }

        #[ink(message)]
        pub fn mint_to_holders(&mut self, expand_amount:u128) -> Result<()>  {
            self.only_owner();
            let total_supply = self.total_supply();
            for holder in self.holders.iter() {
                let balance = self.balance_of(*holder);
                let mint_amount = expand_amount.saturating_mul(balance) / total_supply;
                if mint_amount > 0 {
                    assert!(self.elc_contract.transfer(*holder, mint_amount).is_ok());
                }
            }
            Ok(())
        }

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

