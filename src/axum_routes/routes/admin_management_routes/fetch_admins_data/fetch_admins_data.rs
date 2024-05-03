use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::axum_routes::generic_replies::generic_replies::reply_with_serialized_struct;
use crate::axum_routes::routes::admin_management_routes::fetch_admins_data::fetch_admins_data_extension_builder::FetchAdminsDataExtension;
use crate::axum_routes::routes::admin_management_routes::fetch_admins_data::fetch_admins_data_sql::fetch_admins_data_sql;
use crate::axum_routes::routes::login_routes::stealth_login_route::stealth_login_structs::StealthAuthToken;

pub async fn fetch_admins_data(pool : Extension<FetchAdminsDataExtension>, Json(body) : Json<StealthAuthToken>) -> impl IntoResponse
{
    let unlocked_storage = pool.token_pool.read().await;
    if unlocked_storage.iter().any(|object| object.token == body.token) {
        drop(unlocked_storage);
        let mut unlocked_pool = pool.pool.lock().await;
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
