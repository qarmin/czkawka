#[derive(Debug, Default, Clone)]
pub struct Messages {
    pub messages: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl Messages {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn print_messages(&self) {
        println!("{}", self.create_messages_text());
    }

    pub fn create_messages_text(&self) -> String {
        let mut text_to_return: String = String::new();

        if !self.messages.is_empty() {
            text_to_return += "-------------------------------MESSAGES--------------------------------\n";
            for i in &self.messages {
                text_to_return += i;
                text_to_return += "\n";
            }
            text_to_return += "---------------------------END OF MESSAGES-----------------------------\n";
        }

        if !self.warnings.is_empty() {
            text_to_return += "-------------------------------WARNINGS--------------------------------\n";

            for i in &self.warnings {
                text_to_return += i;
                text_to_return += "\n";
            }
            text_to_return += "---------------------------END OF WARNINGS-----------------------------\n";
        }

        if !self.errors.is_empty() {
            text_to_return += "--------------------------------ERRORS---------------------------------\n";

            for i in &self.errors {
                text_to_return += i;
                text_to_return += "\n";
            }
            text_to_return += "----------------------------END OF ERRORS------------------------------\n";
        }

        text_to_return
    }

    pub fn extend_messages_with(&mut self, messages: Vec<String>, warnings: Vec<String>, errors: Vec<String>) {
        self.messages.extend(messages);
        self.warnings.extend(warnings);
        self.errors.extend(errors);
    }

    pub fn extend_with_another_messages(&mut self, messages: Messages) {
        let (messages, warnings, errors) = (messages.messages, messages.warnings, messages.errors);
        self.messages.extend(messages);
        self.warnings.extend(warnings);
        self.errors.extend(errors);
    }
}
