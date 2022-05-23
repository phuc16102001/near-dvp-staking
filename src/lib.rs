use near_sdk::*;
use near_sdk::borsh::{self,BorshDeserialize,BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};

use crate::config::*;
use crate::account::*;
use crate::types::*;
use crate::utils::*;

mod config;
mod account;
mod types;
mod utils;
mod internal;

// Using `near_bindgen` marco, to notify the smart contract
// BorshSerde to serde as byte code (for storing on-chain)
// Serde to serde as json (for query and display on front-end)
// PanicOnDefault to prevent the smart contract init by itself

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct StakingContract {
    pub owner_id: AccountId,                        // ID of contract owner
    pub ft_contract_id: AccountId,                  // ID of fungible token contract
    pub config: Config,                             // Config incentive rule
    pub total_stake: Balance,                       // Total stake balance
    pub total_paid_reward: Balance,                 // 
    pub num_staker: u64,                            // The number of stakers
    pub pre_reward: Balance,                        // 
    pub last_block_balance_change: BlockHeight,     // Block height when balance updated
    pub accounts: LookupMap<AccountId, Account>,    // Account informations respected to ID  
    pub paused: bool,                               // Staking will be paused when there is no more tokens
}

#[near_bindgen]
impl StakingContract {

    // This macro define the function to init the contract
    // If using PanicOnDefault but without init macro, the smart contract won't be able to initialize
    #[init]
    pub fn new_default_config(
        owner_id: AccountId,
        ft_contract_id: AccountId,
    ) -> Self {
        Self::new(owner_id,ft_contract_id,Config::default())
    }

    #[init]
    pub fn new(
        owner_id: AccountId,
        ft_contract_id: AccountId,
        config: Config
    ) -> Self {
        StakingContract { 
            owner_id, 
            ft_contract_id, 
            config,
            total_stake: 0, 
            total_paid_reward: 0, 
            num_staker: 0, 
            pre_reward: 0, 
            last_block_balance_change: env::block_index(), 
            accounts: LookupMap::new(StorageKey::AccountKey),
            paused: false
        }
    }

    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        assert_at_least_one_yocto();
        
        let account = account_id.unwrap_or_else(|| env::predecessor_account_id());
        let account_stake = self.accounts.get(&account);
        if account_stake.is_some() {
            // Refund all tokens
            refund_deposit(0)
        } else {
            // Create new account
            let storage_usage_before = env::storage_usage();
            self.internal_create_account(account);
            let storage_usage_after = env::storage_usage();

            // Refund the rest tokens
            refund_deposit(storage_usage_after - storage_usage_before);
        }
    }

    pub fn exist_account(&self, account_id: AccountId) -> bool {
        self.accounts.get(&account_id).is_some()
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

// =========================== Unit Test =============================

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, MockedBlockchain};
    use near_sdk::test_utils::{VMContextBuilder, accounts};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();

        builder.current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false);
        testing_env!(context.build());

        let config: Config = Config {
            reward_num: 500,
            reward_denom: 100000,
        };

        let contract = StakingContract::new(
            accounts(1).to_string(), 
            "ft_contract".to_string(), 
            config
        );

        assert_eq!(contract.owner_id, accounts(1).to_string());
        assert_eq!(contract.ft_contract_id, "ft_contract".to_string());
        assert_eq!(config.reward_num, contract.config.reward_num);
        assert_eq!(contract.paused, false);
    }   
}