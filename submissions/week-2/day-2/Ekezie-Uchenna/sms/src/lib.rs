

use std::fmt::Display;

#[derive(Debug)]
struct Student{
    name: String,
    grade: u8,
    status: Status
}

#[derive(Debug, Clone, PartialEq)]
enum Status{
    Active,
    Inactive,
}

pub struct School{
    pub students: Vec<Student>
}

impl School{
    pub fn new() -> Self{
        School{
            students: Vec::new()
        }
    }

    pub fn register_student(&mut self, student: Student){
        self.students.push(student);
    }

    pub fn edit_student(&mut self, name: String, grade: u8){
        for student in &mut self.students{
            if student.name == name{
                student.grade = grade;
            }
        }
    }

    pub fn update_status(&mut self, name: String, status: Status){
        for student in &mut self.students{
            if student.name == name{
                student.status = status.clone();
            }
        }
    }

    pub fn delete_student(&mut self, name: String){
        for student in &mut self.students{
            if student.name == name{
                student.status = Status::Inactive;
            }
        }
    }

    pub fn view_students(&self){
        for student in &self.students{
            println!("Name: {:?}, Grade: {:?}, Status: {:?}", student.name, student.grade, student.status);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register_student(){
        let mut school = School::new();
        let student = Student{
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        school.register_student(student);
        assert_eq!(school.students.len(), 1);
    }

    #[test]
    fn test_edit_student(){
        let mut school = School::new();
        let student = Student{
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        school.register_student(student);
        school.edit_student(String::from("Ekezie Uchenna"), 11);
        assert_eq!(school.students[0].grade, 11);
    }

     #[test]

     fn test_update_student(){
        let mut school = School::new();
        let student = Student{
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        school.register_student(student);
        school.update_status(String::from("Ekezie Uchenna"), Status::Inactive);
        assert_eq!(school.students[0].status, Status::Inactive);
     }

     #[test]

     fn test_delete_student(){
        let mut school = School::new();
        let student = Student{
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        school.register_student(student);
        school.delete_student(String::from("Ekezie Uchenna"));
        assert_eq!(school.students[0].status, Status::Inactive);
     }

     fn test_view_student(){
        let mut school = School::new();
        let student1 = Student{
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        let student2 = Student{
            name: String::from("Josh Tutor"),
            grade: 20,
            status: Status::Inactive
        };
        school.register_student(student1);
        school.register_student(student2);

        school.view_students();
     }
}

