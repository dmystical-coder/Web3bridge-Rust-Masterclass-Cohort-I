// src/token_client.rs

use soroban_sdk::{ token, Address, Env, String };
use crate::errors::Error;

/// Token client for cross-contract calls to SEP-41 token
pub struct TokenClient;

impl TokenClient {
    /// Transfer tokens from one address to another via cross-contract call
    pub fn transfer(
        env: &Env,
        token_contract: &Address,
        from: &Address,
        to: &Address,
        amount: i128
    ) -> Result<(), Error> {
        // Validate inputs
        if amount <= 0 {
            return Err(Error::InvalidSalary);
        }

        // Create token client for cross-contract call
        let token_client = token::Client::new(env, token_contract);

        // Check balance before transfer (optional safety check)
        let balance = token_client.balance(from);
        if balance < amount {
            return Err(Error::InsufficientTokenBalance);
        }

        // Perform the transfer
        // Note: This will require the `from` address to have authorized this contract
        // or the institution admin to call this function
        token_client.transfer(from, to, &amount);

        Ok(())
    }

    /// Transfer tokens using allowance mechanism
    pub fn transfer_from(
        env: &Env,
        token_contract: &Address,
        spender: &Address,
        from: &Address,
        to: &Address,
        amount: i128
    ) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidSalary);
        }

        let token_client = token::Client::new(env, token_contract);

        // Check allowance
        let allowance = token_client.allowance(from, spender);
        if allowance < amount {
            return Err(Error::InsufficientTokenBalance);
        }

        // Perform transfer from
        token_client.transfer_from(spender, from, to, &amount);

        Ok(())
    }

    /// Get token balance for an address
    pub fn get_balance(env: &Env, token_contract: &Address, address: &Address) -> i128 {
        let token_client = token::Client::new(env, token_contract);
        token_client.balance(address)
    }

    /// Get allowance between two addresses
    pub fn get_allowance(
        env: &Env,
        token_contract: &Address,
        from: &Address,
        spender: &Address
    ) -> i128 {
        let token_client = token::Client::new(env, token_contract);
        token_client.allowance(from, spender)
    }

    /// Get token name
    pub fn get_name(env: &Env, token_contract: &Address) -> soroban_sdk::String {
        let token_client = token::Client::new(env, token_contract);
        token_client.name()
    }

    /// Get token symbol
    pub fn get_symbol(env: &Env, token_contract: &Address) -> soroban_sdk::String {
        let token_client = token::Client::new(env, token_contract);
        token_client.symbol()
    }

    /// Get token decimals
    pub fn get_decimals(env: &Env, token_contract: &Address) -> u32 {
        let token_client = token::Client::new(env, token_contract);
        token_client.decimals()
    }

    /// Check if institution has sufficient balance for salary payment
    pub fn can_pay_salary(
        env: &Env,
        token_contract: &Address,
        institution: &Address,
        amount: i128
    ) -> bool {
        let balance = Self::get_balance(env, token_contract, institution);
        balance >= amount
    }

    /// Calculate total salary cost for multiple employees
    pub fn calculate_total_salary_cost(salaries: &[i128]) -> Result<i128, Error> {
        let mut total: i128 = 0;

        for &salary in salaries {
            if salary <= 0 {
                return Err(Error::InvalidSalary);
            }

            total = total.checked_add(salary).ok_or(Error::InvalidSalary)?; // Overflow protection
        }

        Ok(total)
    }

    /// Batch transfer for multiple salary payments
    pub fn batch_transfer(
        env: &Env,
        token_contract: &Address,
        from: &Address,
        transfers: &[(Address, i128)] // (to, amount) pairs
    ) -> Result<(), Error> {
        // Validate all transfers first
        let mut total_amount = 0i128;
        for (_, amount) in transfers {
            if *amount <= 0 {
                return Err(Error::InvalidSalary);
            }
            total_amount = total_amount.checked_add(*amount).ok_or(Error::InvalidSalary)?;
        }

        // Check if sender has sufficient balance
        let balance = Self::get_balance(env, token_contract, from);
        if balance < total_amount {
            return Err(Error::InsufficientTokenBalance);
        }

        // Perform all transfers
        for (to, amount) in transfers {
            Self::transfer(env, token_contract, from, to, *amount)?;
        }

        Ok(())
    }

    /// Approve spending allowance (for future use)
    pub fn approve(
        env: &Env,
        token_contract: &Address,
        from: &Address,
        spender: &Address,
        amount: i128,
        expiration_ledger: u32
    ) -> Result<(), Error> {
        if amount < 0 {
            return Err(Error::InvalidSalary);
        }

        let token_client = token::Client::new(env, token_contract);
        token_client.approve(from, spender, &amount, &expiration_ledger);

        Ok(())
    }

    /// Burn tokens (for future use in compensation/penalty scenarios)
    pub fn burn(
        env: &Env,
        token_contract: &Address,
        from: &Address,
        amount: i128
    ) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidSalary);
        }

        let token_client = token::Client::new(env, token_contract);
        token_client.burn(from, &amount);

        Ok(())
    }
}

/// Token information structure for caching
#[derive(Clone, Debug)]
pub struct TokenInfo {
    pub name: soroban_sdk::String,
    pub symbol: soroban_sdk::String,
    pub decimals: u32,
}

impl TokenInfo {
    /// Get token information
    pub fn get(env: &Env, token_contract: &Address) -> TokenInfo {
        TokenInfo {
            name: TokenClient::get_name(env, token_contract),
            symbol: TokenClient::get_symbol(env, token_contract),
            decimals: TokenClient::get_decimals(env, token_contract),
        }
    }

    /// Format amount with proper decimals
    pub fn format_amount(&self, amount: i128) -> String {
        // This would typically involve decimal formatting
        // For simplicity, we'll return the raw amount as string
        amount.to_i8()
    }
}

/// Salary calculation utilities
pub struct SalaryCalculator;

impl SalaryCalculator {
    /// Calculate monthly salary from annual
    pub fn monthly_from_annual(annual_salary: i128) -> Result<i128, Error> {
        if annual_salary <= 0 {
            return Err(Error::InvalidSalary);
        }

        annual_salary.checked_div(12).ok_or(Error::InvalidSalary)
    }

    /// Calculate annual salary from monthly
    pub fn annual_from_monthly(monthly_salary: i128) -> Result<i128, Error> {
        if monthly_salary <= 0 {
            return Err(Error::InvalidSalary);
        }

        monthly_salary.checked_mul(12).ok_or(Error::InvalidSalary)
    }

    /// Calculate pro-rated salary for partial periods
    pub fn pro_rated_salary(
        annual_salary: i128,
        days_worked: u32,
        total_days: u32
    ) -> Result<i128, Error> {
        if annual_salary <= 0 || total_days == 0 || days_worked > total_days {
            return Err(Error::InvalidSalary);
        }

        let daily_rate = annual_salary.checked_div(total_days as i128).ok_or(Error::InvalidSalary)?;

        daily_rate.checked_mul(days_worked as i128).ok_or(Error::InvalidSalary)
    }
}
