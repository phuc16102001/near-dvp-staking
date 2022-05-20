use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub stake_balance: Balance,                 // Staked tokens
    pub pre_reward: Balance,                    //
    pub last_block: BlockHeight,                // Last block when balance changed
    pub unstake_balance: Balance,               //
    pub unstake_start_time: Timestamp,          // Start time when user begin unstaking
    pub unstake_available_epoch: EpochHeight,   // 1 Epoch ~ 12 hours
}