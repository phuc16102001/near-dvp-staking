use crate::*;

#[near_bindgen]
impl StakingContract {
    pub fn get_account_info(&self, account_id: &AccountId) -> AccountJson {
        let account = self.accounts.get(account_id).unwrap();
        let new_reward = self.internal_calculate_new_reward(Some(&account));
        AccountJson::from(
            account_id.clone(),
            new_reward,
            account
        )
    }

    pub fn get_pool_info(&self) -> PoolJson {
        PoolJson::from(&self)
    }
}