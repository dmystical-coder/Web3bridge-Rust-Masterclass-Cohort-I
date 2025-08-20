use crate::token_import::token_contract::Client as TokenClient;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

use crate::errors::EmployeeError;
use crate::state::*;

#[contract]
pub struct EmployeeManagementContract;

#[contractimpl]
impl EmployeeManagementContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        institution_name: String,
        token_contract: Address,
    ) -> Result<(), EmployeeError> {
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(EmployeeError::AlreadyInitialized);
        }

        admin.require_auth();

        if institution_name.len() == 0 {
            return Err(EmployeeError::InvalidName);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::TokenContract, &token_contract);

        let institution_info = InstitutionInfo {
            admin: admin.clone(),
            name: institution_name,
            total_employees: 0,
            token_contract: token_contract.clone(),
        };

        env.storage()
            .instance()
            .set(&DataKey::Institution, &institution_info);

        let empty_list: Vec<Address> = Vec::new(&env);
        env.storage()
            .persistent()
            .set(&DataKey::EmployeeList, &empty_list);

        // Mark as initialized
        env.storage().instance().set(&DataKey::Initialized, &true);

        Ok(())
    }

    pub fn update_token_contract(
        env: Env,
        admin: Address,
        new_token_contract: Address,
    ) -> Result<(), EmployeeError> {
        //Checks
        Self::ensure_initialized(&env)?;
        Self::ensure_admin(&env, &admin)?;

        env.storage()
            .instance()
            .set(&DataKey::TokenContract, &new_token_contract);

        // Update institution info
        let mut institution_info: InstitutionInfo = env
            .storage()
            .instance()
            .get(&DataKey::Institution)
            .ok_or(EmployeeError::StorageError)?;

        institution_info.token_contract = new_token_contract;
        env.storage()
            .instance()
            .set(&DataKey::Institution, &institution_info);

        Ok(())
    }

    pub fn add_employee(
        env: Env,
        admin: Address,
        employee: Address,
        name: String,
        salary: i128,
        rank: EmployeeRank,
    ) -> Result<(), EmployeeError> {
        Self::ensure_initialized(&env)?;
        Self::ensure_admin(&env, &admin)?;

        // Validate inputs
        if name.len() == 0 {
            return Err(EmployeeError::InvalidName);
        }

        if salary <= 0 {
            return Err(EmployeeError::InvalidSalary);
        }

        let employee_key = DataKey::Employee(employee.clone());
        if env.storage().persistent().has(&employee_key) {
            return Err(EmployeeError::EmployeeAlreadyExists);
        }

        let new_employee = Employee {
            wallet_address: employee.clone(),
            name,
            salary,
            rank,
            status: EmployeeStatus::Active,
            last_salary_payment: 0,
        };

        env.storage().persistent().set(&employee_key, &new_employee);

        // Add to employee list
        let mut employee_list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::EmployeeList)
            .unwrap_or(Vec::new(&env));

        employee_list.push_back(employee.clone());
        env.storage()
            .persistent()
            .set(&DataKey::EmployeeList, &employee_list);

        // Update institution info
        Self::update_total_employees(&env)?;

        Ok(())
    }

    pub fn remove_employee(
        env: Env,
        admin: Address,
        employee: Address,
    ) -> Result<(), EmployeeError> {
        Self::ensure_initialized(&env)?;
        Self::ensure_admin(&env, &admin)?;

        Self::ensure_employee_exists(&env, &employee)?;

        let employee_key = DataKey::Employee(employee.clone());
        env.storage().persistent().remove(&employee_key);

        let mut employee_list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::EmployeeList)
            .unwrap();

        let mut new_list = Vec::new(&env);
        for addr in employee_list.iter() {
            if addr != employee {
                new_list.push_back(addr);
            }
        }

        env.storage()
            .persistent()
            .set(&DataKey::EmployeeList, &new_list);

        Self::update_total_employees(&env)?;

        Ok(())
    }

    pub fn update_employee(
        env: Env,
        admin: Address,
        employee: Address,
        name: Option<String>,
        salary: Option<i128>,
    ) -> Result<(), EmployeeError> {
        Self::ensure_initialized(&env)?;
        Self::ensure_admin(&env, &admin)?;

        let employee_key = DataKey::Employee(employee.clone());
        let mut employee_data: Employee = env
            .storage()
            .persistent()
            .get(&employee_key)
            .ok_or(EmployeeError::EmployeeNotFound)?;

        // Update name if provided
        if let Some(new_name) = name {
            if new_name.len() == 0 {
                return Err(EmployeeError::InvalidName);
            }
            employee_data.name = new_name;
        }

        // Update salary if provided
        if let Some(new_salary) = salary {
            if new_salary <= 0 {
                return Err(EmployeeError::InvalidSalary);
            }
            employee_data.salary = new_salary;
        }

        // Save updated employee
        env.storage()
            .persistent()
            .set(&employee_key, &employee_data);

        Ok(())
    }

    // Helper functions
    fn ensure_initialized(env: &Env) -> Result<(), EmployeeError> {
        if !env.storage().instance().has(&DataKey::Initialized) {
            return Err(EmployeeError::NotInitialized);
        }
        Ok(())
    }

    fn ensure_employee_exists(env: &Env, employee: &Address) -> Result<(), EmployeeError> {
        let employee_key = DataKey::Employee(employee.clone());
        if !env.storage().persistent().has(&employee_key) {
            return Err(EmployeeError::EmployeeNotFound);
        }
        Ok(())
    }

    fn ensure_admin(env: &Env, admin: &Address) -> Result<(), EmployeeError> {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(EmployeeError::StorageError)?;

        if *admin != stored_admin {
            return Err(EmployeeError::Unauthorized);
        }

        Ok(())
    }

    fn update_total_employees(env: &Env) -> Result<(), EmployeeError> {
        let employee_list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::EmployeeList)
            .unwrap_or(Vec::new(env));

        let mut institution_info: InstitutionInfo = env
            .storage()
            .instance()
            .get(&DataKey::Institution)
            .ok_or(EmployeeError::StorageError)?;

        institution_info.total_employees = employee_list.len();
        env.storage()
            .instance()
            .set(&DataKey::Institution, &institution_info);

        Ok(())
    }
}
