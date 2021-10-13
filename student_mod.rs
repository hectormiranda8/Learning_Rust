use course_mod::Course;

type Courses = Vec<Course>;

pub struct Student {
    name: String,
    date_of_birth: u32,
    grade_avg: f32,
    courses: Courses
}

pub fn get_student_average(courses: Courses) {
    let len_courses = courses.len();
    let mut avg: f32 = 0.0;

    for c in courses {
        avg += c.grade_avg;
    }

    return avg / len_courses;
}

fn main() {
    println!("Compiled.");
}