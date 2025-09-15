#![cfg(test)]

use crate::tests::config::{constants::BASE_MINT_AMOUNT, contract::ContractTest};
use soroban_sdk::String;

#[test]
fn test_add_and_new_service() {
    let ContractTest {
        env,
        contract,
        employee_1,
        employer_1,
        ..
    } = ContractTest::setup();

    env.mock_all_auths();

    let service_id_1: u32 = 1;
    let one_day_duration: u64 = 1; // days
    let milestone_payment: i128 = 1000;

    // Now you can call other contract methods
    let service_metadata = String::from_str(&env, "Service 1");

    contract.create_service(
        &employee_1,
        &employer_1,
        &service_id_1,
        &one_day_duration,
        &Some(service_metadata.clone()),
        &milestone_payment,
    );

    let service_data = contract.get_service(&service_id_1);

    assert_eq!(service_data.id, service_id_1);
    assert_eq!(service_data.employee, employee_1);
    assert_eq!(service_data.employer, employer_1);
    assert_eq!(service_data.duration, one_day_duration * 86400); // in seconds
    assert_eq!(service_data.metadata, Some(service_metadata));
    assert_eq!(service_data.milestone_payment, milestone_payment);
}

#[test]
fn test_accept_service() {
    let ContractTest {
        env,
        contract,
        employee_1,
        employer_1,
        token,
        ..
    } = ContractTest::setup();

    env.mock_all_auths();
    let (token_client, _, _) = token;

    let service_id_1: u32 = 1;
    let one_day_duration: u64 = 1; // days
    let milestone_payment: i128 = 1000;

    assert_eq!(token_client.balance(&contract.address), 0);
    assert_eq!(token_client.balance(&employee_1), BASE_MINT_AMOUNT);
    assert_eq!(token_client.balance(&employer_1), BASE_MINT_AMOUNT);

    contract.create_service(
        &employee_1,
        &employer_1,
        &service_id_1,
        &one_day_duration,
        &None,
        &milestone_payment,
    );

    contract.accept_service(&employer_1, &service_id_1);

    assert_eq!(token_client.balance(&employee_1), BASE_MINT_AMOUNT);
    assert_eq!(
        token_client.balance(&employer_1),
        BASE_MINT_AMOUNT - milestone_payment
    );
    assert_eq!(token_client.balance(&contract.address), milestone_payment);
}
