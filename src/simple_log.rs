use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Once;

static INIT_LOG_FILE: Once = Once::new();

const LOG_PATH: &str = "/sdcard/Download/helloworld_log.txt";

fn init_log_file() {
    // Truncate the file once at app start
    let _ = File::create(LOG_PATH);
}

pub fn write_log_line(line: String) {
    INIT_LOG_FILE.call_once(init_log_file);

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(LOG_PATH)
    {
        let _ = writeln!(file, "{}", line);
    }
}

/// Logs a line to the log file with an ISO timestamp
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let timestamp = chrono::Local::now().format("%+").to_string();
        $crate::simple_log::write_log_line(format!("{} {}", timestamp, format!($($arg)*)));
    }};
}
