use crate::*;

// Using pub(crate) to only callable from inside the program instead of outside by command

#[near_bindgen]
impl StakingContract {
    pub(crate) fn internal_create_account(&mut self, account_id: AccountId) {
        let account = Account {
            stake_balance: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            unstake_balance: 0,
            unstake_start_time: 0,
            unstake_available_epoch: 0,
        };
        self.accounts.insert(&account_id, &account);
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
        let rate = (config.reward_num as u64) / config.reward_denom;
        let reward = stake * (cnt_block as Balance) * (rate as Balance);
        reward
    }
}