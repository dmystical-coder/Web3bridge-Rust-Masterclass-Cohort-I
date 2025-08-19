// src/errors.rs

use soroban_sdk::{ contracttype, panic_with_error };

/// Contract error codes
#[contracttype]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // Admin/Authorization errors (1-10)
    AdminNotSet = 1,
    Unauthorized = 2,

    // Institution errors (11-20)
    InstitutionNotFound = 11,
    InstitutionAlreadyExists = 12,
    InstitutionNotActive = 13,
    NoTokenContract = 14,

    // Employee errors (21-40)
    EmployeeNotFound = 21,
    EmployeeAlreadyExists = 22,
    EmployeeNotActive = 23,
    EmployeeTerminated = 24,
    EmployeeAlreadySuspended = 25,
    EmployeeAlreadyActive = 26,
    CannotReactivateTerminated = 27,
    CannotPromoteCEO = 28,

    // Salary/Payment errors (41-50)
    SalaryAlreadyPaid = 41,
    InsufficientTokenBalance = 42,
    TokenTransferFailed = 43,
    PaymentTooEarly = 44,

    // Validation errors (51-60)
    InvalidRank = 51,
    InvalidSalary = 52,
    InvalidName = 53,
    InvalidAddress = 54,

    // General errors (61-70)
    InvalidInput = 61,
    ContractNotInitialized = 62,
    OperationFailed = 63,
}

impl Error {
    /// Get error message for debugging
    pub fn message(&self) -> &'static str {
        match self {
            // Admin/Authorization errors
            Error::AdminNotSet => "Contract admin not set",
            Error::Unauthorized => "Unauthorized operation",

            // Institution errors
            Error::InstitutionNotFound => "Institution not found",
            Error::InstitutionAlreadyExists => "Institution already exists",
            Error::InstitutionNotActive => "Institution is not active",
            Error::NoTokenContract => "No token contract set for institution",

            // Employee errors
            Error::EmployeeNotFound => "Employee not found",
            Error::EmployeeAlreadyExists => "Employee already exists",
            Error::EmployeeNotActive => "Employee is not active",
            Error::EmployeeTerminated => "Employee is terminated",
            Error::EmployeeAlreadySuspended => "Employee is already suspended",
            Error::EmployeeAlreadyActive => "Employee is already active",
            Error::CannotReactivateTerminated => "Cannot reactivate terminated employee",
            Error::CannotPromoteCEO => "CEO cannot be promoted further",

            // Salary/Payment errors
            Error::SalaryAlreadyPaid => "Salary already paid this period",
            Error::InsufficientTokenBalance => "Insufficient token balance",
            Error::TokenTransferFailed => "Token transfer failed",
            Error::PaymentTooEarly => "Payment too early - wait for next payment period",

            // Validation errors
            Error::InvalidRank => "Invalid employee rank",
            Error::InvalidSalary => "Invalid salary amount",
            Error::InvalidName => "Invalid name provided",
            Error::InvalidAddress => "Invalid address provided",

            // General errors
            Error::InvalidInput => "Invalid input provided",
            Error::ContractNotInitialized => "Contract not properly initialized",
            Error::OperationFailed => "Operation failed",
        }
    }

    /// Check if error is critical (should halt execution)
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            Error::ContractNotInitialized | Error::AdminNotSet | Error::TokenTransferFailed
        )
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        !self.is_critical()
    }

    /// Get error category for logging
    pub fn category(&self) -> ErrorCategory {
        match self {
            Error::AdminNotSet | Error::Unauthorized => ErrorCategory::Authorization,

            | Error::InstitutionNotFound
            | Error::InstitutionAlreadyExists
            | Error::InstitutionNotActive
            | Error::NoTokenContract => ErrorCategory::Institution,

            | Error::EmployeeNotFound
            | Error::EmployeeAlreadyExists
            | Error::EmployeeNotActive
            | Error::EmployeeTerminated
            | Error::EmployeeAlreadySuspended
            | Error::EmployeeAlreadyActive
            | Error::CannotReactivateTerminated
            | Error::CannotPromoteCEO => ErrorCategory::Employee,

            | Error::SalaryAlreadyPaid
            | Error::InsufficientTokenBalance
            | Error::TokenTransferFailed
            | Error::PaymentTooEarly => ErrorCategory::Payment,

            | Error::InvalidRank
            | Error::InvalidSalary
            | Error::InvalidName
            | Error::InvalidAddress
            | Error::InvalidInput => ErrorCategory::Validation,

            Error::ContractNotInitialized | Error::OperationFailed => ErrorCategory::System,
        }
    }
}

/// Error categories for better organization
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorCategory {
    Authorization,
    Institution,
    Employee,
    Payment,
    Validation,
    System,
}

/// Helper macros for error handling
#[macro_export]
macro_rules! require {
    ($condition:expr, $error:expr) => {
        if !$condition {
            return Err($error);
        }
    };
}

#[macro_export]
macro_rules! ensure {
    ($condition:expr, $error:expr) => {
        if !$condition {
            panic_with_error!($error);
        }
    };
}

/// Validation helpers
pub struct Validator;

impl Validator {
    pub fn validate_salary(salary: i128) -> Result<(), Error> {
        if salary <= 0 {
            return Err(Error::InvalidSalary);
        }
        if salary > i128::MAX / 2 {
            return Err(Error::InvalidSalary);
        }
        Ok(())
    }

    pub fn validate_name(name: &str) -> Result<(), Error> {
        if name.is_empty() || name.len() > 100 {
            return Err(Error::InvalidName);
        }
        Ok(())
    }

    pub fn validate_payment_interval(
        last_payment: Option<u64>,
        current_time: u64
    ) -> Result<(), Error> {
        if let Some(last) = last_payment {
            let days_since = (current_time - last) / (24 * 60 * 60);
            if days_since < 30 {
                return Err(Error::PaymentTooEarly);
            }
        }
        Ok(())
    }
}

/// Result type alias for convenience
pub type ContractResult<T> = Result<T, Error>;
