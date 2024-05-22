use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::axum_routes::routes::walgreen::requests_from_users::get_phone_and_name::get_phone_and_name_structs::GetPhoneAndName;

pub fn get_phone_and_name_sql(pool : &mut MutexGuard<PooledConn>, body : [GetPhoneAndName ; 1]) -> mysql::Result<(), Error> {
    match pool.exec_batch(r"INSERT INTO walgreen_customers_request VALUES (:id, :customer_name, :customer_phone_email, :customer_comment, NOW())",
        body.iter().map(|value| params! {
            "id" => 0,
            "customer_name" => &value.customer_name,
            "customer_phone_email" => &value.customer_phone_email,
            "customer_comment" => &value.customer_comment
        })
    ) {
        Ok(_) => {
            return Ok(())
        }
        Err(err) => {
            return Err(err)
        }
    }
}