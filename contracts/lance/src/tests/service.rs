#![cfg(test)]

use crate::{
    storage::service_status::ServiceStatus,
    tests::config::{constants::BASE_MINT_AMOUNT, contract::ContractTest},
};

use soroban_sdk::{
    testutils::{Address as _, Ledger as _, MockAuth, MockAuthInvoke},
    token, Env, String, Address
};

use crate::contract::{ContractClient, Contract};

fn create_contract<'a>(
    env: &'a Env,
    admin: &Address,
    token_admin: &Address,
) -> (ContractClient<'a>, token::Client<'a>, token::StellarAssetClient<'a>, Address) {
    let sac = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = sac.address();

    // client with transfer/balance/etc
    let token_client = token::Client::new(env, &token_id);

    // admin client with mint
    let token_admin_client = token::StellarAssetClient::new(env, &token_id);

    let contract_id = env.register(Contract, (admin.clone(), token_id.clone()));
    let client = ContractClient::new(env, &contract_id);

    (client, token_client, token_admin_client, token_id)
}


#[test]
fn test_create_employee_user() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (client, _token, _token_admin_client, _token_id) = create_contract(&env, &admin, &token_admin);

    let employee = Address::generate(&env);
    client.new_user(&employee, &true, &false, &false, &None);

    let user_data = client.get_user(&employee);

    assert_eq!(user_data.address, employee);
    assert!(user_data.is_employee);
    assert!(!user_data.is_employer);
    assert!(!user_data.is_judge);
    assert!(user_data.personal_data.is_none());
}

#[test]
fn test_create_employer_user() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (client, _token, _token_admin_client, _token_id) = create_contract(&env, &admin, &token_admin);

    let employer = Address::generate(&env);
    client.new_user(&employer, &false, &true, &false, &None);

    let user_data = client.get_user(&employer);

    assert_eq!(user_data.address, employer);
    assert!(!user_data.is_employee);
    assert!(user_data.is_employer);
    assert!(!user_data.is_judge);
    assert!(user_data.personal_data.is_none());
}

#[test]
fn test_create_user_with_personal_data() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (client, _token, _token_admin_client, _token_id) = create_contract(&env, &admin, &token_admin);

    let user = Address::generate(&env);
    let personal_data = String::from_str(&env, "Employee 1");

    client.new_user(&user, &true, &false, &false, &Some(personal_data.clone()));

    let user_data = client.get_user(&user);
    assert_eq!(user_data.personal_data, Some(personal_data));
}

#[test]
fn test_multiple_users_are_independent() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (client, _token, _token_admin_client, _token_id) = create_contract(&env, &admin, &token_admin);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    client.new_user(&user1, &true, &false, &false, &None);
    client.new_user(&user2, &false, &true, &false, &None);

    let data1 = client.get_user(&user1);
    let data2 = client.get_user(&user2);

    assert_ne!(data1.address, data2.address);
    assert!(data1.is_employee && !data2.is_employee);
    assert!(data2.is_employer && !data1.is_employer);
}


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
        &true,   // is_employee
        &false,  // is_employer
        &false,  // is_judge
        &Some(personal_data.clone()),
    );

    let user_data = client.get_user(&user_1);

    assert_eq!(user_data.address, user_1);
    assert_eq!(user_data.is_employee, true);
    assert_eq!(user_data.is_employer, false);
    assert_eq!(user_data.is_judge, false);
    assert_eq!(user_data.personal_data, Some(personal_data));
}

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
