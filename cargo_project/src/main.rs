

// create a student class that has age and name properties
// create a new method that takes in the age and name and returns a new student
// create a method that returns the age of the student

// Path: cargo_project\src\main.rs

pub struct Student {
    age: i32,
    name: String
}

impl Student {
    pub fn new(age: i32, name: String) -> Self {
        Self { age, name }
    }

    pub fn get_age(&self) -> i32 {
        return self.age
    }

    pub fn get_name(&self) -> String {
        return self.name.clone()
    }
}

fn main() {
    let student_a = Student::new(20, "Blake".to_string());

    println!("Name: {} Age: {}", student_a.get_name(), student_a.get_age());
}
