use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::structs::structs::WriteToBaseNewCustomer;

pub fn write_route_sql(unlocked : &mut MutexGuard<PooledConn>, sample_to_write : Vec<WriteToBaseNewCustomer>) -> mysql::Result<(), Error> {
    return unlocked.exec_batch(r"INSERT INTO ugo_customers_request VALUES (:id, :request_status, :customer_name, :customer_email, :customer_self_description, NOW())",
                               sample_to_write.iter().map(|value| params!{
        "id" => value.id,
        "request_status" => &value.request_status,
        "customer_name" => &value.customer_name,
        "customer_email" => &value.customer_email,
        "customer_self_description" => &value.customer_self_description
    }))
}