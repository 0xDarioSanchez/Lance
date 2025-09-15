use soroban_sdk::{testutils::Address as _, token, Address, Env};

use crate::{contract::ContractClient, Contract};

use super::{constants::BASE_MINT_AMOUNT, utils::create_token_contract};

pub struct ContractTest<'a> {
    pub env: Env,
    pub contract: ContractClient<'a>,
    pub admin: Address,
    pub employee_1: Address,
    pub employer_1: Address,
    pub token: (token::Client<'a>, token::StellarAssetClient<'a>, Address),
}

impl<'a> ContractTest<'a> {
    pub fn setup() -> Self {
        let env = Env::default();

        let admin = Address::generate(&env);
        let token_issuer = Address::generate(&env);

        let employee_1 = Address::generate(&env);
        let employer_1 = Address::generate(&env);

        let (token_client, token_admin) = create_token_contract(&env, &token_issuer);

        token_admin
            .mock_all_auths()
            .mint(&employee_1, &BASE_MINT_AMOUNT);
        token_admin
            .mock_all_auths()
            .mint(&employer_1, &BASE_MINT_AMOUNT);

        let contract_id = env.register(Contract, (&admin, &token_admin.address));
        let contract = ContractClient::new(&env, &contract_id);

        ContractTest {
            env,
            contract,
            admin,
            employee_1,
            employer_1,
            token: (token_client, token_admin, token_issuer),
        }
    }
}
