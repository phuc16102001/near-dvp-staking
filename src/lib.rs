use near_sdk::*;
use near_sdk::borsh::{self,BorshDeserialize,BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};
// use ::constant::{ONE_YOCTO};

use crate::config::*;
use crate::account::*;
use crate::types::*;
use crate::utils::*;
use crate::enumeration::*;
use crate::pool::*;
use crate::staking_contract_v1::*;
use crate::account_v1::*;
use crate::upgradable_account::*;
use crate::core_impl::*;

mod config;
mod account;
mod types;
mod utils;
mod internal;
mod enumeration;
mod pool;
mod staking_contract_v1;
mod account_v1;
mod upgradable_account;
mod core_impl;

// Using `near_bindgen` marco, to notify the smart contract
// BorshSerde to serde as byte code (for storing on-chain)
// Serde to serde as json (for query and display on front-end)
// PanicOnDefault to prevent the smart contract init by itself
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct StakingContract {
    pub owner_id: AccountId,                        
    pub ft_contract_id: AccountId,                  
    pub config: Config,                             
    pub total_stake: Balance,                       
    pub total_paid_reward: Balance,                 
    pub num_staker: u128,                            
    pub pre_reward: Balance,                        
    pub last_block_balance_change: BlockHeight,     
    pub accounts: LookupMap<AccountId, UpgradableAccount>,    
    pub paused: bool,                               
    pub paused_block: BlockHeight,                  
    pub version: u128,                               // New field to update (V2)
}

#[near_bindgen]
impl StakingContract {

    // This macro define the function to init the contract
    // If using PanicOnDefault but without init macro, the smart contract won't be able to initialize
    // Also, you can use --initFunction to determine which function to be called
    // And --initArgs flag is to pass arguments as JSON
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
            paused: false,
            paused_block: 0,
            version: 2,
        }
    }

    // Storing data on-chain require a small amount of NEAR (since using storage)
    // Instead of using ourself money, we make the user to deposit them
    // This is the `reserved_near` in wallet
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

    pub fn get_version(&self) -> U128 {
        U128(self.version)
    }

    // This is to upgrade the staking contract from v1 to v2
    // Use the private macro to avoid others people calling it (only the contract can call)
    // To migrate, use the command `near dev-deploy path --initFunction migrate --initArgs '{}'`
    //
    // This function MUST BE REMOVED after migrating to be secured
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_contract: StakingContractV1 = env::state_read().expect("Cannot read old contract");
        StakingContract { 
            owner_id: old_contract.owner_id,
            ft_contract_id: old_contract.ft_contract_id,
            config: old_contract.config,
            total_stake: old_contract.total_stake,
            total_paid_reward: old_contract.total_paid_reward,
            num_staker: old_contract.num_staker,
            pre_reward: old_contract.pre_reward,
            last_block_balance_change: old_contract.last_block_balance_change,
            accounts: old_contract.accounts, 
            paused: old_contract.paused,
            paused_block: old_contract.paused_block,
            version: 2
        }
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