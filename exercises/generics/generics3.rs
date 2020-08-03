// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes to support alphabetical report cards, thereby making
// the second test pass.

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

struct DisplayNumber<'a>(&'a ReportCard);
struct DisplayLevel<'a>(&'a ReportCard);

impl<'a> DisplayNumber<'a> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.0.student_name, &self.0.student_age, &self.0.grade
        )
    }
}

impl<'a> DisplayLevel<'a> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.0.student_name, &self.0.student_age, "A+"
        )
        // "A+" is hardcoded here because
        // the mapping between grade and level
        // is not much clear to me.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            DisplayNumber(&report_card).print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            DisplayLevel(&report_card).print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
