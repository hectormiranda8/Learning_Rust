use course_mod::Course;
mod course_mod;

type Courses = Vec<Course>;

struct Student_struct {
    name: String,
    date_of_birth: u32,
    grade_avg: f32,
    courses: Courses
}

trait new {
    fn new(name: String, date_of_birth: u32) -> Self;
}

trait enroll {
    fn enroll(&self, code: String) -> bool;
}

trait drop {
    fn drop(&self, code: String) -> bool;
}

trait to_string {

}

impl Student_struct {
    fn calculate_average(&mut self) {
        let mut len_courses = self.courses.len() as f32;
        let mut avg: f32 = 0.0;
    
        for c in self.courses {
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
        return self.name;
    }

    pub fn get_date_of_birth(&self) -> u32 {
        return self.date_of_birth;
    }

    pub fn get_grade_avg(&self) -> f32 {
        return self.grade_avg;
    }

    pub fn get_courses(&self) -> Courses {
        return self.courses;
    }
}

impl new for Student_struct {
    fn new(name: String, date_of_birth: u32) -> Student_struct {
        Student_struct {
            name: name,
            date_of_birth: date_of_birth,
            grade_avg: 0.0,
            courses: Courses::new()
        }
    }
}

impl enroll for Student_struct {
    fn enroll(&self, code: String) -> bool {
        if course_mod::valid_Code(code) {
            let mut newCourse: Course = Course::new(code);
            self.courses.push(newCourse);
            return true;
        }
        else {
            return false;
        }
    }

}

impl drop for Student_struct {
    fn drop(&self, code: String) -> bool {
        if course_mod::valid_Code(code) {
            let mut idx = 0;
            for c in self.courses {
                if c.get_code() == code {
                    self.courses.remove(idx);
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
}

fn main() {
    println!("Compiled.");
}