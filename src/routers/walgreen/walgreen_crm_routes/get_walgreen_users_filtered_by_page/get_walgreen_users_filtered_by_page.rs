use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::generic_replies::generic_replies::reply_with_serialized_struct;
use crate::routers::walgreen::walgreen_crm_routes::get_walgreen_users_filtered_by_page::get_walgreen_users_filtered_by_page_sql::get_walgreen_users_filtered_by_page_sql;
use crate::routers::walgreen::walgreen_crm_routes::get_walgreen_users_filtered_by_page::total_rows_in_walgreen_users_filtered_sql::total_rows_in_walgreen_users_filtered_sql;
use crate::structs::structs::PageRequestFiltered;
use crate::structs::tool_functions::extract_u32;

pub async fn get_walgreen_users_filtered_by_page(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<PageRequestFiltered>) -> impl IntoResponse {
    match extract_u32(body.page_number) {
        Ok(page_number) => {
            match extract_u32(body.rows_per_page) {
                Ok(rows_per_page) => {
                    let mut unlocked = pool.lock().await;
                    match get_walgreen_users_filtered_by_page_sql(page_number, rows_per_page, &body.filter_type, &body.filter_query, &mut unlocked) {
                        Ok(value) => {
                            match total_rows_in_walgreen_users_filtered_sql(&mut unlocked, &body.filter_type, &body.filter_query) {
                                Ok(counter) => {
                                    reply_with_serialized_struct(true, counter, value)
                                }
                                Err(err) => {
                                    reply_with_serialized_struct(false, err, Vec::new())
                                }
                            }
                        }
                        Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
                    }
                }
                Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
            }
        }
        Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
    }
}