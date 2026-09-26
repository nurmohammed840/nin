pub mod app;

pub mod discover;
pub mod file;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    // android_logger::init_once(
    //     android_logger::Config::default()
    //         .with_max_level(log::LevelFilter::Info),
    // );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "Nin",
        options,
        Box::new(|_cc| Ok(Box::new(app::MyApp::default()))),
    )
    .unwrap();
}
