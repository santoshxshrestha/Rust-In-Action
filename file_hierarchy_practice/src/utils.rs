
    // Function to print a highlighted message
    pub fn print_message(message: &str) {
        println!("*** {} ***", message);
    }

    // Function to log informational messages
    pub fn log_info(info: &str) {
        println!("[INFO]: {}", info);
    }

    // Function to log error messages
    pub fn log_error(error: &str) {
        eprintln!("[ERROR]: {}", error);
    }

    // Function to simulate a delay (for practice, just a loop)
    pub fn simulate_delay() {
        for i in 1..=3 {
            println!("Simulating delay... step {}", i);
        }
    }
