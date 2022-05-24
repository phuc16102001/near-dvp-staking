use crate::*;

#[derive(Deserialize, Serialize)]
#[serde(crate="near_sdk::serde")]
pub struct PoolJson {
    pub total_stake_balance: Balance,
    pub total_reward: Balance,
    pub total_staker: u64,
    pub is_paused: bool
}

impl PoolJson {
    pub fn from(pool: &StakingContract) -> Self {
        PoolJson {
            total_stake_balance: pool.total_stake,
            total_reward: pool.pre_reward + pool.internal_calculate_new_reward(None),
            total_staker: pool.num_staker,
            is_paused: pool.paused,
        }
    }
}