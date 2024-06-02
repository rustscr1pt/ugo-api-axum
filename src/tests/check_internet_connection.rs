use std::fmt::Display;
use online::check;

#[test]
fn check_internet_connection() -> Result<(), String> {
    return match check(None).is_ok() {
        true => { Ok(()) }
        false => { Err("No internet connection".to_string()) }
    }
}