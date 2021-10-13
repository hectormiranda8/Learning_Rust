type Grades = Vec<u32>;

pub struct Course {
    code: String,
    grade_avg: f32,
    grades: Grades
}

pub fn valid_Code(code: String) -> bool{
    // code must be length 8
    if code.len() != 8 {
        return false;
    }

    // first 4 must be characters
    for n in 0..4 {
        let mut ch: char = code.as_bytes()[n] as char;
        if ((ch >= 'a' && ch <= 'z') || 
            (ch >= 'A' && ch <= 'Z')) {
            // pass
        } else {
            return false;
        }
    }
    // last 4 must be digits
    for n in 4..8 {
        let mut ch: char = code.as_bytes()[n] as char;
        if ((ch >= '0' && ch <= '9')) {
            // pass
        } else {
            return false;
        }
    }

    return true;
}

fn main() {
    println!("Compiled.");
}