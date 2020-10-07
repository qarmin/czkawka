#[derive(Default)]
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
        if !self.messages.is_empty() {
            println!("-------------------------------MESSAGES--------------------------------");
            for i in &self.messages {
                println!("{}", i);
            }
            println!("---------------------------END OF MESSAGES-----------------------------");
        }

        if !self.warnings.is_empty() {
            println!("-------------------------------WARNINGS--------------------------------");

            for i in &self.warnings {
                println!("{}", i);
            }
            println!("---------------------------END OF WARNINGS-----------------------------");
        }

        if !self.errors.is_empty() {
            println!("--------------------------------ERRORS---------------------------------");

            for i in &self.errors {
                println!("{}", i);
            }
            println!("----------------------------END OF ERRORS------------------------------");
        }
    }
}
