use std::sync::{Arc, Mutex};

use convers::convert::magic_convert;
use egui::{
    CentralPanel, Color32, FontDefinitions, FontFamily, Frame, Margin, RichText, Rounding,
    ScrollArea, Stroke, Style, TextStyle, Vec2, Vec2b,
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
        style.visuals.widgets.inactive.bg_fill = Color32::from_hex("#1e1e2e").unwrap();
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

        let mut area_rect = None;

        CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(Color32::from_hex("#1e1e2e").unwrap())
                    .inner_margin(Margin::same(5.)),
            )
            .show(ctx, |ui| {
                let input_id = ui.make_persistent_id("input_field");

                Frame::none()
                    .inner_margin(Margin::same(2.))
                    .rounding(Rounding::from(10.))
                    .fill(Color32::from_hex("#1e1e2e").unwrap())
                    .stroke(Stroke::new(1., Color32::from_hex("#cba6f7").unwrap()))
                    .show(ui, |ui| {
                        let input = egui::TextEdit::singleline(&mut self.prompt)
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
                    .show(ui, |ui| {
                        ui.set_width(600.);
                        let response = self.response.lock().unwrap().clone();
                        ui.label(
                            RichText::new(&response)
                                .size(24.)
                                .color(Color32::from_rgb(205, 214, 244)),
                        );
                    });
            });

        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                });
            }
            if i.pointer.any_click() {
                if let Some(rect) = area_rect {
                    if !rect.contains(i.pointer.interact_pos().unwrap_or_default()) {
                        let ctx = ctx.clone();
                        std::thread::spawn(move || {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        });
                    }
                }
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
