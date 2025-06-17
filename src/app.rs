use eframe::egui;
use crate::ui::ClockUI;

pub struct ClockApp {
    ui: ClockUI,
}

impl ClockApp {
    pub fn new() -> Self {
        Self {
            ui: ClockUI::new(),
        }
    }
}

impl eframe::App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui.render(ctx);
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}