use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::generic_replies::generic_replies::reply_with_serialized_struct;
use crate::routers::crm::admin_actions_crm_routes::fetch_admins_data::fetch_admins_data_sql::fetch_admins_data_sql;
use crate::structs::structs::{FetchAdminsDataExtension, StealthAuthToken};

pub async fn fetch_admins_data(pool : Extension<FetchAdminsDataExtension>, Json(body) : Json<StealthAuthToken>) -> impl IntoResponse
{
    let unlocked_storage = pool.token_pool.read().await;
    if unlocked_storage.iter().any(|object| object.token == body.token) {
        drop(unlocked_storage);
        let mut unlocked_pool = pool.db_pool.lock().await;
        match fetch_admins_data_sql(&mut unlocked_pool) {
            Ok(result) => {
                reply_with_serialized_struct(true, "Success", result)
            }
            Err(err) => {
                reply_with_serialized_struct(false, err, Vec::new())
            }
        }
    }
    else {
        reply_with_serialized_struct(false, "Invalid token", Vec::new())
    }
}
