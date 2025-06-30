use android_activity::AndroidApp;

mod simple_log;

#[no_mangle]
fn android_main(_app: AndroidApp) {
    log!("Rust Android Hello World");

    std::process::exit(0);
    //loop {
    //    std::thread::sleep(std::time::Duration::from_secs(1));
    //}
}
