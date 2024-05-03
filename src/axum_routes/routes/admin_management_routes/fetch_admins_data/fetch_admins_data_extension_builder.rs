use std::sync::Arc;
use mysql::PooledConn;
use tokio::sync::{Mutex, RwLock};
use crate::structs::structs::Token;

#[derive(Clone)]
pub struct FetchAdminsDataExtension {
    pub pool : Arc<Mutex<PooledConn>>,
    pub token_pool : Arc<RwLock<Vec<Token>>>
}