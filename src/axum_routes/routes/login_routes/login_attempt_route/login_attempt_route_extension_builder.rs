use std::sync::{Arc};
use tokio::sync::{RwLock};
use crate::structs::structs::{AdminsData, Token};
#[derive(Clone, Debug)]
pub struct LoginAttemptExtension {
    pub tokens_pool: Arc<RwLock<Vec<Token>>>,
    pub admin_pool : Arc<RwLock<Vec<AdminsData>>>
}