#![cfg(test)]

use crate::{
    storage::service_status::ServiceStatus,
    tests::config::{constants::BASE_MINT_AMOUNT, contract::ContractTest},
};

use soroban_sdk::{
    testutils::{Address as _, Ledger as _, MockAuth, MockAuthInvoke},
    token, Address, Env, String,
};

use crate::contract::{Contract, ContractClient};
#[test]
fn test_add_and_get_user() {
    let ContractTest {
        env,
        contract,
        employee_1,
        ..
    } = ContractTest::setup();

    let personal_data = String::from_str(&env, "Employee 1");
    contract.new_user(
        &employee_1,
        &true,  // is_employee
        &false, // is_employer
        &false, // is_judge
        &Some(personal_data.clone()),
    );

    let user_data = contract.get_user(&employee_1);

    assert_eq!(user_data.address, employee_1);
    assert_eq!(user_data.is_employee, true);
    assert_eq!(user_data.is_employer, false);
    assert_eq!(user_data.is_judge, false);
    assert_eq!(user_data.personal_data, Some(personal_data));
}
