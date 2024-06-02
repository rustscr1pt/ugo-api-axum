use crate::structs::constants::DEPLOY_PORT;

#[test]
fn check_if_port_is_open() -> Result<(), String> {
    match std::net::TcpListener::bind(format!("0.0.0.0:{}", DEPLOY_PORT)) {
        Ok(_) => {Ok(())}
        Err(err) => {Err(err.to_string())}
    }
}