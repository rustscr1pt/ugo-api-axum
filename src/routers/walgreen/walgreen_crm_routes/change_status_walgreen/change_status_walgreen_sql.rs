use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::generic_replies::generic_log_writer::generic_log_writer;
use crate::structs::structs::InsertStructNewStatus;

pub fn change_status_walgreen_sql(unlocked : &mut MutexGuard<PooledConn>, id : u32, new_status : String) -> mysql::Result<(), Error> {
    let mut to_insert : Vec<InsertStructNewStatus> = Vec::with_capacity(1);
    to_insert.push(InsertStructNewStatus {
        id,
        new_status : new_status.clone()
    });
    match unlocked.exec_batch(r"UPDATE walgreen_customers_request SET request_status = :status WHERE id = :id",
        to_insert.iter().map(|value| params! {
            "status" => &value.new_status,
            "id" => value.id
        })
    ) {
        Ok(_) => {
            match generic_log_writer(format!("Изменен статус для заявки номер : {} на '{}' -> Walgreen", id, new_status), unlocked) {
                Ok(_) => {return Ok(())}
                Err(err) => {return Err(err)}
            }
        }
        Err(err) => {return Err(err)}
    }
}