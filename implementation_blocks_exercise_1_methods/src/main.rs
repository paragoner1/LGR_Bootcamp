// Complete the method signatures by providing appropriate arguments.

struct Student {
    first_name: String,
    last_name: String,
    roll_no: u16,
}

impl Student {
    fn get_name(&self) -> String {//reference Student without taking ownership
        format!("{} {}", self.first_name, self.last_name)
    }
    fn set_roll_no(&mut self, new_roll_no: u16) {// &mut self to allow mutation of the instance
        self.roll_no = new_roll_no;
    }
    fn convert_to_string(self) -> String { // take ownership so can't be called again on the same instance
        format!(
            "Name: {} {}, Roll no: {}",
            self.first_name, self.last_name, self.roll_no
        )
    }
}

fn main() {
    let mut student = Student {
        first_name: "Harry".to_string(),
        last_name: "Potter".to_string(),
        roll_no: 42,
    };
    println!("Student is: {}", student.get_name());
    student.set_roll_no(50);
    let student_details = student.convert_to_string();
    println!("{student_details}");
}
