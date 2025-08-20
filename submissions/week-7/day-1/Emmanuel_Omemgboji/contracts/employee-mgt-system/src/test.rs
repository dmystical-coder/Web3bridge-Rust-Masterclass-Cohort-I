#![cfg(test)]
use crate::msg_system::{EmployeeManagementContract, EmployeeManagementContractClient};
use crate::token_import::token_contract::Client as TokenClient;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, String,
};

fn create_token_contract(env: &Env, admin: &Address) -> Address {
    let name = String::from_str(&env, &"ballor-token");
    let symbol = String::from_str(&env, &"BLT");

    let contract_id = env.register(TokenContract, (admin, name.clone(), symbol.clone(), 18_u32));
    TokenContractClient::new(&env, &contract_id)
}

fn setup_test() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let employee2 = Address::generate(&env);
    let token_contract = create_token_contract(&env, &admin);

    (env, admin, employee1, employee2, token_contract)
}

#[test]
fn test_initialization() {
    let (env, admin, _, _, token_contract) = setup_test();
    let contract_id = env.register_contract(None, Contract);

    let result = Contract::initialize(
        &env,
        &contract_id,
        admin.clone(),
        String::from_str(&env, "Test Institution"),
        token_contract.clone(),
    );

    assert!(result.is_ok());

    // Test that we can get institution info
    let institution_info = Contract::get_institution_info(&env, &contract_id).unwrap();
    assert_eq!(institution_info.admin, admin);
    assert_eq!(
        institution_info.name,
        String::from_str(&env, "Test Institution")
    );
    assert_eq!(institution_info.total_employees, 0);
    assert_eq!(institution_info.token_contract, token_contract);
}
