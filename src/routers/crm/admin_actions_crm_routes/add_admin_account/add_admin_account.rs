use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::generic_replies::generic_replies::reply_with_message;
use crate::routers::crm::admin_actions_crm_routes::add_admin_account::add_admin_account_sql::add_admins_account_sql;
use crate::structs::structs::{AddAdminAccountExtensionBuilder, AddAdminRequest};
use crate::structs::tool_functions::token_check_before_action;

pub async fn add_admin_account(pool : Extension<AddAdminAccountExtensionBuilder>, Json(body) : Json<AddAdminRequest>) -> impl IntoResponse {
    let read_tokens = pool.token_pool.read().await;
    match token_check_before_action(read_tokens, body.token) {
        true => {
            let mut unlocked = pool.db_pool.lock().await;
            match add_admins_account_sql(&mut unlocked, body.login, body.password) {
                Ok(()) => {
                    reply_with_message(true, "Account has been added to base")
                }
                Err(err) => {
                    reply_with_message(false, err)
                }
            }
        }
        false => {
            reply_with_message(false, "expired token")
        }
    }
}