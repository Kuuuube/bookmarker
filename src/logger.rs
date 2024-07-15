use std::{fs::OpenOptions, io::Write};

pub fn log(message: String) {
    let mut log_file = match OpenOptions::new().append(true).create(true).open("log") {
        Ok(ok) => ok,
        Err(err) => {
            log_error("Failed to open log file".to_string(), err.to_string());
            return;
        }
    };

    match log_file.write_all(format!("{}\n", message).as_bytes()) {
        Ok(_) => (),
        Err(err) => {
            log_error("Failed to write to log file".to_string(), err.to_string());
            return;
        }
    };
}

pub fn log_error(message: String, error: String) {
    let mut error_log_file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open("error_log")
    {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to open error log file {}", err);
            return;
        }
    };

    match error_log_file.write_all(format!("{}, Error: {}\n", message, error).as_bytes()) {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to write to error log file {}", err);
            return;
        }
    };
}
