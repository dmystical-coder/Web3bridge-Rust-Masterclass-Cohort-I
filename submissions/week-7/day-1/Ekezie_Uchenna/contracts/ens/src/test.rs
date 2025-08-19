#![cfg(test)]

use super::{ EmployeeManagementContract, EmployeeManagementContractClient };
use crate::types::{ Employee, Institution, EmployeeRank, EmployeeStatus };
use crate::errors::Error;
use soroban_sdk::{
    testutils::{ Address as _, Events },
    token::{ StellarAssetClient, TokenClient },
    Address,
    Env,
    String,
};

// Create a test environment helper
fn create_test_env() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let institution_id = Address::generate(&env);
    let institution_admin = Address::generate(&env);
    let token_address = Address::generate(&env);

    (env, admin, institution_id, institution_admin, token_address)
}

// Setup contract with admin
fn setup_contract(env: &Env, admin: &Address) -> EmployeeManagementContractClient {
    let contract_id = env.register_contract(None, EmployeeManagementContract);
    let client = EmployeeManagementContractClient::new(env, &contract_id);
    client.initialize(admin);
    client
}

// Setup institution
fn setup_institution(
    client: &EmployeeManagementContractClient,
    institution_id: &Address,
    institution_admin: &Address,
    token_address: &Address
) {
    client.register_institution(
        institution_id,
        &String::from_str(client.env(), "Test Company"),
        institution_admin,
        &Some(token_address.clone())
    );
}

#[test]
fn test_contract_initialization() {
    let (env, admin, _, _, _) = create_test_env();
    let client = setup_contract(&env, &admin);

    // Test that initialization worked
    // Since get_admin is not in the main trait, we test by trying admin operations
    let institution_id = Address::generate(&env);
    let institution_admin = Address::generate(&env);
    let token_address = Address::generate(&env);

    // This should work since admin is set
    let result = client.try_register_institution(
        &institution_id,
        &String::from_str(&env, "Test"),
        &institution_admin,
        &Some(token_address)
    );
    assert!(result.is_ok());
}

#[test]
fn test_register_institution() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);

    let result = client.try_register_institution(
        &institution_id,
        &String::from_str(&env, "Test Company"),
        &institution_admin,
        &Some(token_address.clone())
    );

    assert!(result.is_ok());

    let institution = client.get_institution(&institution_id).unwrap();
    assert_eq!(institution.id, institution_id);
    assert_eq!(institution.name, String::from_str(&env, "Test Company"));
    assert_eq!(institution.admin, institution_admin);
    assert_eq!(institution.token_contract, Some(token_address));
    assert_eq!(institution.is_active, true);
}

#[test]
fn test_register_duplicate_institution() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);

    // Register institution first time
    client.register_institution(
        &institution_id,
        &String::from_str(&env, "Test Company"),
        &institution_admin,
        &Some(token_address.clone())
    );

    // Try to register same institution again
    let result = client.try_register_institution(
        &institution_id,
        &String::from_str(&env, "Test Company 2"),
        &institution_admin,
        &Some(token_address)
    );

    assert_eq!(result.unwrap_err().unwrap(), Error::InstitutionAlreadyExists);
}

#[test]
fn test_add_employee() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee_id = Address::generate(&env);

    let result = client.try_add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    assert!(result.is_ok());

    let employee = client.get_employee(&employee_id).unwrap();
    assert_eq!(employee.id, employee_id);
    assert_eq!(employee.institution_id, institution_id);
    assert_eq!(employee.name, String::from_str(&env, "John Doe"));
    assert_eq!(employee.rank, EmployeeRank::Junior);
    assert_eq!(employee.salary, 50000);
    assert_eq!(employee.status, EmployeeStatus::Active);

    // Check events
    let events = env.events().all();
    assert!(!events.is_empty());
}

#[test]
fn test_add_duplicate_employee() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee_id = Address::generate(&env);

    // Add employee first time
    client.add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    // Try to add same employee again
    let result = client.try_add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "Jane Doe"),
        &EmployeeRank::Senior,
        &70000
    );

    assert_eq!(result.unwrap_err().unwrap(), Error::EmployeeAlreadyExists);
}

