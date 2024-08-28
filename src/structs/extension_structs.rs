use std::sync::Arc;
use mysql::PooledConn;
use rustelebot::types::BotInstance;
use tokio::sync::{Mutex, RwLock};
use crate::structs::structs::{AdminsData, Token};

#[derive(Clone)]
pub struct SQLAndTelegramWebExtension {
    pub arc_sql : Arc<Mutex<PooledConn>>,
    pub telegram_bot : Arc<Mutex<BotInstance>>
}

#[derive(Clone, Debug)]
pub struct LoginAttemptExtension {
    pub db_pool : Arc<Mutex<PooledConn>>,
    pub tokens_pool: Arc<RwLock<Vec<Token>>>,
    pub admin_pool : Arc<RwLock<Vec<AdminsData>>>
}