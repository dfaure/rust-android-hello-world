use std::error::Error;

slint::include_modules!();

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn Error>> {
    // Log to file, on Android
    flexi_logger::Logger::with(flexi_logger::LevelFilter::Info)
        .log_to_file(flexi_logger::FileSpec::try_from("/sdcard/Download/hello_logs.txt")?)
    .start()?;

    slint::android::init(app).unwrap();
    log::info!("slint::android initialized");
    real_main()
}

pub fn real_main() -> Result<(), Box<dyn Error>> {
    log::info!("Rust Android Hello World");
    let ui = AppWindow::new()?;
    ui.run()?;
    Ok(())
}