#[test]
fn test_update_employee() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee_id = Address::generate(&env);
    client.add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    // Update employee
    let result = client.try_update_employee(
        &employee_id,
        &Some(String::from_str(&env, "John Smith")),
        &Some(55000)
    );

    assert!(result.is_ok());

    let employee = client.get_employee(&employee_id).unwrap();
    assert_eq!(employee.name, String::from_str(&env, "John Smith"));
    assert_eq!(employee.salary, 55000);
    assert_eq!(employee.rank, EmployeeRank::Junior); // Should remain unchanged
}

#[test]
fn test_promote_employee() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee_id = Address::generate(&env);
    client.add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    // Promote employee
    let result = client.try_promote_employee(&employee_id, &EmployeeRank::Senior, &80000);

    assert!(result.is_ok());

    let employee = client.get_employee(&employee_id).unwrap();
    assert_eq!(employee.rank, EmployeeRank::Senior);
    assert_eq!(employee.salary, 80000);
    assert!(employee.last_promotion.is_some());
}

#[test]
fn test_promote_inactive_employee() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee_id = Address::generate(&env);
    client.add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    // Suspend employee first
    client.suspend_employee(&employee_id);

    // Try to promote suspended employee
    let result = client.try_promote_employee(&employee_id, &EmployeeRank::Senior, &80000);

    assert_eq!(result.unwrap_err().unwrap(), Error::EmployeeNotActive);
}

#[test]
fn test_suspend_and_reactivate_employee() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee_id = Address::generate(&env);
    client.add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    // Suspend employee
    let result = client.try_suspend_employee(&employee_id);
    assert!(result.is_ok());

    let employee = client.get_employee(&employee_id).unwrap();
    assert_eq!(employee.status, EmployeeStatus::Suspended);

    // Reactivate employee
    let result = client.try_reactivate_employee(&employee_id);
    assert!(result.is_ok());

    let employee = client.get_employee(&employee_id).unwrap();
    assert_eq!(employee.status, EmployeeStatus::Active);
}

#[test]
fn test_suspend_already_suspended_employee() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee_id = Address::generate(&env);
    client.add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    // Suspend employee
    client.suspend_employee(&employee_id);

    // Try to suspend again
    let result = client.try_suspend_employee(&employee_id);
    assert_eq!(result.unwrap_err().unwrap(), Error::EmployeeAlreadySuspended);
}

#[test]
fn test_remove_employee() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee_id = Address::generate(&env);
    client.add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    // Remove employee
    let result = client.try_remove_employee(&employee_id);
    assert!(result.is_ok());

    let employee = client.get_employee(&employee_id).unwrap();
    assert_eq!(employee.status, EmployeeStatus::Terminated);
}

#[test]
fn test_cannot_reactivate_terminated_employee() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee_id = Address::generate(&env);
    client.add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    // Remove employee
    client.remove_employee(&employee_id);

    // Try to reactivate terminated employee
    let result = client.try_reactivate_employee(&employee_id);
    assert_eq!(result.unwrap_err().unwrap(), Error::CannotReactivateTerminated);
}

#[test]
fn test_get_employees_by_institution() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee1_id = Address::generate(&env);
    let employee2_id = Address::generate(&env);

    client.add_employee(
        &employee1_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    client.add_employee(
        &employee2_id,
        &institution_id,
        &String::from_str(&env, "Jane Smith"),
        &EmployeeRank::Senior,
        &80000
    );

    let employees = client.get_employees_by_institution(&institution_id).unwrap();
    assert_eq!(employees.len(), 2);
}

#[test]
fn test_get_employees_by_rank() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee1_id = Address::generate(&env);
    let employee2_id = Address::generate(&env);
    let employee3_id = Address::generate(&env);

    client.add_employee(
        &employee1_id,
        &institution_id,
        &String::from_str(&env, "John Junior"),
        &EmployeeRank::Junior,
        &50000
    );

    client.add_employee(
        &employee2_id,
        &institution_id,
        &String::from_str(&env, "Jane Senior"),
        &EmployeeRank::Senior,
        &80000
    );

    client.add_employee(
        &employee3_id,
        &institution_id,
        &String::from_str(&env, "Bob Junior"),
        &EmployeeRank::Junior,
        &52000
    );

    let junior_employees = client
        .get_employees_by_rank(&institution_id, &EmployeeRank::Junior)
        .unwrap();
    let senior_employees = client
        .get_employees_by_rank(&institution_id, &EmployeeRank::Senior)
        .unwrap();

    assert_eq!(junior_employees.len(), 2);
    assert_eq!(senior_employees.len(), 1);
}

