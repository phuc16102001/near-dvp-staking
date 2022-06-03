use crate::*;

// Need serde to init contract by json
#[derive(BorshDeserialize,BorshSerialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    pub reward_num: u32,        // Incentive numerator
    pub reward_denom: u64,      // Incentive denomerator
    pub num_epoch_unlock: u64,  // Number of epoch need to wait to withdraw when unstake
}

// With APR 15% --> reward = 0.15*token (per year)
// But we use the number of blocks, so APR 15%-18% is 
impl Default for Config {
    fn default() -> Self {
        Self { reward_num: 715, reward_denom: 10u64.pow(9), num_epoch_unlock: 1 }
    }
}