use crate::structs::constants::FILE_LOCATION;

#[test]
fn check_mysql_file_location() -> Result<(), String> {
    return match std::fs::read_to_string(FILE_LOCATION) {
        Ok(_) => {Ok(())}
        Err(err) => {Err(err.to_string())}
    }
}