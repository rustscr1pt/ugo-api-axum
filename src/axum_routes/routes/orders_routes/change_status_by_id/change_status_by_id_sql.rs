use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::axum_routes::routes::orders_routes::change_status_by_id::change_status_by_id_structs::InsertStruct;

pub fn change_status_by_id_sql(unlocked : &mut MutexGuard<PooledConn>, id : u16, new_status : String) -> mysql::Result<(), Error> {
    let mut to_insert : Vec<InsertStruct> = Vec::with_capacity(1);
    to_insert.push(InsertStruct {
        id,
        new_status,
    });
    match unlocked.exec_batch(r"UPDATE ugo_customers_request SET request_status = :status WHERE id = :id",
                              to_insert.iter().map(|value| params! {
            "status" => &value.new_status,
            "id" => value.id
        })
    ) {
        Ok(_) => {
            return Ok(())
        }
        Err(e) => {
            return Err(e)
        }
    }
}