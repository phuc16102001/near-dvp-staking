use crate::*;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StakingContractV1 {
    pub owner_id: AccountId,                                    // ID of contract owner
    pub ft_contract_id: AccountId,                              // ID of fungible token contract
    pub config: Config,                                         // Config incentive rule
    pub total_stake: Balance,                                   // Total stake balance
    pub total_paid_reward: Balance,                             // 
    pub num_staker: u128,                                       // The number of stakers
    pub pre_reward: Balance,                                    // 
    pub last_block_balance_change: BlockHeight,                 // Block height when balance updated
    pub accounts: LookupMap<AccountId, UpgradableAccount>,      // Account informations respected to ID  
    pub paused: bool,                                           // Staking will be paused when there is no more tokens
    pub paused_block: BlockHeight,                              // Block height when contract paused  
}