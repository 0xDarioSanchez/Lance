#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::contract::{Contract, ContractClient};

#[test]
fn test_add_and_new_service() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = Contract;
    let admin = Address::generate(&env);
    let token = Address::generate(&env);

    // Register contract AND initialize it in one step
    let contract_id = env.register(contract, (admin, token));
    let client = ContractClient::new(&env, &contract_id);

    let emploee_1 = Address::generate(&env);
    let employer_1 = Address::generate(&env);
    let service_id_1: u32 = 1;
    let one_day_duration: u64 = 1; // days
    let milestone_payment: i128 = 1000;

    // Now you can call other contract methods
    let service_metadata = String::from_str(&env, "Service 1");

    client.create_service(
        &emploee_1,
        &employer_1,
        &service_id_1,
        &one_day_duration,
        &Some(service_metadata.clone()),
        &milestone_payment,
    );

    let service_data = client.get_service(&service_id_1);

    assert_eq!(service_data.id, service_id_1);
    assert_eq!(service_data.employee, emploee_1);
    assert_eq!(service_data.employer, employer_1);
    assert_eq!(service_data.duration, one_day_duration * 86400); // in seconds
    assert_eq!(service_data.metadata, Some(service_metadata));
    assert_eq!(service_data.milestone_payment, milestone_payment);
}
