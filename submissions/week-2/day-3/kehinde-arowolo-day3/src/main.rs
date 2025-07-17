use std::fmt;

#[derive(Debug, Clone)]
enum EmployeeType {
    Media,
    ITDepartment,
    Manager,
    SocialMedia,
    Technician,
    KitchenStaff,
}

#[derive(Debug)]
struct Employee {
    employee_type: EmployeeType,
    is_employed: bool,
}

#[derive(Debug)]
enum AccessError {
    Terminated,
    Unauthorized,
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessError::Terminated => write!(f, "Access denied: Employee is terminated"),
            AccessError::Unauthorized => write!(f, "Access denied: Employee role is not authorized"),
        }
    }
}

fn check_building_access(employee: &Employee) -> Result<(), AccessError> {
    if !employee.is_employed {
        return Err(AccessError::Terminated);
    }
    
    match employee.employee_type {
        EmployeeType::Media | EmployeeType::ITDepartment | EmployeeType::Manager => {
            Ok(())
        }
        EmployeeType::SocialMedia | EmployeeType::Technician | EmployeeType::KitchenStaff => {
            Err(AccessError::Unauthorized)
        }
    }
}

fn print_access_status(employee: &Employee) -> Result<(), AccessError> {
    check_building_access(employee)?; 
    println!(" Employee can access the web3bridge garage");
    Ok(())
}

fn main() {

    let employees = vec![
        Employee {
            employee_type: EmployeeType::Media,
            is_employed: true,
        },
        Employee {
            employee_type: EmployeeType::ITDepartment,
            is_employed: true,
        },
        Employee {
            employee_type: EmployeeType::Manager,
            is_employed: true,
        },
        Employee {
            employee_type: EmployeeType::SocialMedia,
            is_employed: true,
        },
        Employee {
            employee_type: EmployeeType::Technician,
            is_employed: true,
        },
        Employee {
            employee_type: EmployeeType::KitchenStaff,
            is_employed: true,
        },
    ];

    for employee in employees {
        println!("\nTesting {:?} employee:", employee.employee_type);
        match print_access_status(&employee) {
            Ok(()) => {},
            Err(e) => println!("{}", e),
        }
    }
}