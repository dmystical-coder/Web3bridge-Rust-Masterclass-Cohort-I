use soroban_sdk::{contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug)]
pub enum TokenError {
    NegativeAmount = 1,
    InsufficientBalance = 2,
    InsufficientAllowance = 3,
}