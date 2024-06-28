use crate::structs::constants::{DEPLOY_PORT, STANDARD_IP};

// Check if it's possible to use port for deploying an API
#[test]
fn check_if_port_is_open() -> Result<(), String> {
    match std::net::TcpListener::bind(format!("{}:{}", STANDARD_IP, DEPLOY_PORT)) {
        Ok(_) => {Ok(())}
        Err(err) => {Err(err.to_string())}
    }
}