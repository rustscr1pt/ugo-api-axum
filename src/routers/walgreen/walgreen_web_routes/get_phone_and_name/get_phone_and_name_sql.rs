use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::structs::structs::WriteDataBody;

pub fn get_phone_and_name_sql(pool : &mut MutexGuard<PooledConn>, body : [WriteDataBody ; 1]) -> mysql::Result<(), Error> {
    match pool.exec_batch(r"INSERT INTO walgreen_customers_request VALUES (:id, :request_status, :customer_name, :customer_email, :customer_self_description, NOW())",
        body.iter().map(|value| params! {
            "id" => 0,
            "request_status" => "БЕЗ ВНИМАНИЯ".to_string(),
            "customer_name" => &value.name,
            "customer_email" => &value.email,
            "customer_self_description" => &value.about_customer
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