use std::process;

pub struct ArgumentsPair {
    pub command: String,
    pub argument: Option<String>,
}

impl ArgumentsPair {
    pub fn has_command(ar: &[ArgumentsPair], command: &str) -> bool {
        for a in ar {
            if a.command == command {
                return true;
            }
        }
        false
    }
    pub fn get_argument(ar: &[ArgumentsPair], command: &str, can_be_empty: bool) -> String {
        for a in ar {
            if a.command == command {
                if !can_be_empty && a.argument == Option::None {
                    println!("FATAL ERROR: {} commands should have argument passed", command);
                    process::exit(1);
                }
                return match &a.argument {
                    Some(t) => t.clone(),
                    None => "".to_string(),
                };
            }
        }
        panic!("INTERNAL ERROR: Get argument should always return value");
    }
}
