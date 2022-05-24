use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum UpgradableAccount {
    Version1(AccountV1),
    Current(Account)
}

impl From<Account> for UpgradableAccount {
    fn from(account: Account) -> Self {
        UpgradableAccount::Current(account)
    }
}

impl From<UpgradableAccount> for Account {
    fn from(upgradable_account: UpgradableAccount) -> Self {
        match upgradable_account {
            UpgradableAccount::Current(account) => account,
            UpgradableAccount::Version1(account) => {
                Account {
                    stake_balance: account.stake_balance,
                    pre_reward: account.stake_balance,
                    last_block_balance_change: account.last_block_balance_change,
                    unstake_balance: account.unstake_balance,
                    unstake_start_time: account.unstake_start_time,
                    unstake_available_epoch: account.unstake_available_epoch,
                    membership: Membership::Basic
                }
            },
        }
    }
}