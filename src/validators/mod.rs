pub trait CharValidator: std::fmt::Display {
    fn disable_empty(&self) -> bool {
        true
    }
    fn validate_first_char(&self, c: char) -> Result<(), String>;
    fn validate_char(&self, c: char) -> bool;
    fn try_while_valid<'a>(&self, input: &'a str) -> Result<(&'a str, &'a str), (usize, String)>;
}
