#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::contract::{Contract, ContractClient};

#[test]
fn test_add_and_get_user() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let (client, token, token_admin_client, token_id) = create_contract(&env, &admin, &token_admin);

    let user_1 = Address::generate(&env);

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
