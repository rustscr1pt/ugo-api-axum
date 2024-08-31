use std::fs;
use mysql::{Pool, PooledConn};
use crate::structs::constants::FILE_LOCATION;

// Establish connection with the db at the start of execution.

pub fn establish_connection() -> PooledConn { // First action to check the connection and establish a working pool
    let pool = Pool::new(fs::read_to_string(FILE_LOCATION()).unwrap().trim()).expect("Couldn't connect to a base");
    println!("Connection with MySQL pool is established!");
    return pool.get_conn().unwrap();
}