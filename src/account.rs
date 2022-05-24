use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Deserialize, Serialize)]
#[serde(crate="near_sdk::serde")]
pub enum Membership {
    Basic,
    Standard,
    Companion
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub stake_balance: Balance,                 
    pub pre_reward: Balance,                    
    pub last_block_balance_change: BlockHeight, 
    pub unstake_balance: Balance,               
    pub unstake_start_time: Timestamp,          
    pub unstake_available_epoch: EpochHeight,   
    pub membership: Membership,                 // Upgraded field
}

#[derive(Deserialize, Serialize)]
#[serde(crate="near_sdk::serde")]
pub struct AccountJson {
    pub account_id: AccountId,
    pub stake_balance: Balance,
    pub unstake_balance: Balance,
    pub reward: Balance,
    pub can_withdraw: bool,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_available_epoch: EpochHeight,
    pub current_epoch: EpochHeight,
    pub membership: Membership,                 // Upgraded field
}

// To cast from Account to Json
impl AccountJson {
    pub fn from(account_id: AccountId, new_reward: Balance, account: Account) -> Self {
        AccountJson { 
            account_id,
            stake_balance: account.stake_balance,
            unstake_balance: account.unstake_balance,
            reward: account.pre_reward + new_reward,
            can_withdraw: account.unstake_available_epoch <= env::epoch_height(),
            unstake_start_timestamp: account.unstake_start_time,
            unstake_available_epoch: account.unstake_available_epoch,
            current_epoch: env::epoch_height(),
            membership: account.membership,
        }
    }
}