use czkawka_core::helpers::messages::Messages;
use gtk4::TextView;
use gtk4::prelude::*;

use crate::flg;

pub fn print_text_messages_to_text_view(text_messages: &Messages, text_view: &TextView) {
    let mut messages: String = String::new();
    if !text_messages.messages.is_empty() {
        messages += format!("############### {}({}) ###############\n", flg!("text_view_messages"), text_messages.messages.len()).as_str();
    }
    for text in &text_messages.messages {
        messages += text.as_str();
        messages += "\n";
    }
    if !text_messages.warnings.is_empty() {
        messages += format!("############### {}({}) ###############\n", flg!("text_view_warnings"), text_messages.warnings.len()).as_str();
    }
    for text in &text_messages.warnings {
        messages += text.as_str();
        messages += "\n";
    }
    if !text_messages.errors.is_empty() {
        messages += format!("############### {}({}) ###############\n", flg!("text_view_errors"), text_messages.errors.len()).as_str();
    }
    for text in &text_messages.errors {
        messages += text.as_str();
        messages += "\n";
    }
    text_view.buffer().set_text(messages.as_str());
}

pub fn reset_text_view(text_view: &TextView) {
    text_view.buffer().set_text("");
}

pub fn add_text_to_text_view(text_view: &TextView, string_to_append: &str) {
    let buffer = text_view.buffer();
    let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();
    if current_text.is_empty() {
        buffer.set_text(string_to_append);
    } else {
        buffer.set_text(format!("{current_text}\n{string_to_append}").as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use czkawka_core::helpers::messages::Messages;
    use gtk4::TextView;

    fn get_buffer_text(text_view: &TextView) -> String {
        let buffer = text_view.buffer();
        buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string()
    }

    #[gtk4::test]
    fn test_reset_text_view() {
        let text_view = TextView::new();
        text_view.buffer().set_text("Some text");
        reset_text_view(&text_view);
        assert_eq!(get_buffer_text(&text_view), "");
    }

    #[gtk4::test]
    fn test_add_text_to_text_view() {
        let text_view = TextView::new();
        add_text_to_text_view(&text_view, "Line1");
        assert_eq!(get_buffer_text(&text_view), "Line1");
        add_text_to_text_view(&text_view, "Line2");
        assert_eq!(get_buffer_text(&text_view), "Line1\nLine2");
    }

    #[gtk4::test]
    fn test_print_text_messages_to_text_view() {
        let text_view = TextView::new();
        let mut messages = Messages::default();
        messages.messages.push("msg1".to_string());
        messages.warnings.push("warn1".to_string());
        messages.errors.push("err1".to_string());
        print_text_messages_to_text_view(&messages, &text_view);
        let text = get_buffer_text(&text_view);
        assert!(text.contains("msg1"));
        assert!(text.contains("warn1"));
        assert!(text.contains("err1"));
    }
}
