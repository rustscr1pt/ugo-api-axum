use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::generic_replies::generic_log_writer::generic_log_writer;
use crate::generic_replies::generic_replies::reply_with_message;
use crate::structs::constants::SESSION_DURATION;
use crate::structs::extension_structs::LoginAttemptExtension;
use crate::structs::structs::{LoginRequestData, Token};
use crate::structs::tool_functions::release_string_uuid;

pub async fn login_attempt_route(main_actor : Extension<LoginAttemptExtension>, Json(body) : Json<LoginRequestData>) -> impl IntoResponse {
    let read_pool = main_actor.admin_pool.read().await;
    for elements in read_pool.iter() {
        if elements.user_login == body.login && elements.user_password == body.password {
            let mut unlocked = main_actor.tokens_pool.write().await;
            let generated_token: String = release_string_uuid();
            unlocked.push(Token {
                token: generated_token.clone(),
                time_remaining: SESSION_DURATION(),
            });
            let mut unlocked = main_actor.db_pool.lock().await;
            match generic_log_writer(format!("Попытка войти с данными : {} - {} => Успешно. Выдан токен : {}.", body.login, body.password, &generated_token), &mut unlocked) {
                Ok(_) => {
                    return reply_with_message(true, &generated_token)
                }
                Err(e) => {
                    return reply_with_message(false, e)
                }
            }
        }
    }
    let mut unlocked = main_actor.db_pool.lock().await;
    match generic_log_writer(format!("Попытка войти с данными : {} - {} => Ошибка. Неверные данные.", body.login, body.password), &mut unlocked) {
        Ok(_) => {
            return reply_with_message(false, "Couldn't find you in a list. Try again")
        }
        Err(e) => {
            return reply_with_message(false, e)
        }
    }
}