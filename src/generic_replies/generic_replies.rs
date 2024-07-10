use std::fmt::{Debug, Display};
use axum::Json;
use serde::Serialize;
use crate::structs::structs::{Message, ReplyWithStruct};

// Reject any connections that don't match filters.
pub async fn reject_unmatched_connection() -> Json<Message> {
    return Json(Message {
        is_succeed: false,
        message: "Missing required header-s or JSON body, your connection is dropped".to_string(),
    })
}

// Standard reply to customer's request at the main page
pub fn reply_with_message<T>(condition : bool, message : T) -> Json<Message>
    where T : Display
{
    return Json(Message {
        is_succeed: condition,
        message: message.to_string(),
    })
}

// Reply with a data formatted inside a Vec<T>
pub fn reply_with_serialized_struct<T : Debug + Serialize, A : Display>(condition : bool, message : A, json_reply : Vec<T>) -> Json<ReplyWithStruct<T>>
{
    match condition {
        true => {
            return Json(ReplyWithStruct::<T> {
                is_succeed: true,
                message: message.to_string(),
                reply: json_reply,
            })
        }
        false => {
            return Json(ReplyWithStruct::<T>{
                is_succeed: false,
                message : message.to_string(),
                reply: json_reply,
            })
        }
    }
}
