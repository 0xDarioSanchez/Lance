#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::contract::{Contract, ContractClient};

#[test]
fn test_add_and_get_user() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = Contract;
    let admin = Address::generate(&env);
    let token = Address::generate(&env);

    // Register contract AND initialize it in one step
    let contract_id = env.register(contract, (admin, token));
    let client = ContractClient::new(&env, &contract_id);

    let user_1 = Address::generate(&env);

    // Now you can call other contract methods
    let personal_data = String::from_str(&env, "Employee 1");
    client.new_user(
        &user_1,
        &true,  // is_employee
        &false, // is_employer
        &false, // is_judge
        &Some(personal_data.clone()),
    );

    let user_data = client.get_user(&user_1);

    assert_eq!(user_data.address, user_1);
    assert_eq!(user_data.is_employee, true);
    assert_eq!(user_data.is_employer, false);
    assert_eq!(user_data.is_judge, false);
    assert_eq!(user_data.personal_data, Some(personal_data));
}
