//! Messages: Utility for collecting and printing messages, warnings, and errors.

/// Stores messages, warnings, and errors for reporting.
#[derive(Debug, Default, Clone)]
pub struct Messages {
    /// Informational messages.
    pub messages: Vec<String>,
    /// Warning messages.
    pub warnings: Vec<String>,
    /// Error messages.
    pub errors: Vec<String>,
}

impl Messages {
    /// Creates a new, empty `Messages` struct.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new `Messages` struct with errors.
    pub fn new_from_errors(errors: Vec<String>) -> Self {
        Self { errors, ..Default::default() }
    }

    /// Creates a new `Messages` struct with warnings.
    pub fn new_from_warnings(warnings: Vec<String>) -> Self {
        Self { warnings, ..Default::default() }
    }

    /// Creates a new `Messages` struct with messages.
    pub fn new_from_messages(messages: Vec<String>) -> Self {
        Self { messages, ..Default::default() }
    }

    /// Prints all messages, warnings, and errors to the provided writer.
    pub fn print_messages_to_writer<T: std::io::Write>(&self, writer: &mut T) -> std::io::Result<()> {
        let text = self.create_messages_text();
        writer.write_all(text.as_bytes())
    }

    /// Creates a formatted string containing all messages, warnings, and errors.
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

    /// Extends this `Messages` struct with another, appending all messages, warnings, and errors.
    pub fn extend_with_another_messages(&mut self, messages: Self) {
        let (messages, warnings, errors) = (messages.messages, messages.warnings, messages.errors);
        self.messages.extend(messages);
        self.warnings.extend(warnings);
        self.errors.extend(errors);
    }
}
