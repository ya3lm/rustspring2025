// Define the Student struct
struct Student {
    major: String,
}

// Higher-order function to update majors
fn update_majors(collection: &mut Vec<Student>, behavior: fn(&mut Student, String), new_major: String) {
    for student in collection.iter_mut() {
        behavior(student, new_major.clone());
    }
}

// First-order function to assign major
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

fn main() {
    // Create a vector of students
    let mut students = vec![
        Student { major: "Undeclared".to_string() },
        Student { major: "Undeclared".to_string() },
        Student { major: "Undeclared".to_string() },
    ];

    // Print initial majors
    println!("Before update:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }

    // Update all students' majors to "Computer Engineering"
    update_majors(&mut students, assign_major, "Computer Engineering".to_string());

    // Print updated majors
    println!("\nAfter update:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }
}