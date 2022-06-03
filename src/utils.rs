use crate::*;

pub fn assert_at_least_one_yocto() {
    assert!(env::attached_deposit() >= 1, "Require at least 1 yoctoNEAR");
}

pub fn refund_deposit(storage_used: StorageUsage) {
    // NEAR cost to use `storage_used` bytes
    let cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    assert!(
        attached_deposit >= cost,
        "Must attach at least {} yoctoNEAR to cover storage",
        cost
    );

    let refund = attached_deposit - cost;
    if refund>0 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}