use std::fs;
use mysql::Pool;
use crate::structs::constants::FILE_LOCATION;

// Check if it's possible to establish the connection with db
#[test]
fn check_connection_mysql() -> Result<(), String> {
    return match fs::read_to_string(FILE_LOCATION) {
        Ok(result) => {
            match Pool::new(result.trim()) {
                Ok(pool) => {
                    match pool.get_conn() {
                        Ok(connection) => {
                            drop(connection);
                            Ok(())
                        }
                        Err(err) => { Err(err.to_string()) }
                    }
                }
                Err(err) => { Err(err.to_string()) }
            }
        }
        Err(err) => { Err(err.to_string()) }
    }
}