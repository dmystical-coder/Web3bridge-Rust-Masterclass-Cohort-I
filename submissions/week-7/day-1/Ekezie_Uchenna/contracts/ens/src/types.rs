// src/types.rs

use soroban_sdk::{ contracttype, Address, String };

/// Employee status enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EmployeeStatus {
    Active,
    Suspended,
    Terminated,
}

/// Employee rank enumeration (ordered by seniority)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EmployeeRank {
    Intern,
    Junior,
    Mid,
    Senior,
    Lead,
    Manager,
    Director,
    VP,
    CEO,
}

/// Employee data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employee {
    pub id: Address,
    pub institution_id: Address,
    pub name: String,
    pub rank: EmployeeRank,
    pub salary: i128,
    pub status: EmployeeStatus,
    pub hire_date: u64,
    pub last_promotion: Option<u64>,
    pub last_salary_payment: Option<u64>,
}

/// Institution data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Institution {
    pub id: Address,
    pub name: String,
    pub admin: Address,
    pub token_contract: Option<Address>,
    pub employee_count: u32,
    pub is_active: bool,
}

impl EmployeeRank {
    /// Get the numeric value of a rank (for comparison)
    pub fn level(&self) -> u8 {
        match self {
            EmployeeRank::Intern => 1,
            EmployeeRank::Junior => 2,
            EmployeeRank::Mid => 3,
            EmployeeRank::Senior => 4,
            EmployeeRank::Lead => 5,
            EmployeeRank::Manager => 6,
            EmployeeRank::Director => 7,
            EmployeeRank::VP => 8,
            EmployeeRank::CEO => 9,
        }
    }

    /// Check if this rank is higher than another
    pub fn is_higher_than(&self, other: &EmployeeRank) -> bool {
        self.level() > other.level()
    }
}

impl Employee {
    /// Check if employee can receive salary payment
    pub fn can_receive_salary(&self) -> bool {
        self.status == EmployeeStatus::Active
    }

    /// Check if employee can be promoted
    pub fn can_be_promoted(&self) -> bool {
        self.status == EmployeeStatus::Active && self.rank != EmployeeRank::CEO
    }

    /// Check if employee can be suspended
    pub fn can_be_suspended(&self) -> bool {
        self.status == EmployeeStatus::Active
    }

    /// Check if employee can be reactivated
    pub fn can_be_reactivated(&self) -> bool {
        self.status == EmployeeStatus::Suspended
    }
}

impl Institution {
    /// Check if institution can manage employees
    pub fn can_manage_employees(&self) -> bool {
        self.is_active
    }

    /// Check if institution can pay salaries
    pub fn can_pay_salaries(&self) -> bool {
        self.is_active && self.token_contract.is_some()
    }
}
