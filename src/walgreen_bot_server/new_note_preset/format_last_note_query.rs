use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;

pub fn format_note_sql_query(selector : &BaseSelector) -> String {
    if selector == &BaseSelector::Wallgreen {
        return format!("SELECT id, related_id, text_info, date_time FROM `walgreen_order_notes` WHERE id=(SELECT MAX(id) FROM `walgreen_order_notes`)")
    }
    else {
        return format!("SELECT id, related_id, text_info, date_time FROM `order_notes` WHERE id=(SELECT MAX(id) FROM `order_notes`)")
    }
}