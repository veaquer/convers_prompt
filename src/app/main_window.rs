use std::sync::{Arc, Mutex};

use convers::convert::magic_convert;
use egui::{
    Align, CentralPanel, Color32, FontDefinitions, FontFamily, Frame, Layout, Margin, RichText,
    Rounding, ScrollArea, Stroke, Style, TextStyle, Vec2, Vec2b, Visuals,
};
use tokio::runtime::Runtime;

pub struct MainWindow {
    prompt: String,
    response: Arc<Mutex<String>>,
    runtime: Arc<Runtime>,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            response: Arc::new(Mutex::new(String::new())),
            runtime: Arc::new(Runtime::new().unwrap()),
        }
    }
}

impl eframe::App for MainWindow {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style: Style = (*ctx.style()).clone();
        style.visuals.widgets.inactive.bg_fill = Color32::from_hex("#181825").unwrap();
        style.visuals.widgets.inactive.fg_stroke =
            Stroke::new(1., Color32::from_hex("#cdd6f4").unwrap());
        style.text_styles.insert(
            TextStyle::Name("big".into()),
            egui::FontId::new(24.0, FontFamily::Proportional),
        );
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "agave".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "/usr/share/fonts/AgaveNerdFont-Regular.ttf"
            )),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "agave".to_owned());
        ctx.set_fonts(fonts);
        ctx.set_style(style);
        let mut visuals = Visuals::dark();
        visuals.extreme_bg_color = Color32::from_rgb(255, 255, 255);

        ctx.set_visuals(visuals);
        let mut area_rect = None;

        CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(Color32::from_hex("#181825").unwrap())
                    .inner_margin(Margin::same(5.)),
            )
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    let input_id = ui.make_persistent_id("input_field");

                    Frame::none()
                        .inner_margin(Margin::same(2.))
                        .rounding(Rounding::from(10.))
                        .fill(Color32::from_hex("#181825").unwrap())
                        .stroke(Stroke::new(1., Color32::from_hex("#9399b2").unwrap()))
                        .show(ui, |ui| {
                            let input = egui::TextEdit::singleline(&mut self.prompt)
                                .hint_text(
                                    RichText::new("Your prompt here...")
                                        .color(Color32::from_hex("#a6adc8").unwrap()),
                                )
                                .frame(false)
                                .font(TextStyle::Name("big".into()))
                                .id(input_id);
                            ui.add_sized(Vec2::new(ui.available_width(), 30.), input);
                        });

                    ui.memory_mut(|mem| {
                        mem.request_focus(input_id);
                    });
                    area_rect = Some(ui.min_rect());
                    ui.add_space(10.);
                    ScrollArea::vertical()
                        .animated(true)
                        .auto_shrink(Vec2b::new(true, false))
                        .stick_to_right(true)
                        .stick_to_bottom(false)
                        .show(ui, |ui| {
                            let response = self.response.lock().unwrap().clone();
                            ui.label(
                                RichText::new(&response)
                                    .size(24.)
                                    .color(Color32::from_rgb(205, 214, 244)),
                            );
                        });
                    ui.add_space(10.);
                    Frame::none().show(ui, |ui| {
                        ui.set_min_height(100.);
                        ui.set_width(ui.available_width());
                        ui.label("Press Enter to submit your prompt.");
                    });
                });
            });

        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                });
            }

            if i.key_pressed(egui::Key::Enter) {
                let prompt = self.prompt.clone();
                let response = Arc::clone(&self.response);
                let runtime = Arc::clone(&self.runtime);
                runtime.spawn(async move {
                    match magic_convert(&prompt).await {
                        Ok(resp) => {
                            let mut response_lock = response.lock().unwrap();
                            *response_lock = resp;
                        }
                        Err(e) => {
                            let mut response_lock = response.lock().unwrap();
                            *response_lock = e.to_string();
                        }
                    }
                });
                self.prompt.clear();
            }
        });
    }
}
