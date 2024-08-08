use crate::wallgreen_bot_server::new_order_preset::get_last_record_sql::BaseSelector;

pub fn format_the_source(source : BaseSelector) -> String {
    if source == BaseSelector::Ugo {
        return "Ugo-Vape".to_string()
    } else {
        return "Walgreen".to_string()
    }
}