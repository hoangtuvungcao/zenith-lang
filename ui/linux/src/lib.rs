// Linux-specific Zenith UI implementation
// Uses native X11/Wayland backend via eframe

use eframe::egui;
use std::sync::mpsc::{channel, Receiver, Sender};
use zenith_runtime::{Event, Value};

pub struct LinuxRenderer {
    ui_root: Value,
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
}

impl LinuxRenderer {
    pub fn new(ui_root: Value) -> Self {
        let (sender, receiver) = channel();
        Self {
            ui_root,
            event_sender: sender,
            event_receiver: receiver,
        }
    }

    pub fn run(self) -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([800.0, 600.0])
                .with_title("Zenith App"),
            ..Default::default()
        };

        eframe::run_native("Zenith", options, Box::new(|_cc| Box::new(self)))
    }

    fn render_widget(&self, ui: &mut egui::Ui, widget: &Value) {
        match widget {
            Value::Object(map) => {
                let m = map.lock().unwrap();

                if let Some(Value::String(widget_type)) = m.get("type") {
                    match widget_type.as_str() {
                        "Text" => {
                            if let Some(Value::String(text)) = m.get("text") {
                                ui.label(text);
                            }
                        }
                        "Button" => {
                            let label = m
                                .get("label")
                                .and_then(|v| {
                                    if let Value::String(s) = v {
                                        Some(s.as_str())
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or("Button");

                            if ui.button(label).clicked() {
                                if let Some(Value::String(id)) = m.get("on_click") {
                                    let _ = self.event_sender.send(Event::Click { id: id.clone() });
                                }
                            }
                        }
                        "Column" => {
                            ui.vertical(|ui| {
                                if let Some(Value::Array(children)) = m.get("children") {
                                    let children_locked = children.lock().unwrap();
                                    for child in children_locked.iter() {
                                        self.render_widget(ui, child);
                                    }
                                }
                            });
                        }
                        "Row" => {
                            ui.horizontal(|ui| {
                                if let Some(Value::Array(children)) = m.get("children") {
                                    let children_locked = children.lock().unwrap();
                                    for child in children_locked.iter() {
                                        self.render_widget(ui, child);
                                    }
                                }
                            });
                        }
                        "Center" => {
                            ui.vertical_centered(|ui| {
                                if let Some(child) = m.get("child") {
                                    self.render_widget(ui, child);
                                }
                            });
                        }
                        _ => {
                            ui.label(format!("Unknown widget: {}", widget_type));
                        }
                    }
                }
            }
            Value::String(s) => {
                ui.label(s);
            }
            _ => {}
        }
    }
}

impl eframe::App for LinuxRenderer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_widget(ui, &self.ui_root);
        });
    }
}
