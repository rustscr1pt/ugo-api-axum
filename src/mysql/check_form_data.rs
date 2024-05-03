use crate::axum_routes::routes::orders_routes::write_route::write_route_structs::WriteDataBody;
use crate::structs::enums::CheckFieldsCase;

pub fn check_before_sending(body : &WriteDataBody) -> CheckFieldsCase {
    if !body.email.contains("@") {
        return CheckFieldsCase::Email
    }
    else if body.name.len() <= 1 {
        return CheckFieldsCase::Name
    }
    else if body.about_customer.len() <= 1 {
        return CheckFieldsCase::AboutCustomer
    }
    else {
        return CheckFieldsCase::Ok
    }
}