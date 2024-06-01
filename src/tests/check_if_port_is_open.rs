use port_scanner::scan_port;
use crate::structs::constants::DEPLOY_PORT;

#[test]
fn check_if_port_is_open() -> Result<(), String> {
    return match scan_port(DEPLOY_PORT) {
        true => { Ok(()) }
        false => { Err(format!("Port {} is occupied, please pick another one!", DEPLOY_PORT)) }
    }
}