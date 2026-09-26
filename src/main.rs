use nin::app::MyApp;

fn main() -> eframe::Result {
    eframe::run_native(
        "My egui App",
        Default::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
