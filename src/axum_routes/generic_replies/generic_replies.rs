use std::fmt::{Debug, Display};
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;
use crate::structs::constants::CORS_ROUTE;
use crate::structs::structs::{Message, ReplyWithStruct};

pub fn release_headers_for_reply() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", CORS_ROUTE.parse().unwrap());
    return headers
}

// Reject any connections that don't match filters.
pub async fn reject_unmatched_connection() -> (HeaderMap, Json<Message>) {
    return (
            release_headers_for_reply(),
            Json(Message {
                is_succeed: false,
                message: "This request is forbidden, your connection is dropped".to_string(),
            })
        )
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

pub fn reply_with_serialized_struct<T : Debug + Serialize, A : Display>(condition : bool, message : A, json_reply : Vec<T>) -> (HeaderMap, Json<ReplyWithStruct<T>>)
{
    match condition {
        true => {
            return (release_headers_for_reply(), Json(ReplyWithStruct::<T> {
                is_succeed: true,
                message: message.to_string(),
                reply: json_reply,
            }))
        }
        false => {
            return (release_headers_for_reply(), Json(ReplyWithStruct::<T>{
                is_succeed: false,
                message : message.to_string(),
                reply: json_reply,
            }))
        }
    }
}
