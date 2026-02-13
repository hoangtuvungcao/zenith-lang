use egui::{Context, Ui};
use std::sync::mpsc::Sender;
use zenith_runtime::{Event, Value};

pub struct ZenithRenderer {
    ui_root: Value,
    event_sender: Sender<Event>,
}

impl ZenithRenderer {
    pub fn new(ui_root: Value, event_sender: Sender<Event>) -> Self {
        Self {
            ui_root,
            event_sender,
        }
    }

    pub fn render(&self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_widget(ui, &self.ui_root);
        });
    }

    fn render_widget(&self, ui: &mut Ui, widget: &Value) {
        match widget {
            Value::Object(obj) => {
                let map = obj.lock().unwrap();
                let r#type = map.get("type").and_then(|v| match v {
                    Value::String(s) => Some(s.as_str()),
                    _ => None,
                });

                match r#type {
                    Some("Text") => {
                        let text = map.get("text").map(|v| v.to_string()).unwrap_or_default();
                        ui.label(text);
                    }
                    Some("TextField") => {
                        let initial_text =
                            map.get("value").map(|v| v.to_string()).unwrap_or_default();
                        let mut text = initial_text.clone();
                        if ui.text_edit_singleline(&mut text).changed() {
                            if let Some(Value::String(id)) = map.get("on_change") {
                                let _ = self.event_sender.send(Event::Input {
                                    id: id.clone(),
                                    value: text,
                                });
                            }
                        }
                    }
                    Some("Button") => {
                        let label = map
                            .get("label")
                            .map(|v| v.to_string())
                            .unwrap_or("Button".to_string());
                        if ui.button(label).clicked() {
                            if let Some(Value::String(id)) = map.get("on_click") {
                                let _ = self.event_sender.send(Event::Click { id: id.clone() });
                            } else {
                                println!("Button clicked, but no on_click handler provided.");
                            }
                        }
                    }
                    Some("Column") => {
                        ui.vertical(|ui| {
                            if let Some(Value::Array(children)) = map.get("children") {
                                let children_vec = children.lock().unwrap();
                                for child in children_vec.iter() {
                                    self.render_widget(ui, child);
                                }
                            }
                        });
                    }
                    Some("Row") => {
                        ui.horizontal(|ui| {
                            if let Some(Value::Array(children)) = map.get("children") {
                                let children_vec = children.lock().unwrap();
                                for child in children_vec.iter() {
                                    self.render_widget(ui, child);
                                }
                            }
                        });
                    }
                    Some("Container") => {
                        if let Some(child) = map.get("child") {
                            self.render_widget(ui, child);
                        }
                    }
                    Some("Center") => {
                        ui.vertical_centered(|ui| {
                            if let Some(child) = map.get("child") {
                                self.render_widget(ui, child);
                            }
                        });
                    }
                    _ => {
                        ui.label(format!("Unknown widget type: {:?}", r#type));
                    }
                }
            }
            Value::String(s) => {
                ui.label(s);
            }
            _ => {
                ui.label(format!("Unsupported widget: {}", widget));
            }
        }
    }
}
