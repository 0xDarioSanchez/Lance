use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct Vote {
    account: Address,
    vote: bool,
}