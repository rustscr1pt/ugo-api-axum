use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use tokio::sync::RwLock;
use crate::generic_replies::generic_replies::reply_with_message;
use crate::structs::structs::{StealthAuthToken, Token};

pub async fn stealth_login(pool : Extension<Arc<RwLock<Vec<Token>>>>, Json(body) : Json<StealthAuthToken>) -> impl IntoResponse {
    let unlocked = pool.read().await;
    match check_the_token(unlocked.clone(), body.token) {
        true => {reply_with_message(true, "Authorized by token")}
        false => {reply_with_message(false, "Token is not valid")}
    }
}

fn check_the_token(main : Vec<Token>, to_check : String) -> bool {
    let parsed_tokens = main.into_iter().map(|object| object.token).collect::<Vec<String>>();
    for element in parsed_tokens.into_iter() {
        if element == to_check {
            return true
        }
    }
    return false
}