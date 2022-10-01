#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct ERC20 {
        // name: String,
        // decimal: u8,
        // symbol: String,
        total_supply: Balance, // Balance 是 ink-env 提供的类型
        balances: Mapping<AccountId, Balance>,
        approval: Mapping<(AccountId, AccountId), Balance>, // 定义授权相关的存储
    }

    #[ink(event)]
    pub struct Transfer {
        // topic 方便索引
        #[ink(topic)]
        from: AccountId,
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        owner: AccountId,
        spender: AccountId,
        value: Balance,
    }

    // define pub enum Error()
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    impl ERC20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();

            // transfer total_supply to msg sender
            // self.env()
            // Self::env()
            let sender = Self::env().caller();
            balances.insert(&sender, &total_supply);
            // 打印节点日志
            ink::env::debug_println!(
                "Balance in constructor, Account: {:?} | Balance: {:?}",
                &sender,
                balances.get(&sender)
            );

            // emit event
            Self::env().emit_event(Transfer {
                from: AccountId::default(),
                to: sender,
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                approval: Default::default(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            // 是一个 Mapping 结构，结果是一个 Option
            let balance = self.balances.get(&who);
            ink::env::debug_println!("Balance_of Account: {:?} | {:?}", &who, &balance);
            balance.unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(
            &mut self,
            to: AccountId,
            value: Balance,
        ) -> core::result::Result<(), Error> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            // sub
            self.balances.insert(&from, &(from_balance - value));

            // add
            let to_balance = self.balance_of(to);
            self.balances.insert(&to, &(to_balance + value));

            // emit event
            self.env().emit_event(Transfer { from, to, value });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> core::result::Result<(), Error> {
            let from_balance = self.balance_of(from);
            // make sure sufficient
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            // sub
            self.balances.insert(from, &(from_balance - value));

            // add
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));

            // emit event
            self.env().emit_event(Transfer { from, to, value });

            Ok(())
        }

        #[ink(message)]
        pub fn approve(
            &mut self,
            spender: AccountId,
            value: Balance,
        ) -> core::result::Result<(), Error> {
            let from = self.env().caller();
            self.approval.insert((from, spender), &value);
            Ok(())
        }

        #[ink(message)]
        pub fn allowance(&self, spender: AccountId) -> Balance {
            let from = self.env().caller();
            self.approval.get(&(from, spender)).unwrap()
        }
    }
}
