mod student_mod;
mod course_mod;

fn main() {
    println!("Compiled.\n");

    let name = std::string::String::from("Hector Miranda");
    let bd = 19990217;
    let mut student1: student_mod::Student_struct = student_mod::Student::new(name, bd);

    // name of student
    println!("Name of student: {}", student1.get_name());

    // date of birth of student
    println!("Date of birth: {}", student1.get_date_of_birth());

    // enroll course
    println!("\nEnrolling CIIC4050 and ICOM4060...");
    let name = std::string::String::from("CIIC4050");
    student_mod::Student::enroll(&mut student1, name);
    let name = std::string::String::from("ICOM4060");
    student_mod::Student::enroll(&mut student1, name);

    let s1_courses = student1.get_courses();
    for c in s1_courses {
        println!("Student is enrolled in: {}", c.get_code());
    }

    // drop course
    println!("\nDropping ICOM4060...");
    let name = std::string::String::from("ICOM4060");
    student_mod::Student::drop(&mut student1, name);

    let s1_courses = student1.get_courses();
    for c in s1_courses {
        println!("Student is enrolled in: {}", c.get_code());
    }

    
}