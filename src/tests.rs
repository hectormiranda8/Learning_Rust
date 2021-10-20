#[path = "student_mod.rs"] mod student_mod;
#[path = "course_mod.rs"] mod course_mod;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[test]
fn test() {
    assert_eq!(1, 1);
}


/* COURSE TESTING */
//test valid_code method
#[test]
fn test_valid_code() {
    let code = std::string::String::from("ICOM4060");
    assert_eq!(course_mod::valid_code(code), true);
    let code = std::string::String::from("ICOM40");
    assert_eq!(course_mod::valid_code(code), false);
}

//create course
#[test]
fn create_course() {
    let code = std::string::String::from("ICOM4060");
    let course = course_mod::Course::new(code);
    assert_eq!(type_of(course), "studentdb::tests::course_mod::Course");
}

//get course code
#[test]
fn get_course_code() {
    let code = std::string::String::from("ICOM4060");
    let course = course_mod::Course::new(code.clone());
    let c_code = course.get_code();
    assert_eq!(code, c_code);
}

//add-remove-get grades, grade avg
#[test]
fn add_grade() {
    let code = std::string::String::from("ICOM4060");
    let mut course = course_mod::Course::new(code);
    let grades = &course.get_grades();
    assert_eq!(grades.len(), 0 as usize);

    course.add_grade(90.0 as u32);
    let grades = &course.get_grades();
    assert_eq!(grades.len(), 1 as usize);
    
    let grade = grades[0];
    assert_eq!(grade, 90.0 as u32);

    course.add_grade(80.0 as u32);
    let grades = &course.get_grades();
    assert_eq!(grades.len(), 2 as usize);
    assert_eq!(grades[1], 80.0 as u32);

    let avg = course.get_grade_avg();
    assert_eq!(avg, 85 as f32);

    course.remove_grade(1 as u32);
    let grades = &course.get_grades();
    assert_eq!(grades.len(), 1 as usize);
    let avg = course.get_grade_avg();
    assert_eq!(avg, 90 as f32);
}


/* STUDENT TESTING */
//create student, get student name, get student date of birth
#[test]
fn create_student() {
    let name = "Hector Miranda".to_string();
    let bd = 19990217;
    let student: student_mod::Student_struct = student_mod::Student::new(name.clone(), bd.clone());
    assert_eq!(name, student.get_name());
    assert_eq!(bd, student.get_date_of_birth());
    assert_eq!(type_of(student), "studentdb::tests::student_mod::Student_struct");
}

//enroll and drop course, add and remove grade, get student courses
//get student grade avg 
#[test]
fn student_functions() {
    let name = "Hector Miranda".to_string();
    let bd = 19990217;
    let mut student: student_mod::Student_struct = student_mod::Student::new(name, bd);
    
    student_mod::Student::enroll(&mut student, "ICOM4060".to_string());

    assert_eq!(student.get_courses().len(), 1 as usize);
    assert_eq!(student.get_courses()[0].get_code(), "ICOM4060".to_string());
    assert_eq!(student.get_courses()[0].get_grade_avg(), 0.0);

    student_mod::Student::enroll(&mut student, "CIIC4050".to_string());

    assert_eq!(student.get_courses().len(), 2 as usize);
    assert_eq!(student.get_courses()[1].get_code(), "CIIC4050".to_string());
    assert_eq!(student.get_courses()[1].get_grade_avg(), 0.0);
    assert_eq!(student.get_grade_avg(), 0.0);

    let result = student_mod::Student::add_grade(&mut student, "ICOM4060".to_string(), 90 as u32);
    assert_eq!(result, true);
    assert_eq!(student.get_courses()[0].get_code(), "ICOM4060".to_string());
    assert_eq!(student.get_courses()[0].get_grade_avg(), 90 as f32);
    assert_eq!(student.get_grade_avg(), 90 as f32);

    let result = student_mod::Student::add_grade(&mut student, "ICOM".to_string(), 90 as u32);
    assert_eq!(result, false);
    let result = student_mod::Student::add_grade(&mut student, "ICOM4050".to_string(), 90 as u32);
    assert_eq!(result, false);

    let result = student_mod::Student::add_grade(&mut student, "ICOM4060".to_string(), 100 as u32);
    assert_eq!(result, true);
    assert_eq!(student.get_courses()[0].get_code(), "ICOM4060".to_string());
    assert_eq!(student.get_courses()[0].get_grade_avg(), 95 as f32);
    assert_eq!(student.get_grade_avg(), 95 as f32);

    let result = student_mod::Student::add_grade(&mut student, "CIIC4050".to_string(), 90 as u32);
    assert_eq!(result, true);
    assert_eq!(student.get_courses()[1].get_code(), "CIIC4050".to_string());
    assert_eq!(student.get_courses()[1].get_grade_avg(), 90 as f32);
    // assert_eq!(student.get_grade_avg(), (((90+100+90) as f32)/3.0) as f32);

    let result = student_mod::Student::drop(&mut student, "ICOM4060".to_string());
    assert_eq!(result, true);
    assert_eq!(student.get_courses().len(), 1 as usize);
    assert_eq!(student.get_grade_avg(), 90 as f32);

    let result = student_mod::Student::remove_grade(&mut student, "CIIC4050".to_string(), 0 as u32);
    assert_eq!(result, true);
    assert_eq!(student.get_grade_avg(), 0 as f32);

}