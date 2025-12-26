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

pub enum MessageLimit {
    NoLimit,
    Characters(usize),
    Lines(usize),
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

        if !self.errors.is_empty() {
            text_to_return += "--------------------------------ERRORS---------------------------------\n";
            for i in &self.errors {
                text_to_return += i;
                text_to_return += "\n";
            }
            text_to_return += "----------------------------END OF ERRORS------------------------------\n";
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messages_constructors_and_text_formatting() {
        // Test new()
        let msg = Messages::new();
        assert!(msg.messages.is_empty());
        assert!(msg.warnings.is_empty());
        assert!(msg.errors.is_empty());
        assert_eq!(msg.create_messages_text(), "");

        // Test new_from_errors()
        let errors = vec!["Error 1".to_string(), "Error 2".to_string()];
        let msg = Messages::new_from_errors(errors.clone());
        assert_eq!(msg.errors, errors);
        let text = msg.create_messages_text();
        assert!(text.contains("ERRORS"));
        assert!(text.contains("Error 1"));

        // Test new_from_warnings()
        let warnings = vec!["Warning 1".to_string()];
        let msg = Messages::new_from_warnings(warnings.clone());
        assert_eq!(msg.warnings, warnings);
        let text = msg.create_messages_text();
        assert!(text.contains("WARNINGS"));

        // Test new_from_messages()
        let messages = vec!["Message 1".to_string()];
        let msg = Messages::new_from_messages(messages.clone());
        assert_eq!(msg.messages, messages);
        let text = msg.create_messages_text();
        assert!(text.contains("MESSAGES"));

        // Test all types together
        let mut msg = Messages::new();
        msg.messages.push("Info".to_string());
        msg.warnings.push("Warn".to_string());
        msg.errors.push("Err".to_string());
        let text = msg.create_messages_text();
        assert!(text.contains("MESSAGES"));
        assert!(text.contains("Info"));
        assert!(text.contains("WARNINGS"));
        assert!(text.contains("Warn"));
        assert!(text.contains("ERRORS"));
        assert!(text.contains("Err"));
    }

    #[test]
    fn test_extend_and_writer() {
        // Test extend_with_another_messages()
        let mut msg1 = Messages::new();
        msg1.messages.push("Msg1".to_string());
        msg1.warnings.push("Warn1".to_string());
        msg1.errors.push("Err1".to_string());

        let mut msg2 = Messages::new();
        msg2.messages.push("Msg2".to_string());
        msg2.warnings.push("Warn2".to_string());
        msg2.errors.push("Err2".to_string());

        msg1.extend_with_another_messages(msg2);

        assert_eq!(msg1.messages.len(), 2);
        assert_eq!(msg1.warnings.len(), 2);
        assert_eq!(msg1.errors.len(), 2);
        assert!(msg1.messages.contains(&"Msg1".to_string()));
        assert!(msg1.messages.contains(&"Msg2".to_string()));

        // Test print_messages_to_writer()
        let mut buffer = Vec::new();
        let result = msg1.print_messages_to_writer(&mut buffer);
        result.unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Msg1"));
        assert!(output.contains("Warn2"));
        assert!(output.contains("Err1"));
    }
}
