use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::generic_replies::generic_replies::reply_with_message;
use crate::mysql::check_form_data::check_before_sending;
use crate::routers::ugo_vape::ugo_vape_web_routes::write_route::write_route_sql::write_route_sql;
use crate::structs::enums::CheckFieldsCase;
use crate::structs::extension_structs::SQLAndTelegramWebExtension;
use crate::structs::structs::{WriteDataBody, WriteToBaseNewCustomer};
use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;
use crate::walgreen_bot_server::new_order_preset::get_last_record_sql::{get_last_record_sql};

pub async fn write_route(main_actor : Extension<SQLAndTelegramWebExtension>, Json(body) : Json<WriteDataBody>) -> impl IntoResponse {
    let cloned_telegram_bot = Arc::clone(&main_actor.telegram_bot);
    match check_before_sending(&body) {
        CheckFieldsCase::Ok => {
            let mut sample_to_write : Vec<WriteToBaseNewCustomer> = Vec::with_capacity(1);
            sample_to_write.push(WriteToBaseNewCustomer {
                id: 0,
                request_status: "БЕЗ ВНИМАНИЯ".to_string(),
                customer_name: body.name,
                customer_email: body.email,
                customer_self_description: body.about_customer
            });
            let mut unlocked = main_actor.arc_sql.lock().await;
            match write_route_sql(&mut unlocked, sample_to_write) // Insert and get a response if it was successful or not.
            {
                Ok(_) => {
                    get_last_record_sql(&mut unlocked, BaseSelector::Ugo, cloned_telegram_bot).await;
                    reply_with_message(true, "Ваш запрос был отправлен! Мы ответим вам как можно скорее.")
                }
                Err(e) => {
                    reply_with_message(false, e)
                }
            }
        }
        CheckFieldsCase::Email => {
            reply_with_message(false, "Проверьте на правильность поле email")
        }
        CheckFieldsCase::Name => {
            reply_with_message(false, "Поле 'имя' должно содержать больше 1 символа")
        }
        CheckFieldsCase::AboutCustomer => {
            reply_with_message(false, "Поле 'о вас' должно содержать больше 1 символа")
        }
    }
}