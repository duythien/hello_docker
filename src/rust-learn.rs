//Problem 2:

/* The code below contains a student struct with three fields.
Your taks is to create a hashmap called student_database in the main function,
which will store instances of the Student structure. The keys of the hashmap
should be unique student IDs, represented as integers while the values will be
instances of the student struct.

Implement a function called add_student() that takes the student's ID, name, age, and grade as parameters.
The function should add a new entry to the student_database hashmap if the student ID doesn't already exist.
If the student ID already exists in the hashmap, the function should not add a new entry.

Use the skeleton of the function given below.
*/

use std::collections::HashMap;
struct Student {
    name: String,
    age: i32,
    grade: String,
}

fn add_student(
    student_database: &mut HashMap<i32, Student>,
    id: i32,
    name: String,
    age: i32,
    grade: String,
) -> Result<(), String> {
    // Your code here
    match student_database.contains_key(&id) {
        true => Err(format!("The student already exit with id {:?}", id)),
        false => {
            //let student_database: HashMap<i32, Student> = HashMap::new();

            student_database.insert(
                id,
                Student {
                    name: name,
                    age: age,
                    grade: grade,
                },
            );

            Ok(())
        }
    }
}

fn main() {
    let mut student_database: HashMap<i32, Student> = HashMap::new();
    add_student(
        &student_database,
        1,
        String::from("John"),
        17,
        String::from("Grade 11"),
    );

    // add_student(
    //     &mut student_database,
    //     2,
    //     String::from("Sarah"),
    //     16,
    //     String::from("Grade 10"),
    // );

    // let a = add_student(
    //     &mut student_database,
    //     3,
    //     String::from("Sarah"),
    //     16,
    //     String::from("Grade 12"),
    // )
    // .is_ok();

    // dbg!(a);

    // Printing the student database

    for (id, student) in &student_database {
        println!("Student ID: {}", id);
        println!("Name: {}", student.name);
        println!("Age: {}", student.age);
        println!("Grade: {}", student.grade);
        println!("------------------");
    }
}
