use std::collections::HashMap;

fn main() {
    let mut students: HashMap<String, u32> = HashMap::new();
    students.insert("Ravi".to_string(), 100);
    students.insert("Raju".to_string(), 100);
    students.insert("Lalu".to_string(), 100);

    for (student, marks) in students.iter() {
        println!("Student name: {:?} marks:{}", student, marks);
    }

    match students.get("Raju") {
        Some(marks) => println!("Found: {}", marks),
        None => println!("Not found"),
    }
}
