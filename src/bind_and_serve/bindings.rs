use std::fmt::Display;
use tokio::net::TcpListener;
use crate::structs::constants::{DEPLOY_PORT, STANDARD_IP};

pub async fn tokio_bindings() -> Result<TcpListener, impl Display> {
    match TcpListener::bind(format!("{}:{}", STANDARD_IP,  DEPLOY_PORT())).await {
        Ok(listener) => {
            return Ok(listener)
        },
        Err(e) => {
            return Err(e)
        }
    }
}