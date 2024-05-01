use std::fmt::Display;
use axum::http::HeaderMap;
use axum::Json;
use crate::structs::constants::CORS_ROUTE;
use crate::structs::structs::Message;

pub fn release_headers_for_reply() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", CORS_ROUTE.parse().unwrap());
    return headers
}

// Standard reply to customer's request at the main page
pub fn reply_with_message<T>(condition : bool, message : T) -> (HeaderMap, Json<Message>)
    where T : Display
{
    return (release_headers_for_reply(), Json(Message {
        is_succeed: condition,
        message: message.to_string(),
    }))
}
