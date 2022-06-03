use crate::*;

// Using pub(crate) to only callable from inside the program instead of outside by command

#[near_bindgen]
impl StakingContract {
    pub(crate) fn internal_unstake(&mut self, account_id: AccountId, amount: Balance) {
        let upgradable_account = self.accounts.get(&account_id).unwrap();
        let mut account = Account::from(upgradable_account);
        assert!(amount <= account.stake_balance, "Cannot unstake more than the staking amount");

        // Update account reward
        let new_reward = self.internal_calculate_new_reward(Some(&account));
        account.pre_reward += new_reward;
        account.last_block_balance_change = env::block_index();
    
        // Update account unstake
        account.stake_balance -= amount;
        account.unstake_balance += amount;
        account.unstake_start_time = env::block_timestamp();
        account.unstake_available_epoch = env::epoch_height() + self.config.num_epoch_unlock;
    
        if account.stake_balance==0 {
            self.num_staker -= 1;
        }    

        let new_contract_reward = self.internal_calculate_new_reward(None);
        self.pre_reward += new_contract_reward;
        self.total_stake -= amount;
        self.last_block_balance_change = env::block_index();
    }

    pub(crate) fn internal_deposit_and_stake(&mut self, sender_id: AccountId, amount: Balance) {
        let upgradable_account = self.accounts.get(&sender_id);
        assert!(upgradable_account.is_some(), "Account not found, please registry first");
        assert_eq!(self.is_paused(), false, "Contract is paused");
        assert_eq!(self.ft_contract_id, env::predecessor_account_id(), "Only accept the correct fungible token");

        // Update account
        let mut account = Account::from(upgradable_account.unwrap());
        let is_new_staker = (account.stake_balance==0);
        let new_reward = self.internal_calculate_new_reward(Some(&account));

        account.pre_reward += new_reward;                           // Update pre_reward to become the new phase of staking
        account.stake_balance += amount;                            // Staking the deposit amount
        account.last_block_balance_change = env::block_index();     // Update current block
        self.accounts.insert(&sender_id, &UpgradableAccount::from(account));

        // Update pool
        let new_global_reward = self.internal_calculate_new_reward(None);
        self.pre_reward += new_global_reward;
        self.total_stake += amount;
        self.last_block_balance_change = env::block_index();
        
        if is_new_staker {
            self.num_staker += 1;
        }
    }

    pub(crate) fn internal_create_account(&mut self, account_id: AccountId) {
        let account = Account {
            stake_balance: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            unstake_balance: 0,
            unstake_start_time: 0,
            unstake_available_epoch: 0,
            membership: Membership::Basic,
        };
        self.accounts.insert(
            &account_id, 
            &UpgradableAccount::from(account)
        );
    }

    // Calculate the reward for users from the previous changes
    // Generally, we treat that users have deposited twice
    // ==============================================
    // | time  |     t1     |      t2     |   now   |
    // | money |    10$     |      20$    |         |
    // ==============================================
    // t2 = last_block_balance_change
    // now = block_height
    // reward = (now-t2)*stake*rate
    //
    // If account=None, the return is for global
    pub(crate) fn internal_calculate_new_reward(&self, account: Option<&Account>) -> Balance {
        let last_block = if self.paused {
            self.paused_block
        }  else {
            env::block_index()
        };

        let last_change = if account.is_some() {
            account.unwrap().last_block_balance_change
        } else {
            self.last_block_balance_change
        };

        let stake = if account.is_some() {
            account.unwrap().stake_balance
        } else {
            self.total_stake
        };

        let cnt_block = last_block - last_change;
        let config = self.config;
        let reward = ((config.reward_num as Balance) * (stake as Balance) * (cnt_block as Balance)) / (config.reward_denom as Balance);
        reward
    }
}