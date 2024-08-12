use crate::structs::structs::BasicPartGetAll;
use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;
use crate::walgreen_bot_server::new_order_preset::format_the_source::format_the_source;

pub fn format_new_order_message(object : &BasicPartGetAll, source : BaseSelector) -> String {
    return format!("Принята новая заявка с сайта : {}\n\nПорядковый номер : {}\nУстановлен статус : {}\n\nИмя заказчика : {}\nПочтовый адрес : {}\nОписание заявки : {}\n\nВремя добавления : {}\n\nБольше информации в панели по ссылке : https://ugo-vape.ru/__admin-panel/",
                   format_the_source(source), object.id, object.request_status, object.customer_name, object.customer_email, object.customer_self_description, object.date_time_added)
}