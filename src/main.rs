use app::main_window::MainWindow;
use clap::Arg;
use egui::Vec2;
use single_instance::SingleInstance;

mod app;
const APP_NAME: &str = "convers_prompt";

fn main() {
    env_logger::init();
    let instance = SingleInstance::new(APP_NAME).unwrap();
    let matches = clap::Command::new(APP_NAME)
        .version("0.1.6")
        .author("veaquer veaquer@gmail.com")
        .about("Raycast simple prototype")
        .arg(Arg::new("width").short('W').long("width"))
        .arg(Arg::new("height").short('H').long("height"));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size(Vec2::new(
                matches
                    .clone()
                    .get_matches()
                    .get_one("width")
                    .map(|x: &String| x.parse::<f32>().unwrap())
                    .unwrap_or(600.),
                matches
                    .get_matches()
                    .get_one("height")
                    .map(|x: &String| x.parse::<f32>().unwrap())
                    .unwrap_or(400.),
            ))
            .with_resizable(false)
            .with_always_on_top()
            .with_drag_and_drop(true),
        centered: true,
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
