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
    real_main();

    // When we get here, exit process so Android restarts fresh next time
    std::process::exit(0);
}

pub fn real_main() -> Result<(), Box<dyn Error>> {
    log::info!("real_main calling test_reqwest");
    //let ui = AppWindow::new()?;
    //ui.run()?;
    if let Err(e) = test_reqwest() {
        log::warn!("Error: {:?}", e);
    }
    Ok(())
}

pub fn test_reqwest() -> Result<(), Box<dyn Error>> {
    log::info!("test_main: starting");
    let slint_future = async move {
        log::info!("test_main (future): running");
        match reqwest::get("http://httpbin.org/range/26").await {
            Ok(response) => {
                log::info!("download worked; {}", response.text().await.unwrap());
            },
            Err(e) => {
                log::error!("download error: {:?}", e);
            }
        }
        log::info!("test_main (future): calling quit_event_loop");
        slint::quit_event_loop().unwrap();
    };
    log::info!("test_main: future created");
    // Spawn the future on Slint's event loop
    slint::spawn_local(async_compat::Compat::new(slint_future))?;
    log::info!("test_main: after spawn_local");
    slint::run_event_loop_until_quit()?;
    Ok(())
}
