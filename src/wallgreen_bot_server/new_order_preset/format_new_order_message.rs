use crate::structs::structs::BasicPartGetAll;
use crate::wallgreen_bot_server::new_order_preset::get_last_record_sql::BaseSelector;

pub fn format_new_order_message(object : &BasicPartGetAll, source : BaseSelector) -> String {
    return format!("Принята новая заявка с сайта : {}\n\nПорядковый номер : {}\nУстановлен статус : {}\n\nИмя заказчика : {}\nПочтовый адрес : {}\nОписание заявки : {}\n\nВремя добавления : {}",
                   format_source(source), object.id, object.request_status, object.customer_name, object.customer_email, object.customer_self_description, object.date_time_added)
}

fn format_source(source : BaseSelector) -> String {
    if source == BaseSelector::Ugo {
        return "Ugo-Vape".to_string()
    } else {
        return "Walgreen".to_string()
    }
}