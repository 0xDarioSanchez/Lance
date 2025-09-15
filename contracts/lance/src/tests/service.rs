#![cfg(test)]

use crate::{storage::service_status::ServiceStatus, tests::config::contract::ContractTest};

use soroban_sdk::{testutils::Address as _, Address, String};

#[test]
fn test_add_and_new_service() {
    let ContractTest {
        env,
        contract,
        employee_1,
        employer_1,
        ..
    } = ContractTest::setup();

    // *****************
    // ***** Given *****
    // *****************
    env.mock_all_auths();

    let service_id_1: u32 = 1;
    let one_day_duration: u64 = 1; // days
    let milestone_payment: i128 = 1000;

    // Now you can call other contract methods
    let service_metadata = String::from_str(&env, "Service 1");

    // *****************
    //  ***** When *****
    // *****************
    contract.create_service(
        &employee_1,
        &employer_1,
        &service_id_1,
        &one_day_duration,
        &Some(service_metadata.clone()),
        &milestone_payment,
    );

    // ****************
    // ***** Then *****
    // ****************
    let service_data = contract.get_service(&service_id_1);

    assert_eq!(service_data.id, service_id_1);
    assert_eq!(service_data.employee, employee_1);
    assert_eq!(service_data.employer, employer_1);
    assert_eq!(service_data.duration, one_day_duration * 86400); // in seconds
    assert_eq!(service_data.metadata, Some(service_metadata));
    assert_eq!(service_data.milestone_payment, milestone_payment);
    assert_eq!(service_data.status, ServiceStatus::CREATED);
}
