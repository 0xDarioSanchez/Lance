use soroban_sdk::contracttype;

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum ServiceStatus {
    CREATED,
    ACCEPTED, 
    WAITING,
    DISPUTING,
    COMPLETED,
}