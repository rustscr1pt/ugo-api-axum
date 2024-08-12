use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;

pub fn format_the_source(source : BaseSelector) -> String {
    if source == BaseSelector::Ugo {
        return "Ugo-Vape".to_string()
    } else {
        return "Walgreen".to_string()
    }
}