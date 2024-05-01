use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::write_route::write_route_structs::WriteDataBody;
use crate::mysql::check_form_data::check_before_sending;
use crate::structs::enums::CheckFieldsCase;

pub async fn write_route(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<WriteDataBody>) -> impl IntoResponse {
    match check_before_sending(&body) {
        CheckFieldsCase::Ok => {

        }
        CheckFieldsCase::Email => {}
        CheckFieldsCase::Name => {}
        CheckFieldsCase::AboutCustomer => {}
    }
}