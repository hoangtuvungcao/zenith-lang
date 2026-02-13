// Windows-specific Zenith UI implementation
// Uses DirectX/OpenGL backend via eframe

use eframe::egui;
pub use zenith_runtime::{Event, Value};

pub struct WindowsRenderer {
    ui_root: Value,
}

impl WindowsRenderer {
    pub fn new(ui_root: Value) -> Self {
        Self { ui_root }
    }

    pub fn run(self) -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([800.0, 600.0])
                .with_title("Zenith App"),
            renderer: eframe::Renderer::Glow,
            ..Default::default()
        };

        eframe::run_native("Zenith", options, Box::new(|_cc| Box::new(self)))
    }
}

impl eframe::App for WindowsRenderer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Windows UI - WIP");
            // Rendering logic here (same as Linux)
        });
    }
}
