pub fn get_email_for_student(student_id: &str) -> String {
    format!("U{student_id}@unimail.hud.ac.uk")
}
