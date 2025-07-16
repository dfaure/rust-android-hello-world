use std::error::Error;

use rust_android_hello_world::real_main;

fn main() -> Result<(), Box<dyn Error>> {
    // Log everything to stderr
    // (I use flexi_logger because on Android I need to log to a file)
    let _logger = flexi_logger::Logger::with(flexi_logger::LevelFilter::Debug).start().unwrap();

    real_main()
}
