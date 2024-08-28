use crate::structs::structs::NoteObjectNotationFull;
use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;

pub fn format_new_note_message(object : &NoteObjectNotationFull, source : &BaseSelector) -> String {
    return format!("Добавлен новый комментарий к заявке №{} с сайта : {}\n\nСодержание : '{}'\n\nВремя добавления : {}", object.related_id, format_the_source(source), object.text_info, object.date_time)
}

fn format_the_source(source: &BaseSelector) -> String {
    if *source == BaseSelector::Wallgreen {
        return String::from("Walgreen")
    }
    else {
        return String::from("Ugo-Vape")
    }
}