use std::sync::{Arc};
use tokio::sync::RwLock;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::structs::structs::Token;

#[derive(Debug, Clone)]
pub struct AddAdminAccountExtensionBuilder {
    pub db_pool : Arc<Mutex<PooledConn>>,
    pub token_pool : Arc<RwLock<Vec<Token>>>
}