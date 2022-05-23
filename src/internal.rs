use crate::*;

// Using pub(crate) to only callable from inside the program instead of outside by command

#[near_bindgen]
impl StakingContract {
    pub(crate) fn internal_create_account(&mut self, account_id: AccountId) {
        let account = Account {
            stake_balance: 0,
            pre_reward: 0,
            last_block: env::block_index(),
            unstake_balance: 0,
            unstake_start_time: 0,
            unstake_available_epoch: 0,
        };
        self.accounts.insert(&account_id, &account);
    }
}