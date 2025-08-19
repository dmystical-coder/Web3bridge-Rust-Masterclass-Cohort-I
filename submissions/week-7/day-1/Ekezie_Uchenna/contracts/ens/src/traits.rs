// src/traits.rs

use soroban_sdk::{ Address, Env, Vec, String };
use crate::types::{ Employee, Institution, EmployeeRank };
use crate::errors::Error;

/// Main contract interface trait
pub trait EmployeeManagement {
    /// Contract initialization
    fn initialize(env: Env, admin: Address) -> Result<(), Error>;

    /// Institution management
    fn register_institution(
        env: Env,
        institution_id: Address,
        name: String,
        admin: Address,
        token_contract: Option<Address>
    ) -> Result<(), Error>;

    /// Core employee actions
    fn add_employee(
        env: Env,
        employee_id: Address,
        institution_id: Address,
        name: String,
        rank: EmployeeRank,
        salary: i128
    ) -> Result<(), Error>;

    fn update_employee(
        env: Env,
        employee_id: Address,
        new_name: Option<String>,
        new_salary: Option<i128>
    ) -> Result<(), Error>;

    fn promote_employee(
        env: Env,
        employee_id: Address,
        new_rank: EmployeeRank,
        new_salary: i128
    ) -> Result<(), Error>;

    fn suspend_employee(env: Env, employee_id: Address) -> Result<(), Error>;

    fn reactivate_employee(env: Env, employee_id: Address) -> Result<(), Error>;

    fn remove_employee(env: Env, employee_id: Address) -> Result<(), Error>;

    /// Salary payment via cross-contract call
    fn pay_salary(env: Env, employee_id: Address) -> Result<(), Error>;

    /// Query functions
    fn get_employee(env: Env, employee_id: Address) -> Result<Employee, Error>;

    fn get_institution(env: Env, institution_id: Address) -> Result<Institution, Error>;

    fn get_employees_by_institution(
        env: Env,
        institution_id: Address
    ) -> Result<Vec<Address>, Error>;

    fn get_employees_by_rank(
        env: Env,
        institution_id: Address,
        rank: EmployeeRank
    ) -> Result<Vec<Address>, Error>;

    fn get_active_employee_count(env: Env, institution_id: Address) -> Result<u32, Error>;
}

/// Administrative functions trait (optional extension)

pub trait AdminFunctions {
    /// Transfer admin role
    fn transfer_admin(env: Env, new_admin: Address) -> Result<(), Error>;

    /// Deactivate institution
    fn deactivate_institution(env: Env, institution_id: Address) -> Result<(), Error>;

    /// Reactivate institution
    fn reactivate_institution(env: Env, institution_id: Address) -> Result<(), Error>;

    /// Emergency pause contract
    fn pause_contract(env: Env) -> Result<(), Error>;

    /// Resume contract operations
    fn resume_contract(env: Env) -> Result<(), Error>;

    /// Get contract admin
    fn get_admin(env: Env) -> Result<Address, Error>;
}

/// Institution management trait (for future extension)

pub trait InstitutionManagement {
    /// Update institution information
    fn update_institution(
        env: Env,
        institution_id: Address,
        new_name: Option<String>,
        new_admin: Option<Address>,
        new_token_contract: Option<Address>
    ) -> Result<(), Error>;

    /// Transfer institution admin role
    fn transfer_institution_admin(
        env: Env,
        institution_id: Address,
        new_admin: Address
    ) -> Result<(), Error>;

    /// Set institution token contract
    fn set_institution_token(
        env: Env,
        institution_id: Address,
        token_contract: Address
    ) -> Result<(), Error>;

    /// Get institution statistics
    fn get_institution_stats(env: Env, institution_id: Address) -> Result<InstitutionStats, Error>;
}

/// Reporting and analytics trait (for future extension)

pub trait Analytics {
    /// Get employee statistics
    fn get_employee_stats(env: Env, employee_id: Address) -> Result<EmployeeStats, Error>;

    /// Get salary statistics for institution
    fn get_salary_stats(env: Env, institution_id: Address) -> Result<SalaryStats, Error>;

    /// Get rank distribution for institution
    fn get_rank_distribution(
        env: Env,
        institution_id: Address
    ) -> Result<Vec<(EmployeeRank, u32)>, Error>;

    /// Calculate total payroll for institution
    fn calculate_total_payroll(env: Env, institution_id: Address) -> Result<i128, Error>;
}

/// Batch operations trait (for efficiency)

pub trait BatchOperations {
    /// Batch add employees
    fn batch_add_employees(
        env: Env,
        institution_id: Address,
        employees: Vec<(Address, String, EmployeeRank, i128)>
    ) -> Result<(), Error>;

    /// Batch promote employees
    fn batch_promote_employees(
        env: Env,
        promotions: Vec<(Address, EmployeeRank, i128)>
    ) -> Result<(), Error>;

    /// Batch pay salaries
    fn batch_pay_salaries(env: Env, employee_ids: Vec<Address>) -> Result<(), Error>;

    /// Batch update salaries
    fn batch_update_salaries(env: Env, updates: Vec<(Address, i128)>) -> Result<(), Error>;
}

/// Supporting data structures for analytics
use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmployeeStats {
    pub employee_id: Address,
    pub days_employed: u64,
    pub promotion_count: u32,
    pub total_salary_paid: i128,
    pub current_rank: EmployeeRank,
    pub performance_score: Option<u32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstitutionStats {
    pub institution_id: Address,
    pub total_employees: u32,
    pub active_employees: u32,
    pub suspended_employees: u32,
    pub terminated_employees: u32,
    pub total_payroll: i128,
    pub average_salary: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SalaryStats {
    pub min_salary: i128,
    pub max_salary: i128,
    pub average_salary: i128,
    pub median_salary: i128,
    pub total_payroll: i128,
    pub salary_payments_this_month: u32,
}

/// Validation trait for input checking
pub trait Validation {
    /// Validate employee data
    fn validate_employee_data(
        name: &String,
        rank: &EmployeeRank,
        salary: i128
    ) -> Result<(), Error>;

    /// Validate institution data
    fn validate_institution_data(name: &String, admin: &Address) -> Result<(), Error>;

    /// Validate promotion logic
    fn validate_promotion(
        current_rank: &EmployeeRank,
        new_rank: &EmployeeRank,
        current_salary: i128,
        new_salary: i128
    ) -> Result<(), Error>;
}

/// Default validation implementation
impl Validation for () {
    fn validate_employee_data(
        name: &String,
        _rank: &EmployeeRank,
        salary: i128
    ) -> Result<(), Error> {
        if name.len() == 0 || name.len() > 100 {
            return Err(Error::InvalidName);
        }
        if salary <= 0 {
            return Err(Error::InvalidSalary);
        }
        Ok(())
    }

    fn validate_institution_data(name: &String, _admin: &Address) -> Result<(), Error> {
        if name.len() == 0 || name.len() > 100 {
            return Err(Error::InvalidName);
        }
        Ok(())
    }

    fn validate_promotion(
        current_rank: &EmployeeRank,
        new_rank: &EmployeeRank,
        current_salary: i128,
        new_salary: i128
    ) -> Result<(), Error> {
        // Check if promotion is to a higher rank
        if !new_rank.is_higher_than(current_rank) {
            return Err(Error::InvalidRank);
        }

        // Check if salary increases with promotion (optional business rule)
        if new_salary <= current_salary {
            return Err(Error::InvalidSalary);
        }

        // CEO cannot be promoted further
        if current_rank == &EmployeeRank::CEO {
            return Err(Error::CannotPromoteCEO);
        }

        Ok(())
    }
}
