use crate::*;

#[derive(Deserialize, Serialize)]
#[serde(crate="near_sdk::serde")]
pub struct PoolJson {
    pub total_stake_balance: U128,
    pub total_reward: U128,
    pub total_staker: U128,
    pub is_paused: bool
}

impl PoolJson {
    pub fn from(pool: &StakingContract) -> Self {
        PoolJson {
            total_stake_balance: U128(pool.total_stake),
            total_reward: U128(pool.pre_reward + pool.internal_calculate_new_reward(None)),
            total_staker: U128(pool.num_staker),
            is_paused: pool.paused,
        }
    }
}