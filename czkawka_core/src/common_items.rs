use crate::common::Common;
use crate::common_messages::Messages;
use std::time::SystemTime;

#[derive(Default)]
pub struct ExcludedItems {
    pub items: Vec<String>,
}

impl ExcludedItems {
    pub fn new() -> Self {
        Default::default()
    }
    /// Setting excluded items which needs to contains * wildcard
    /// Are a lot of slower than absolute path, so it should be used to heavy
    pub fn set_excluded_items(&mut self, mut excluded_items: String, text_messages: &mut Messages) {
        let start_time: SystemTime = SystemTime::now();

        if excluded_items.is_empty() {
            return;
        }

        excluded_items = excluded_items.replace("\"", "");
        let expressions: Vec<String> = excluded_items.split(',').map(String::from).collect();
        let mut checked_expressions: Vec<String> = Vec::new();

        for expression in expressions {
            let expression: String = expression.trim().to_string();

            if expression == "" {
                continue;
            }
            if expression == "DEFAULT" {
                // TODO add more files by default
                checked_expressions.push("*/.git/*".to_string());
                continue;
            }
            if !expression.contains('*') {
                text_messages.warnings.push("Excluded Items Warning: Wildcard * is required in expression, ignoring ".to_string() + expression.as_str());
                continue;
            }

            checked_expressions.push(expression);
        }
        self.items = checked_expressions;
        Common::print_time(start_time, SystemTime::now(), "set_excluded_items".to_string());
    }
}
