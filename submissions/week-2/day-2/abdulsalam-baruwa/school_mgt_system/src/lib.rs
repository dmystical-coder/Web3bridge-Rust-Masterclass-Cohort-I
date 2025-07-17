#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StudentStatus {
    Active,
    Inactive,
}

pub struct Student {
    pub id: u32,
    pub name: String,
    pub grade: u8,
    pub status: StudentStatus,
}

impl Student {
    // Returns the student's ID.
    pub fn id(&self) -> u32 {
        self.id
    }
}

pub struct Manager {
    students: Vec<Student>,
    next_id: u32,
}

impl Manager {
    // Creates a new, empty Manager.
    pub fn new() -> Self {
        Self {
            students: Vec::new(),
            next_id: 1,
        }
    }

    // Registers a new student as Active with the provided name and grade.
    // Returns the ID and name of the newly registered student.
    pub fn register_student(&mut self, name: String, grade: u8) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        println!("New student registered, {name} with ID, {id}.");

        self.students.push(Student {
            id,
            name,
            grade,
            status: StudentStatus::Active,
        });
        id
    }

    // Edits an existing student's name and/or grade by student ID.
    // Pass Some(value) to update a field or None to leave it unchanged.
    // Returns true on success, false if the student was not found.
    pub fn edit_student(
        &mut self,
        student_id: u32,
        new_name: Option<String>,
        new_grade: Option<u8>,
    ) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == student_id) {
            if let Some(n) = new_name {
                student.name = n;
            }
            if let Some(g) = new_grade {
                student.grade = g;
            }
            return true;
        }
        false
    }

    // Updates the status (Active/Inactive) of an existing student by ID.
    // Returns true on success, false if the student was not found.
    pub fn update_status(&mut self, student_id: u32, status: StudentStatus) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == student_id) {
            student.status = status;
            return true;
        }
        false
    }

    // Deletes a student by ID. Returns true if the student existed and was removed.
    pub fn delete_student(&mut self, student_id: u32) -> bool {
        if let Some(idx) = self.students.iter().position(|s| s.id == student_id) {
            self.students.remove(idx);
            return true;
        }
        false
    }

    // Retrieves a reference to a student by ID (for viewing).
    pub fn view_student(&self, student_id: u32) -> Option<&Student> {
        self.students.iter().find(|s| s.id == student_id)
    }

    // Retrieves all students with the given name.
    pub fn view_students_by_name(&self, name: &str) -> Vec<&Student> {
        self.students.iter().filter(|s| s.name == name).collect()
    }

    // Toggles a student's status between Active and Inactive by ID.
    // Returns true on success, false if the student was not found.
    pub fn toggle_student_status(&mut self, student_id: u32) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == student_id) {
            student.status = match student.status {
                StudentStatus::Active => StudentStatus::Inactive,
                StudentStatus::Inactive => StudentStatus::Active,
            };
            println!(
                "Student ID {} status toggled to {:?}",
                student_id, student.status
            );
            return true;
        }
        false
    }

    // Returns all students with the specified status.
    pub fn get_students_by_status(&self, status: StudentStatus) -> Vec<&Student> {
        self.students
            .iter()
            .filter(|s| s.status == status)
            .collect()
    }

    // Returns a slice of all students currently managed.
    pub fn all_students(&self) -> &[Student] {
        &self.students
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut manager = Manager::new();
        let alice_id = manager.register_student("Alice".to_string(), 10);
        let bob_id = manager.register_student("Bob".to_string(), 9);

        // IDs should be unique and sequential
        assert_eq!(alice_id, 1);
        assert_eq!(bob_id, 2);
        assert_ne!(alice_id, bob_id);

        // Students should be registered with correct data
        let alice = manager.view_student(alice_id).unwrap();
        assert_eq!(alice.name, "Alice");
        assert_eq!(alice.grade, 10);
        assert_eq!(alice.status, StudentStatus::Active);

        // Students with same name should be allowed
        let alice2_id = manager.register_student("Alice".to_string(), 8);
        assert_ne!(alice_id, alice2_id);

        let alices = manager.view_students_by_name("Alice");
        assert_eq!(alices.len(), 2);
    }

    #[test]
    fn test_edit_student() {
        let mut manager = Manager::new();
        let student_id = manager.register_student("Charlie".to_string(), 11);

        // Test editing grade only
        assert!(manager.edit_student(student_id, None, Some(12)));
        let student = manager.view_student(student_id).unwrap();
        assert_eq!(student.name, "Charlie");
        assert_eq!(student.grade, 12);

        // Test editing name only
        assert!(manager.edit_student(student_id, Some("Charles".to_string()), None));
        let student = manager.view_student(student_id).unwrap();
        assert_eq!(student.name, "Charles");
        assert_eq!(student.grade, 12);

        // Test editing both name and grade
        assert!(manager.edit_student(student_id, Some("Chuck".to_string()), Some(10)));
        let student = manager.view_student(student_id).unwrap();
        assert_eq!(student.name, "Chuck");
        assert_eq!(student.grade, 10);

        // Test editing non-existent student
        assert!(!manager.edit_student(999, Some("Nobody".to_string()), Some(5)));
    }

    #[test]
    fn test_update_status() {
        let mut manager = Manager::new();
        let student_id = manager.register_student("Diana".to_string(), 9);

        // Student should start as Active
        assert_eq!(
            manager.view_student(student_id).unwrap().status,
            StudentStatus::Active
        );

        // Update to Inactive
        assert!(manager.update_status(student_id, StudentStatus::Inactive));
        assert_eq!(
            manager.view_student(student_id).unwrap().status,
            StudentStatus::Inactive
        );

        // Update back to Active
        assert!(manager.update_status(student_id, StudentStatus::Active));
        assert_eq!(
            manager.view_student(student_id).unwrap().status,
            StudentStatus::Active
        );

        // Test updating non-existent student
        assert!(!manager.update_status(999, StudentStatus::Inactive));
    }

    #[test]
    fn test_toggle_student_status() {
        let mut manager = Manager::new();
        let student_id = manager.register_student("Eva".to_string(), 11);

        // Student should start as Active
        assert_eq!(
            manager.view_student(student_id).unwrap().status,
            StudentStatus::Active
        );

        // Toggle to Inactive
        assert!(manager.toggle_student_status(student_id));
        assert_eq!(
            manager.view_student(student_id).unwrap().status,
            StudentStatus::Inactive
        );

        // Toggle back to Active
        assert!(manager.toggle_student_status(student_id));
        assert_eq!(
            manager.view_student(student_id).unwrap().status,
            StudentStatus::Active
        );

        // Test toggling non-existent student
        assert!(!manager.toggle_student_status(999));
    }

    #[test]
    fn test_delete_student() {
        let mut manager = Manager::new();
        let student_id = manager.register_student("Frank".to_string(), 8);

        // Student should exist
        assert!(manager.view_student(student_id).is_some());

        // Delete student
        assert!(manager.delete_student(student_id));

        // Student should no longer exist
        assert!(manager.view_student(student_id).is_none());

        // Deleting non-existent student should return false
        assert!(!manager.delete_student(student_id));
        assert!(!manager.delete_student(999));
    }

    #[test]
    fn test_view_student() {
        let mut manager = Manager::new();
        let student_id = manager.register_student("Grace".to_string(), 12);

        // Test viewing existing student
        let student = manager.view_student(student_id).unwrap();
        assert_eq!(student.id, student_id);
        assert_eq!(student.name, "Grace");
        assert_eq!(student.grade, 12);
        assert_eq!(student.status, StudentStatus::Active);

        // Test viewing non-existent student
        assert!(manager.view_student(999).is_none());
    }

    #[test]
    fn test_view_students_by_name() {
        let mut manager = Manager::new();
        let john1_id = manager.register_student("John".to_string(), 9);
        let john2_id = manager.register_student("John".to_string(), 10);
        let jane_id = manager.register_student("Jane".to_string(), 11);

        // Test finding multiple students with same name
        let johns = manager.view_students_by_name("John");
        assert_eq!(johns.len(), 2);
        assert!(johns.iter().any(|s| s.id == john1_id));
        assert!(johns.iter().any(|s| s.id == john2_id));

        // Test finding single student
        let janes = manager.view_students_by_name("Jane");
        assert_eq!(janes.len(), 1);
        assert_eq!(janes[0].id, jane_id);

        // Test finding non-existent name
        let nobodies = manager.view_students_by_name("Nobody");
        assert_eq!(nobodies.len(), 0);
    }

    #[test]
    fn test_get_students_by_status() {
        let mut manager = Manager::new();
        let active1_id = manager.register_student("Active1".to_string(), 9);
        let active2_id = manager.register_student("Active2".to_string(), 10);
        let inactive_id = manager.register_student("Inactive1".to_string(), 11);

        // Make one student inactive
        manager.update_status(inactive_id, StudentStatus::Inactive);

        // Test getting active students
        let active_students = manager.get_students_by_status(StudentStatus::Active);
        assert_eq!(active_students.len(), 2);
        assert!(active_students.iter().any(|s| s.id == active1_id));
        assert!(active_students.iter().any(|s| s.id == active2_id));

        // Test getting inactive students
        let inactive_students = manager.get_students_by_status(StudentStatus::Inactive);
        assert_eq!(inactive_students.len(), 1);
        assert_eq!(inactive_students[0].id, inactive_id);
    }

    #[test]
    fn test_all_students() {
        let mut manager = Manager::new();

        // Empty manager should return empty slice
        assert_eq!(manager.all_students().len(), 0);

        // Add some students
        manager.register_student("Student1".to_string(), 9);
        manager.register_student("Student2".to_string(), 10);
        manager.register_student("Student3".to_string(), 11);

        let all_students = manager.all_students();
        assert_eq!(all_students.len(), 3);
        assert_eq!(all_students[0].name, "Student1");
        assert_eq!(all_students[1].name, "Student2");
        assert_eq!(all_students[2].name, "Student3");
    }

    #[test]
    fn test_student_id_method() {
        let mut manager = Manager::new();
        let student_id = manager.register_student("Test".to_string(), 10);
        let student = manager.view_student(student_id).unwrap();

        // Test the id() method on Student
        assert_eq!(student.id(), student_id);
        assert_eq!(student.id(), student.id);
    }

    #[test]
    fn test_multiple_operations() {
        let mut manager = Manager::new();

        // Register multiple students
        let alice_id = manager.register_student("Alice".to_string(), 10);
        let bob_id = manager.register_student("Bob".to_string(), 9);
        let charlie_id = manager.register_student("Charlie".to_string(), 11);

        // Edit some students
        manager.edit_student(alice_id, Some("Alice Smith".to_string()), Some(11));
        manager.update_status(bob_id, StudentStatus::Inactive);

        // Toggle status
        manager.toggle_student_status(charlie_id);

        // Verify final state
        let alice = manager.view_student(alice_id).unwrap();
        assert_eq!(alice.name, "Alice Smith");
        assert_eq!(alice.grade, 11);
        assert_eq!(alice.status, StudentStatus::Active);

        let bob = manager.view_student(bob_id).unwrap();
        assert_eq!(bob.status, StudentStatus::Inactive);

        let charlie = manager.view_student(charlie_id).unwrap();
        assert_eq!(charlie.status, StudentStatus::Inactive);

        // Check counts by status
        assert_eq!(
            manager.get_students_by_status(StudentStatus::Active).len(),
            1
        );
        assert_eq!(
            manager
                .get_students_by_status(StudentStatus::Inactive)
                .len(),
            2
        );
        assert_eq!(manager.all_students().len(), 3);
    }
}