#[test]
fn test_get_active_employee_count() {
    let (env, admin, institution_id, institution_admin, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);
    setup_institution(&client, &institution_id, &institution_admin, &token_address);

    let employee1_id = Address::generate(&env);
    let employee2_id = Address::generate(&env);

    client.add_employee(
        &employee1_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    client.add_employee(
        &employee2_id,
        &institution_id,
        &String::from_str(&env, "Jane Smith"),
        &EmployeeRank::Senior,
        &80000
    );

    // Both should be active
    assert_eq!(client.get_active_employee_count(&institution_id).unwrap(), 2);

    // Suspend one employee
    client.suspend_employee(&employee1_id);
    assert_eq!(client.get_active_employee_count(&institution_id).unwrap(), 1);

    // Remove one employee
    client.remove_employee(&employee2_id);
    assert_eq!(client.get_active_employee_count(&institution_id).unwrap(), 0);
}

#[test]
fn test_pay_salary_without_token_contract() {
    let (env, admin, institution_id, institution_admin, _) = create_test_env();
    let client = setup_contract(&env, &admin);

    // Register institution without token contract
    client.register_institution(
        &institution_id,
        &String::from_str(&env, "Test Company"),
        &institution_admin,
        &None
    );

    let employee_id = Address::generate(&env);
    client.add_employee(
        &employee_id,
        &institution_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    // Try to pay salary without token contract
    let result = client.try_pay_salary(&employee_id);
    assert_eq!(result.unwrap_err().unwrap(), Error::NoTokenContract);
}

#[test]
fn test_get_nonexistent_employee() {
    let (env, admin, _, _, _) = create_test_env();
    let client = setup_contract(&env, &admin);

    let nonexistent_employee = Address::generate(&env);
    let result = client.try_get_employee(&nonexistent_employee);
    assert_eq!(result.unwrap_err().unwrap(), Error::EmployeeNotFound);
}

#[test]
fn test_get_nonexistent_institution() {
    let (env, admin, _, _, _) = create_test_env();
    let client = setup_contract(&env, &admin);

    let nonexistent_institution = Address::generate(&env);
    let result = client.try_get_institution(&nonexistent_institution);
    assert_eq!(result.unwrap_err().unwrap(), Error::InstitutionNotFound);
}

#[test]
fn test_multiple_institutions() {
    let (env, admin, _, _, token_address) = create_test_env();
    let client = setup_contract(&env, &admin);

    let institution1_id = Address::generate(&env);
    let institution2_id = Address::generate(&env);
    let institution1_admin = Address::generate(&env);
    let institution2_admin = Address::generate(&env);

    // Register two institutions
    client.register_institution(
        &institution1_id,
        &String::from_str(&env, "Company A"),
        &institution1_admin,
        &Some(token_address.clone())
    );

    client.register_institution(
        &institution2_id,
        &String::from_str(&env, "Company B"),
        &institution2_admin,
        &Some(token_address)
    );

    let employee1_id = Address::generate(&env);
    let employee2_id = Address::generate(&env);

    // Add employees to different institutions
    client.add_employee(
        &employee1_id,
        &institution1_id,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000
    );

    client.add_employee(
        &employee2_id,
        &institution2_id,
        &String::from_str(&env, "Jane Smith"),
        &EmployeeRank::Senior,
        &80000
    );

    // Verify employees are in correct institutions
    let inst1_employees = client.get_employees_by_institution(&institution1_id).unwrap();
    let inst2_employees = client.get_employees_by_institution(&institution2_id).unwrap();

    assert_eq!(inst1_employees.len(), 1);
    assert_eq!(inst2_employees.len(), 1);
}
