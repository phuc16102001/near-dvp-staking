use crate::*;

pub trait FungibleTokenReceiver {
    // When receive tokens from user through FT contract
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for StakingContract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        env::log(format!("User {} staking {} NEAR with message \"{}\"", sender_id, amount.0, msg).as_bytes());
        self.internal_deposit_and_stake(sender_id, amount.0);

        PromiseOrValue::Value(U128(0))
    }
}