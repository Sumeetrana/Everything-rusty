struct Student {
    name: String,
}

trait Name {
    fn name_change(&mut self, new_name: String);
}

impl Name for Student {
    fn name_change(&mut self, new_name: String) {
        self.name = new_name
    }
}

fn main() {
    let mut student = Student {
        name: String::from("Harry"),
    };

    student.name_change(String::from("Ron"));

    println!("Studnet name: {}", student.name)
}
