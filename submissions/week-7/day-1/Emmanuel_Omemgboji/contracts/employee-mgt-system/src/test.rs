#![cfg(test)]
use crate::errors::EmployeeError;
use crate::mgt_system::{EmployeeManagementContract, EmployeeManagementContractClient};
use crate::token_import::token_contract::Client as TokenContractClient;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, String,
};

fn create_token_contract(env: &Env, admin: &Address) -> Address {
    let name = String::from_str(env, "ballor-token");
    let symbol = String::from_str(env, "BLT");
    let decimals = 18u32;

    let token_contract_id = env.register(
        crate::token_import::token_contract::WASM,
        (admin, name.clone(), symbol.clone(), decimals),
    );

    token_contract_id
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
    let contract_id = env.register_contract(None, EmployeeManagementContract);

    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    let result = mgt_client.initialize(
        &admin.clone(),
        &String::from_str(&env, "Test Institution"),
        &token_contract.clone(),
    );

    assert_eq!(result, ());

    // Test that we can get institution info
    let institution_info = mgt_client.get_institution_info();
    assert_eq!(institution_info.admin, admin);
    assert_eq!(
        institution_info.name,
        String::from_str(&env, "Test Institution")
    );
    assert_eq!(institution_info.total_employees, 0_u32);
    assert_eq!(institution_info.token_contract, token_contract);

    let token_client = TokenContractClient::new(&env, &token_contract);
    assert_eq!(token_client.name(), String::from_str(&env, "ballor-token"));
    assert_eq!(token_client.symbol(), String::from_str(&env, "BLT"));
    assert_eq!(token_client.decimals(), 18u32);
}

#[test]
fn test_initialization_already_initialized() {
    let (env, admin, _, _, token_contract) = setup_test();
    let contract_id = env.register_contract(None, EmployeeManagementContract);
    let mgt_client = EmployeeManagementContractClient::new(&env, &contract_id);

    // First initialization should succeed
    mgt_client.initialize(
        &admin,
        &String::from_str(&env, "Test Institution"),
        &token_contract,
    );

    // Second initialization should panic
    let result = mgt_client.try_initialize(
        &admin,
        &String::from_str(&env, "Test Institution"),
        &token_contract,
    );

    assert_eq!(
        result.unwrap_err(),
        Ok(EmployeeError::AlreadyInitialized),
        "Error should be AlreadyInitialized"
    );
}
