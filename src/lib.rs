use std::error::Error;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(_app: android_activity::AndroidApp) -> Result<(), Box<dyn Error>> {
    flexi_logger::Logger::with(flexi_logger::LevelFilter::Info)
        .log_to_file(flexi_logger::FileSpec::try_from("/sdcard/Download/hello_logs.txt")?)
        .start()?;

    real_main()
}

pub fn real_main() -> Result<(), Box<dyn Error>> {
    log::info!("Rust Android Hello World");

    std::process::exit(0);
    //Ok(())
}
