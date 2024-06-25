use crate::structs::enums::CheckFieldsCase;
use crate::structs::structs::WriteDataBody;

// Check fields before sending data to the server
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