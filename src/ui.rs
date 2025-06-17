use chrono::Local;
use eframe::egui::{self, CentralPanel, Color32, FontId, RichText};

pub struct ClockUI;

impl ClockUI {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        self.setup_theme(ctx);
        self.render_clock(ctx);
    }

    fn setup_theme(&self, ctx: &egui::Context) {
        ctx.set_visuals(egui::Visuals {
            override_text_color: Some(Color32::from_rgb(0, 255, 0)),
            ..egui::Visuals::dark()
        });
    }

    fn render_clock(&self, ctx: &egui::Context) {
        CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::BLACK))
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    let time_str = Local::now().format("%H:%M:%S").to_string();
                    ui.label(
                        RichText::new(time_str)
                            .font(FontId::proportional(256.0))
                            .strong()
                            .color(Color32::from_rgb(0, 255, 0)),
                    );
                });
            });
    }
}
