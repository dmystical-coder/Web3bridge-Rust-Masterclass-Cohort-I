// // src/events.rs

// use soroban_sdk::{ contracttype, Symbol, symbol_short, Address, Env };
// use crate::types::EmployeeRank;

// /// Event data structures
// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct InstitutionRegisteredEvent {
//     pub institution_id: Address,
//     pub admin: Address,
// }

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct EmployeeAddedEvent {
//     pub employee_id: Address,
//     pub institution_id: Address,
//     pub rank: EmployeeRank,
//     pub salary: i128,
// }

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct EmployeeUpdatedEvent {
//     pub employee_id: Address,
//     pub old_salary: i128,
//     pub new_salary: i128,
// }

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct EmployeePromotedEvent {
//     pub employee_id: Address,
//     pub old_rank: EmployeeRank,
//     pub new_rank: EmployeeRank,
//     pub old_salary: i128,
//     pub new_salary: i128,
// }

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct EmployeeSuspendedEvent {
//     pub employee_id: Address,
// }

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct EmployeeReactivatedEvent {
//     pub employee_id: Address,
// }

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct EmployeeRemovedEvent {
//     pub employee_id: Address,
// }

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct SalaryPaidEvent {
//     pub employee_id: Address,
//     pub amount: i128,
//     pub timestamp: u64,
// }

// /// Event topics (using symbol_short for efficiency)
// pub struct Topics;

// impl Topics {
//     pub const INSTITUTION_REGISTERED: Symbol = symbol_short!("inst_reg");
//     pub const EMPLOYEE_ADDED: Symbol = symbol_short!("emp_add");
//     pub const EMPLOYEE_UPDATED: Symbol = symbol_short!("emp_upd");
//     pub const EMPLOYEE_PROMOTED: Symbol = symbol_short!("emp_prom");
//     pub const EMPLOYEE_SUSPENDED: Symbol = symbol_short!("emp_susp");
//     pub const EMPLOYEE_REACTIVATED: Symbol = symbol_short!("emp_react");
//     pub const EMPLOYEE_REMOVED: Symbol = symbol_short!("emp_rem");
//     pub const SALARY_PAID: Symbol = symbol_short!("sal_paid");
// }

// /// Events utility struct
// pub struct Events;

// impl Events {
//     /// Institution registered event
//     pub fn institution_registered(env: &Env, institution_id: &Address, admin: &Address) {
//         env.events().publish((Topics::INSTITUTION_REGISTERED,), InstitutionRegisteredEvent {
//             institution_id: institution_id.clone(),
//             admin: admin.clone(),
//         });
//     }

//     /// Employee added event
//     pub fn employee_added(
//         env: &Env,
//         employee_id: &Address,
//         institution_id: &Address,
//         rank: &EmployeeRank,
//         salary: i128
//     ) {
//         env.events().publish((Topics::EMPLOYEE_ADDED,), EmployeeAddedEvent {
//             employee_id: employee_id.clone(),
//             institution_id: institution_id.clone(),
//             rank: rank.clone(),
//             salary,
//         });
//     }

//     /// Employee updated event
//     pub fn employee_updated(env: &Env, employee_id: &Address, old_salary: i128, new_salary: i128) {
//         env.events().publish((Topics::EMPLOYEE_UPDATED,), EmployeeUpdatedEvent {
//             employee_id: employee_id.clone(),
//             old_salary,
//             new_salary,
//         });
//     }

//     /// Employee promoted event
//     pub fn employee_promoted(
//         env: &Env,
//         employee_id: &Address,
//         old_rank: &EmployeeRank,
//         new_rank: &EmployeeRank,
//         old_salary: i128,
//         new_salary: i128
//     ) {
//         env.events().publish((Topics::EMPLOYEE_PROMOTED,), EmployeePromotedEvent {
//             employee_id: employee_id.clone(),
//             old_rank: old_rank.clone(),
//             new_rank: new_rank.clone(),
//             old_salary,
//             new_salary,
//         });
//     }

//     /// Employee suspended event
//     pub fn employee_suspended(env: &Env, employee_id: &Address) {
//         env.events().publish((Topics::EMPLOYEE_SUSPENDED,), EmployeeSuspendedEvent {
//             employee_id: employee_id.clone(),
//         });
//     }

//     /// Employee reactivated event
//     pub fn employee_reactivated(env: &Env, employee_id: &Address) {
//         env.events().publish((Topics::EMPLOYEE_REACTIVATED,), EmployeeReactivatedEvent {
//             employee_id: employee_id.clone(),
//         });
//     }

//     /// Employee removed event
//     pub fn employee_removed(env: &Env, employee_id: &Address) {
//         env.events().publish((Topics::EMPLOYEE_REMOVED,), EmployeeRemovedEvent {
//             employee_id: employee_id.clone(),
//         });
//     }

//     /// Salary paid event
//     pub fn salary_paid(env: &Env, employee_id: &Address, amount: i128, timestamp: u64) {
//         env.events().publish((Topics::SALARY_PAID,), SalaryPaidEvent {
//             employee_id: employee_id.clone(),
//             amount,
//             timestamp,
//         });
//     }
// }

// /// Event filtering helpers
// pub struct EventFilter;

// impl EventFilter {
//     /// Check if an event is employee-related
//     pub fn is_employee_event(topic: &Symbol) -> bool {
//         matches!(
//             *topic,
//             Topics::EMPLOYEE_ADDED |
//                 Topics::EMPLOYEE_UPDATED |
//                 Topics::EMPLOYEE_PROMOTED |
//                 Topics::EMPLOYEE_SUSPENDED |
//                 Topics::EMPLOYEE_REACTIVATED |
//                 Topics::EMPLOYEE_REMOVED
//         )
//     }

//     /// Check if an event is financial
//     pub fn is_financial_event(topic: &Symbol) -> bool {
//         matches!(*topic, Topics::SALARY_PAID | Topics::EMPLOYEE_PROMOTED)
//     }

//     /// Check if an event is administrative
//     pub fn is_admin_event(topic: &soroban_sdk::Symbol) -> bool {
//         matches!(
//             *topic,
//             Topics::INSTITUTION_REGISTERED |
//                 Topics::EMPLOYEE_SUSPENDED |
//                 Topics::EMPLOYEE_REACTIVATED |
//                 Topics::EMPLOYEE_REMOVED
//         )
//     }
// }
