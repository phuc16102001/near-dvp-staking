use crate::*;

pub const FT_TRANSFER_GAS: Gas = 10_000_000_000_000;
pub const HARVEST_CALLBACK_GAS: Gas = 10_000_000_000_000;
pub trait FungibleTokenReceiver {
    // When receive tokens from user through FT contract
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

#[ext_contract(ext_ft)]
pub trait FungibleTokenCore {
    // To transfer token when user harvest
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_self)]
pub trait ExtStakingContract {
    // For callback after harvest successfully
    fn ft_harvest_callback(&mut self, account_id: AccountId, amount: U128);
}

#[near_bindgen]
impl FungibleTokenReceiver for StakingContract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        env::log(format!("User {} staking {} NEAR with message \"{}\"", sender_id, amount.0, msg).as_bytes());
        self.internal_deposit_and_stake(sender_id, amount.0);

        PromiseOrValue::Value(U128(0))
    }
}

#[near_bindgen]
impl StakingContract {

    #[private]
    pub fn ft_harvest_callback(&mut self, account_id: AccountId, amount: U128) -> U128 {
        assert_eq!(env::promise_results_count(), 1, "Too many result of promise");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),          // Will not be handle
            PromiseResult::Successful(_value) => {
                let upgradable_account = self.accounts.get(&account_id).unwrap();
                let mut account = Account::from(upgradable_account);

                account.pre_reward = 0;
                account.last_block_balance_change = env::block_index();

                self.accounts.insert(&account_id, &UpgradableAccount::from(account));
                self.total_paid_reward += amount.0;

                amount
            },
            PromiseResult::Failed => env::panic("Callback failed".as_bytes()),
        }
    }

    #[payable]
    pub fn harvest(&mut self) -> Promise {
        assert_one_yocto();
        let account_id: AccountId = env::predecessor_account_id();
        let upgradable_account = self.accounts.get(&account_id).unwrap();
        let account = Account::from(upgradable_account);

        let new_reward = self.internal_calculate_new_reward(Some(&account));
        let current_reward = self.pre_reward + new_reward;
        assert!(current_reward > 0, "Your reward is zero");

        ext_ft::ft_transfer(
            account_id.clone(), 
            U128(current_reward), 
            Some("Harvest reward from staking".to_string()), 
            &self.ft_contract_id, 
            1, 
            FT_TRANSFER_GAS
        ).then(ext_self::ft_harvest_callback(
            account_id.clone(), 
            U128(current_reward), 
            &env::current_account_id(), 
            0, 
            HARVEST_CALLBACK_GAS
        ))
    }
}