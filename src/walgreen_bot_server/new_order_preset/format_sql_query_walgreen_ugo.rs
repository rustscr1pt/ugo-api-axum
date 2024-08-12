use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;
use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector::Wallgreen;

pub fn format_sql_query_walgreen_ugo(selector : &BaseSelector) -> String {
    if selector == &Wallgreen {
        return format!("SELECT id, request_status, customer_name, customer_email, customer_self_description, date_time_added FROM `walgreen_customers_request` WHERE id=(SELECT MAX(id) FROM `walgreen_customers_request`)")
    }
    else {
        return format!("SELECT id, request_status, customer_name, customer_email, customer_self_description, date_time_added FROM `ugo_customers_request` WHERE id=(SELECT MAX(id) FROM `ugo_customers_request`)")
    }
}
