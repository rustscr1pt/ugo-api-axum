use std::fmt::Display;
use online::check;

#[test]
fn check_internet_connection<T : Display>() -> Result<(), T> {
    return match check(None).is_ok() {
        true => { Ok(()) }
        false => { Err("No internet connection") }
    }
}