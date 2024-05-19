use std::sync::{Arc, Mutex};
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use crate::axum_routes::routes::walgreen::requests_from_users::get_phone_and_name::get_phone_and_name_structs::GetPhoneAndName;

pub async fn get_phone_and_name(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<GetPhoneAndName>) -> impl IntoResponse {

}