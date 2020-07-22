// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.
fn example() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}

 // This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// An imaginary magical school has a new report card generation system written in rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5). However, the school also issues alphabetical grades
// (A+ -> F-) and needs to be able to print both types of report card!

// Make the necessary code changes to support alphabetical report cards, thereby making the second
// test pass.
pub enum Grade {
    Numerical(f32),
    Alphabetical(String)
}

pub struct ReportCard {
    pub grade: Grade,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, match &self.grade {
            Grade::Numerical(grade) => grade.to_string(),
            Grade::Alphabetical(grade) => grade.clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

        #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value,  42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numerical(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(report_card.print(), "Tom Wriggle (12) - achieved a grade of 2.1");
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: Grade::Alphabetical(String::from("A+")),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(report_card.print(), "Gary Plotter (11) - achieved a grade of A+");
    }
}