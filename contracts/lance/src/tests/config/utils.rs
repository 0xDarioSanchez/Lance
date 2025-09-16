use soroban_sdk::{Address, Env, Val, Vec};
use soroban_sdk::testutils::Events; 

pub(crate) fn get_contract_events(env: &Env, contract_address: Address) -> Vec<(Address, Vec<Val>, Val)> {
    let mut contract_events = Vec::new(env);

    // `env.events()` implements the `Events` trait in v22
    for event in env.events().all() {
        if event.0 == contract_address {
            // use `push_back` in v22
            contract_events.push_back(event);
        }
    }

    contract_events
}