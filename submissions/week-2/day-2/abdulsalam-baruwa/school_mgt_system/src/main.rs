use school_mgt_system::Manager;

fn main() {
    let mut manager = Manager::new();
    manager.register_student("Alice".to_string(), 10);
    manager.register_student("Bob".to_string(), 11);
    manager.register_student("Charlie".to_string(), 12);
    println!(
        "Students: {:?}",
        manager
            .all_students()
            .iter()
            .map(|s| &s.name)
            .collect::<Vec<_>>()
    );
}
