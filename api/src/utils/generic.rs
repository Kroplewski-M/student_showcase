pub fn get_email_for_student(student_id: &str) -> String {
    format!("U{student_id}@unimail.hud.ac.uk")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_email_correctly() {
        let email = get_email_for_student("1234567");
        assert_eq!(email, "U1234567@unimail.hud.ac.uk");
    }

    #[test]
    fn prepends_u_prefix() {
        let email = get_email_for_student("0000000");
        assert!(email.starts_with('U'));
    }

    #[test]
    fn uses_correct_domain() {
        let email = get_email_for_student("1234567");
        assert!(email.ends_with("@unimail.hud.ac.uk"));
    }

    #[test]
    fn empty_id_still_formats() {
        let email = get_email_for_student("");
        assert_eq!(email, "U@unimail.hud.ac.uk");
    }
}
