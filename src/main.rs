use app::main_window::MainWindow;
use egui::Vec2;
use single_instance::SingleInstance;

mod app;
const APP_NAME: &str = "convers_prompt";

fn main() {
    env_logger::init();
    let instance = SingleInstance::new("convers_prompt").unwrap();

    // Check if another instance is already running
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size(Vec2::new(400., 300.))
            .with_always_on_top(),
        ..Default::default()
    };
    if instance.is_single() {
        let _ = eframe::run_native(
            APP_NAME,
            options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);

                Ok(Box::<MainWindow>::default())
            }),
        );
    }
}
