#[path = "student_mod.rs"] mod student_mod;
#[path = "course_mod.rs"] mod course_mod;
use super::*;
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
//create student
//get student name
//get student date of birth
//get student grade avg 
//get student courses
//enroll course
//drop course
//add grade
//remove grade
