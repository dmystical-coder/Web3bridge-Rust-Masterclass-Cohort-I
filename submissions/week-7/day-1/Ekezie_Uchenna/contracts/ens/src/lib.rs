#![no_std]

mod types;
mod storage;
mod errors;
// mod events;
mod token_client;
mod traits;

use soroban_sdk::{ contract, contractimpl, Address, Env, String, Vec };

use types::{ Employee, Institution, EmployeeRank, EmployeeStatus };
use storage::Storage;
use errors::Error;
use events::Events;
use token_client::TokenClient;
use traits::EmployeeManagement;

#[contract]
pub struct EmployeeManagementContract;

#[contractimpl]
impl EmployeeManagement for EmployeeManagementContract {
    /// Initialize the contract with admin
    fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        admin.require_auth();
        Storage::set_admin(&env, &admin);
        Ok(())
    }

    /// Register a new institution
    fn register_institution(
        env: Env,
        institution_id: Address,
        name: String,
        admin: Address,
        token_contract: Option<Address>
    ) -> Result<(), Error> {
        let contract_admin = Storage::get_admin(&env)?;
        contract_admin.require_auth();

        if Storage::has_institution(&env, &institution_id) {
            return Err(Error::InstitutionAlreadyExists);
        }

        let institution = Institution {
            id: institution_id.clone(),
            name,
            admin,
            token_contract,
            employee_count: 0,
            is_active: true,
        };

        Storage::set_institution(&env, &institution_id, &institution);
        Events::institution_registered(&env, &institution_id, &institution.admin);

        Ok(())
    }

    /// Add new employee to institution
    fn add_employee(
        env: Env,
        employee_id: Address,
        institution_id: Address,
        name: String,
        rank: EmployeeRank,
        salary: i128
    ) -> Result<(), Error> {
        let institution = Storage::get_institution(&env, &institution_id)?;
        institution.admin.require_auth();

        if !institution.is_active {
            return Err(Error::InstitutionNotActive);
        }

        if Storage::has_employee(&env, &employee_id) {
            return Err(Error::EmployeeAlreadyExists);
        }

        let employee = Employee {
            id: employee_id.clone(),
            institution_id: institution_id.clone(),
            name,
            rank: rank.clone(),
            salary,
            status: EmployeeStatus::Active,
            hire_date: env.ledger().timestamp(),
            last_promotion: None,
            last_salary_payment: None,
        };

        Storage::set_employee(&env, &employee_id, &employee);
        Storage::add_employee_to_institution(&env, &institution_id, &employee_id);

        Events::employee_added(&env, &employee_id, &institution_id, &rank, salary);

        Ok(())
    }

    /// Update employee information
    fn update_employee(
        env: Env,
        employee_id: Address,
        new_name: Option<String>,
        new_salary: Option<i128>
    ) -> Result<(), Error> {
        let mut employee = Storage::get_employee(&env, &employee_id)?;
        let institution = Storage::get_institution(&env, &employee.institution_id)?;
        institution.admin.require_auth();

        if employee.status == EmployeeStatus::Terminated {
            return Err(Error::EmployeeTerminated);
        }

        let old_salary = employee.salary;

        if let Some(name) = new_name {
            employee.name = name;
        }

        if let Some(salary) = new_salary {
            employee.salary = salary;
        }

        Storage::set_employee(&env, &employee_id, &employee);
        Events::employee_updated(&env, &employee_id, old_salary, employee.salary);

        Ok(())
    }

    /// Promote employee with new rank and salary
    fn promote_employee(
        env: Env,
        employee_id: Address,
        new_rank: EmployeeRank,
        new_salary: i128
    ) -> Result<(), Error> {
        let mut employee = Storage::get_employee(&env, &employee_id)?;
        let institution = Storage::get_institution(&env, &employee.institution_id)?;
        institution.admin.require_auth();

        if employee.status != EmployeeStatus::Active {
            return Err(Error::EmployeeNotActive);
        }

        let old_rank = employee.rank.clone();
        let old_salary = employee.salary;

        employee.rank = new_rank.clone();
        employee.salary = new_salary;
        employee.last_promotion = Some(env.ledger().timestamp());

        Storage::set_employee(&env, &employee_id, &employee);
        Events::employee_promoted(&env, &employee_id, &old_rank, &new_rank, old_salary, new_salary);

        Ok(())
    }

    /// Suspend employee
    fn suspend_employee(env: Env, employee_id: Address) -> Result<(), Error> {
        let mut employee = Storage::get_employee(&env, &employee_id)?;
        let institution = Storage::get_institution(&env, &employee.institution_id)?;
        institution.admin.require_auth();

        if employee.status == EmployeeStatus::Terminated {
            return Err(Error::EmployeeTerminated);
        }

        if employee.status == EmployeeStatus::Suspended {
            return Err(Error::EmployeeAlreadySuspended);
        }

        employee.status = EmployeeStatus::Suspended;
        Storage::set_employee(&env, &employee_id, &employee);
        Events::employee_suspended(&env, &employee_id);

        Ok(())
    }

    /// Reactivate suspended employee
    fn reactivate_employee(env: Env, employee_id: Address) -> Result<(), Error> {
        let mut employee = Storage::get_employee(&env, &employee_id)?;
        let institution = Storage::get_institution(&env, &employee.institution_id)?;
        institution.admin.require_auth();

        if employee.status == EmployeeStatus::Terminated {
            return Err(Error::CannotReactivateTerminated);
        }

        if employee.status == EmployeeStatus::Active {
            return Err(Error::EmployeeAlreadyActive);
        }

        employee.status = EmployeeStatus::Active;
        Storage::set_employee(&env, &employee_id, &employee);
        Events::employee_reactivated(&env, &employee_id);

        Ok(())
    }

    /// Remove (terminate) employee
    fn remove_employee(env: Env, employee_id: Address) -> Result<(), Error> {
        let mut employee = Storage::get_employee(&env, &employee_id)?;
        let institution = Storage::get_institution(&env, &employee.institution_id)?;
        institution.admin.require_auth();

        employee.status = EmployeeStatus::Terminated;
        Storage::set_employee(&env, &employee_id, &employee);
        Storage::remove_employee_from_institution(&env, &employee.institution_id, &employee_id);
        Events::employee_removed(&env, &employee_id);

        Ok(())
    }

    /// Pay salary to employee via token contract
    fn pay_salary(env: Env, employee_id: Address) -> Result<(), Error> {
        let mut employee = Storage::get_employee(&env, &employee_id)?;
        let institution = Storage::get_institution(&env, &employee.institution_id)?;
        institution.admin.require_auth();

        if employee.status != EmployeeStatus::Active {
            return Err(Error::EmployeeNotActive);
        }

        let token_contract = institution.token_contract.ok_or(Error::NoTokenContract)?;

        // Check if payment is due (30 days since last payment)
        let current_time = env.ledger().timestamp();
        if let Some(last_payment) = employee.last_salary_payment {
            let days_since_payment = (current_time - last_payment) / (24 * 60 * 60);
            if days_since_payment < 30 {
                return Err(Error::SalaryAlreadyPaid);
            }
        }

        // Make cross-contract call to transfer tokens
        TokenClient::transfer(
            &env,
            &token_contract,
            &institution.id,
            &employee_id,
            employee.salary
        )?;

        employee.last_salary_payment = Some(current_time);
        Storage::set_employee(&env, &employee_id, &employee);
        Events::salary_paid(&env, &employee_id, employee.salary, current_time);

        Ok(())
    }

    /// Get employee information
    fn get_employee(env: Env, employee_id: Address) -> Result<Employee, Error> {
        Storage::get_employee(&env, &employee_id)
    }

    /// Get institution information
    fn get_institution(env: Env, institution_id: Address) -> Result<Institution, Error> {
        Storage::get_institution(&env, &institution_id)
    }

    /// Get all employees for an institution
    fn get_employees_by_institution(
        env: Env,
        institution_id: Address
    ) -> Result<Vec<Address>, Error> {
        if !Storage::has_institution(&env, &institution_id) {
            return Err(Error::InstitutionNotFound);
        }
        Ok(Storage::get_institution_employees(&env, &institution_id))
    }

    /// Get employees by rank within an institution
    fn get_employees_by_rank(
        env: Env,
        institution_id: Address,
        rank: EmployeeRank
    ) -> Result<Vec<Address>, Error> {
        let employees = Storage::get_institution_employees(&env, &institution_id);
        let mut filtered = Vec::new(&env);

        for i in 0..employees.len() {
            let employee_id = employees.get(i).unwrap();
            if let Ok(employee) = Storage::get_employee(&env, &employee_id) {
                if employee.rank == rank {
                    filtered.push_back(employee_id);
                }
            }
        }

        Ok(filtered)
    }

    /// Get active employees count for an institution
    fn get_active_employee_count(env: Env, institution_id: Address) -> Result<u32, Error> {
        let employees = Storage::get_institution_employees(&env, &institution_id);
        let mut count = 0;

        for i in 0..employees.len() {
            let employee_id = employees.get(i).unwrap();
            if let Ok(employee) = Storage::get_employee(&env, &employee_id) {
                if employee.status == EmployeeStatus::Active {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}
