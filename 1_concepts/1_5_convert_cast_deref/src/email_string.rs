use std::ops::Deref;
use std::ops::DerefMut;

use regex::Regex;

const VALID_EMAIL: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

pub struct EmailString(String);

impl TryFrom<String> for EmailString {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let regex = Regex::new(VALID_EMAIL).unwrap();
        if regex.is_match(&value) {
            Ok(EmailString(value))
        } else {
            Err("invalid email")
        }
    }
}

impl TryFrom<&str> for EmailString {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let regex = Regex::new(VALID_EMAIL).unwrap();
        if regex.is_match(value) {
            Ok(EmailString(value.to_owned()))
        } else {
            Err("invalid email")
        }
    }
}

impl Deref for EmailString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EmailString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}