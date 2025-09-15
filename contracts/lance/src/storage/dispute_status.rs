use soroban_sdk::contracttype;

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum DisputaStat {
    OPEN,
    VOTING, 
    EXECUTED,
    FINISHED,
}