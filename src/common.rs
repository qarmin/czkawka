use std::time::SystemTime;

pub struct Common();
impl Common {
    pub fn print_time(start_time: SystemTime, end_time: SystemTime, function_name: String) {
        if false {
            println!(
                "Execution of function \"{}\" took {:?}",
                function_name,
                end_time.duration_since(start_time).expect("Time cannot go reverse.")
            );
        }
    }
}
