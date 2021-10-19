type Grades = Vec<u32>;

pub struct Course {
    code: String,
    grade_avg: f32,
    grades: Grades
}

impl Course {
    fn calculate_average(&mut self) {
        let mut avg: u32 = 0.0 as u32;
        let length = self.grades.len() as u32;
        for g in &self.grades {
            avg += g;
        }
        self.grade_avg = (avg / length) as f32;
    }

    pub fn new (code: String) -> Course {
        Course {
            code: code,
            grade_avg: 0.0,
            grades: Grades::with_capacity(100)
        }
    }

    pub fn get_code (&self) -> String {
        return self.code.clone();
    }

    pub fn get_grade_avg(&self) -> f32 {
        return self.grade_avg;
    }

    pub fn get_grades(&self) -> &Grades {
        return &self.grades;
    }

    pub fn add_grade(&mut self, grade: u32) -> bool {
        self.grades.push(grade);
        self.calculate_average();
        return true;
    }

    pub fn remove_grade(&mut self, grade_idx: u32) -> bool {
        if grade_idx >= self.grades.len() as u32 {
            return false;
        }
        self.grades.remove(grade_idx as usize);
        self.calculate_average();
        return true;
    }
}

pub fn valid_code(code: String) -> bool{
    // code must be length 8
    if code.len() != 8 {
        return false;
    }

    // first 4 must be characters
    for n in 0..4 {
        let mut ch: char = code.as_bytes()[n] as char;
        if (ch >= 'a' && ch <= 'z') || 
            (ch >= 'A' && ch <= 'Z') {
            // pass
        } else {
            return false;
        }
    }
    // last 4 must be digits
    for n in 4..8 {
        let mut ch: char = code.as_bytes()[n] as char;
        if ch >= '0' && ch <= '9' {
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