#[path = "course_mod.rs"] mod course_mod;
// use serde::ser::{Serialize, SerializeStruct, Serializer};

type Courses = Vec<course_mod::Course>;

pub struct Student_struct {
    name: String,
    date_of_birth: u32, // YYYYMMDD
    grade_avg: f32,
    courses: Courses
}

pub trait Student {
    fn new(name: String, date_of_birth: u32) -> Self;
    fn enroll(&mut self, code: String) -> bool;
    fn drop(&mut self, code: String) -> bool;
    fn add_grade(&mut self, code: String, grade: u32) -> bool;
    fn remove_grade(&mut self, code: String, grade_idx: u32) -> bool;
}

impl Student_struct {

    fn calculate_average(&mut self) {
        let mut len_courses = self.courses.len() as f32;
        let mut avg: f32 = 0.0;
    
        for c in &self.courses {
            if c.get_grades().len() == 0 {
                len_courses -= 1.0;
            }
            else {
                avg += c.get_grade_avg();
            }
        }
    
        self.grade_avg = avg / len_courses;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_date_of_birth(&self) -> u32 {
        return self.date_of_birth;
    }

    pub fn get_grade_avg(&self) -> f32 {
        return self.grade_avg;
    }

    pub fn get_courses(&self) -> &Courses {
        return &self.courses;
    }

}

impl Student for Student_struct {

    fn new(name: String, date_of_birth: u32) -> Student_struct {
        Student_struct {
            name: name,
            date_of_birth: date_of_birth,
            grade_avg: 0.0,
            courses: Courses::new()
        }
    }

    fn enroll(&mut self, code: String) -> bool {
        if course_mod::valid_code(code.clone()) {
            let newCourse: course_mod::Course = course_mod::Course::new(code.clone());
            self.courses.push(newCourse);
            return true;
        }
        else {
            return false;
        }
    }

    fn drop(&mut self, code: String) -> bool {
        if course_mod::valid_code(code.clone()) {
            let mut idx = 0;
            for c in &self.courses {
                if c.get_code() == code {
                    self.courses.remove(idx);
                    self.calculate_average();
                    return true;
                }
                idx += 1;
            }
            return false;
        }
        else {
            return false;
        }
    }


    fn add_grade(&mut self, code: String, grade: u32) -> bool {
        if !course_mod::valid_code(code.clone()) {
            return false;
        }

        let mut found = false;
        let mut idx = 0;

        // find course index
        for c in &self.courses {
            if c.get_code() == code {
                found = true;
                break;
            }
            idx += 1;
        }

        if !found {
            return false;
        }

        // add grade to that corresponding course
        self.courses[idx].add_grade(grade);
        self.calculate_average();

        return true;
    }

    fn remove_grade(&mut self, code: String, grade_idx: u32) -> bool {
        if !course_mod::valid_code(code.clone()) {
            return false;
        }

        // find course index
        let mut idx = 0;
        let mut found = false;
        for c in &self.courses {
            if c.get_code() == code {
                found = true;
                break;
            }
            idx += 1;
        }
        if !found {
            return false; // course not registered
        }

        let result = self.courses[idx].remove_grade(grade_idx);
        if result {
            self.calculate_average(); // calc avg again if removed grade succ.
        }

        return result;
    }

}

fn main() {
    println!("Compiled.");
}