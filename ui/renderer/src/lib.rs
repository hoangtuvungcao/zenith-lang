use egui::{Color32, RichText, TextEdit, Ui, Vec2};
use std::collections::HashMap;
use zenith_runtime::Value;

// Callback signature: (action_name, optional_payload)
pub type EventHandler = Box<dyn Fn(String, Option<String>)>;

/// Renders a Zenith Value as an Egui widget tree with enhanced styling
pub fn render_ui(ui: &mut Ui, value: &Value, on_event: &EventHandler) {
    match value {
        Value::Object(map) => {
            let m = map.lock().unwrap();
            if let Some(Value::String(widget_type)) = m.get("type") {
                match widget_type.as_str() {
                    // Layouts
                    "Column" => render_column(ui, &m, on_event),
                    "Row" => render_row(ui, &m, on_event),
                    "Center" => render_center(ui, &m, on_event),
                    "Container" => render_container(ui, &m, on_event),
                    "Card" => render_card(ui, &m, on_event),
                    "Spacer" => render_spacer(ui, &m),

                    // Widgets
                    "Text" => render_text(ui, &m),
                    "Heading" => render_heading(ui, &m),
                    "Button" => render_button(ui, &m, on_event),
                    "TextField" => render_text_field(ui, &m, on_event),
                    "TextArea" => render_text_area(ui, &m, on_event),
                    "Checkbox" => render_checkbox(ui, &m, on_event),
                    "ScrollView" => render_scroll_view(ui, &m, on_event),
                    "Grid" => render_grid(ui, &m, on_event),
                    "Slider" => render_slider(ui, &m, on_event),
                    "ProgressBar" => render_progress_bar(ui, &m),
                    "ComboBox" => render_combo_box(ui, &m, on_event),
                    "Tabs" => render_tabs(ui, &m, on_event),
                    "Table" => render_table(ui, &m, on_event),
                    "Window" => render_window(ui, &m, on_event),
                    "CollapsingHeader" => render_collapsing_header(ui, &m, on_event),
                    "MenuBar" => render_menu_bar(ui, &m, on_event),
                    "StatusBar" => render_status_bar(ui, &m, on_event),
                    "SplitPane" => render_split_pane(ui, &m, on_event),
                    "Tree" => render_tree(ui, &m, on_event),
                    "CodeEditor" => render_code_editor(ui, &m, on_event),
                    "Separator" => {
                        ui.separator();
                    }

                    _ => {
                        ui.label(
                            RichText::new(format!("Unknown widget: {}", widget_type))
                                .color(Color32::LIGHT_RED),
                        );
                    }
                }
            } else {
                ui.label(RichText::new("Invalid UI Object: Missing 'type'").color(Color32::RED));
            }
        }
        Value::Array(items) => {
            let arr = items.lock().unwrap();
            for item in arr.iter() {
                render_ui(ui, item, on_event);
            }
        }
        _ => {
            ui.label(format!("{:?}", value));
        }
    }
}

fn get_color(map: &HashMap<String, Value>, key: &str) -> Option<Color32> {
    if let Some(Value::String(color_str)) = map.get(key) {
        parse_color(color_str)
    } else {
        None
    }
}

fn parse_color(color_str: &str) -> Option<Color32> {
    match color_str {
        "#3498db" | "blue" => Some(Color32::from_rgb(52, 152, 219)),
        "#2ecc71" | "green" => Some(Color32::from_rgb(46, 204, 113)),
        "#e74c3c" | "red" => Some(Color32::from_rgb(231, 76, 60)),
        "#f39c12" | "orange" => Some(Color32::from_rgb(243, 156, 18)),
        "#9b59b6" | "purple" => Some(Color32::from_rgb(155, 89, 182)),
        "#34495e" | "dark" => Some(Color32::from_rgb(52, 73, 94)),
        "white" => Some(Color32::WHITE),
        "black" => Some(Color32::BLACK),
        _ => None,
    }
}

// ========== LAYOUTS ==========

fn render_column(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let spacing = if let Some(Value::Integer(s)) = map.get("spacing") {
        *s as f32
    } else {
        5.0
    };

    ui.spacing_mut().item_spacing.y = spacing;

    ui.vertical(|ui| {
        if let Some(children) = map.get("children") {
            render_ui(ui, children, on_event);
        }
    });
}

fn render_row(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let spacing = if let Some(Value::Integer(s)) = map.get("spacing") {
        *s as f32
    } else {
        5.0
    };

    ui.spacing_mut().item_spacing.x = spacing;

    ui.horizontal(|ui| {
        if let Some(children) = map.get("children") {
            render_ui(ui, children, on_event);
        }
    });
}

fn render_center(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    ui.centered_and_justified(|ui| {
        if let Some(child) = map.get("child") {
            render_ui(ui, child, on_event);
        }
    });
}

fn render_container(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let padding = if let Some(Value::Integer(p)) = map.get("padding") {
        *p as f32
    } else {
        10.0
    };

    let mut frame = egui::Frame::none().inner_margin(padding);

    if let Some(color) = get_color(map, "color") {
        frame = frame.fill(color);
    }

    // Support rounding
    if let Some(Value::Integer(r)) = map.get("rounding") {
        frame = frame.rounding(*r as f32);
    }

    frame.show(ui, |ui| {
        if let Some(child) = map.get("child") {
            render_ui(ui, child, on_event);
        }
    });
}

fn render_card(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    egui::Frame::group(ui.style())
        .inner_margin(12.0)
        .outer_margin(4.0)
        .rounding(8.0)
        .fill(Color32::from_rgb(40, 44, 52))
        .show(ui, |ui| {
            if let Some(child) = map.get("child") {
                render_ui(ui, child, on_event);
            }
        });
}

fn render_spacer(ui: &mut Ui, map: &HashMap<String, Value>) {
    let height = if let Some(Value::Integer(h)) = map.get("height") {
        *h as f32
    } else {
        10.0
    };

    ui.add_space(height);
}

// ========== WIDGETS ==========

fn render_text(ui: &mut Ui, map: &HashMap<String, Value>) {
    if let Some(Value::String(text)) = map.get("text") {
        let mut rich_text = RichText::new(text);

        // Size
        if let Some(Value::Integer(size)) = map.get("size") {
            rich_text = rich_text.size(*size as f32);
        }

        // Color
        if let Some(color) = get_color(map, "color") {
            rich_text = rich_text.color(color);
        }

        // Bold
        if let Some(Value::Boolean(true)) = map.get("bold") {
            rich_text = rich_text.strong();
        }

        ui.label(rich_text);
    }
}

fn render_heading(ui: &mut Ui, map: &HashMap<String, Value>) {
    if let Some(Value::String(text)) = map.get("text") {
        let level = if let Some(Value::Integer(l)) = map.get("level") {
            *l
        } else {
            1
        };

        let size = match level {
            1 => 28.0,
            2 => 24.0,
            3 => 20.0,
            _ => 18.0,
        };

        ui.heading(RichText::new(text).size(size).strong());
    }
}

fn render_button(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let label = if let Some(Value::String(text)) = map.get("label") {
        text.clone()
    } else {
        "Button".to_string()
    };

    let mut button = egui::Button::new(label);

    // Min size for better touch targets
    if let Some(Value::Integer(width)) = map.get("min_width") {
        button = button.min_size(Vec2::new(*width as f32, 0.0));
    }

    // Style
    if let Some(Value::String(style)) = map.get("style") {
        button = match style.as_str() {
            "primary" => button.fill(Color32::from_rgb(52, 152, 219)),
            "success" => button.fill(Color32::from_rgb(46, 204, 113)),
            "danger" => button.fill(Color32::from_rgb(231, 76, 60)),
            "warning" => button.fill(Color32::from_rgb(243, 156, 18)),
            _ => button,
        };
    }

    if ui.add(button).clicked() {
        if let Some(Value::String(action)) = map.get("on_click") {
            on_event(action.clone(), None);
        }
    }
}

fn render_text_field(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let initial_text = if let Some(Value::String(val)) = map.get("value") {
        val.clone()
    } else {
        "".to_string()
    };

    let hint = if let Some(Value::String(h)) = map.get("hint") {
        h.clone()
    } else {
        "Enter text...".to_string()
    };

    // Use a unique ID to store state across frames if needed
    let id = if let Some(Value::String(id_str)) = map.get("on_change") {
        egui::Id::new(id_str)
    } else {
        ui.make_persistent_id("text_field_auto")
    };

    // SYNC GUARD: If focused, we might want to prioritize egui's internal buffer
    // to avoid "jumping" characters when Zenith state lags behind.
    let mut text = initial_text;

    let mut text_edit = TextEdit::singleline(&mut text)
        .hint_text(hint)
        .desired_width(200.0)
        .id(id);

    // Password mode
    if let Some(Value::Boolean(true)) = map.get("password") {
        text_edit = text_edit.password(true);
    }

    let response = ui.add(text_edit);

    if response.changed() {
        if let Some(Value::String(action)) = map.get("on_change") {
            on_event(action.clone(), Some(text));
        }
    }
}

fn render_checkbox(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let mut checked = if let Some(Value::Boolean(val)) = map.get("value") {
        *val
    } else {
        false
    };

    let label = if let Some(Value::String(text)) = map.get("label") {
        text.clone()
    } else {
        "Checkbox".to_string()
    };

    if ui.checkbox(&mut checked, label).changed() {
        if let Some(Value::String(action)) = map.get("on_change") {
            let value = if checked { "true" } else { "false" };
            on_event(action.clone(), Some(value.to_string()));
        }
    }
}

fn render_text_area(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let initial_text = if let Some(Value::String(val)) = map.get("value") {
        val.clone()
    } else {
        "".to_string()
    };

    let hint = if let Some(Value::String(h)) = map.get("hint") {
        h.clone()
    } else {
        "".to_string()
    };

    let id = if let Some(Value::String(id_str)) = map.get("on_change") {
        egui::Id::new(id_str)
    } else {
        ui.make_persistent_id("text_area_auto")
    };

    let mut text = initial_text;

    let text_edit = TextEdit::multiline(&mut text)
        .hint_text(hint)
        .desired_width(f32::INFINITY)
        .id(id);

    let response = ui.add(text_edit);

    if response.changed() {
        if let Some(Value::String(action)) = map.get("on_change") {
            on_event(action.clone(), Some(text));
        }
    }
}

fn render_scroll_view(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        if let Some(child) = map.get("child") {
            render_ui(ui, child, on_event);
        }
    });
}

fn render_grid(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let mut grid = egui::Grid::new(egui::Id::new("zenith_grid"));

    if let Some(Value::Integer(spacing)) = map.get("spacing") {
        grid = grid.spacing(Vec2::new(*spacing as f32, *spacing as f32));
    }

    if let Some(Value::Boolean(striped)) = map.get("striped") {
        grid = grid.striped(*striped);
    }

    grid.show(ui, |ui| {
        if let Some(children) = map.get("children") {
            if let Value::Array(items) = children {
                let items = items.lock().unwrap();
                for item in items.iter() {
                    // Each item in the grid children array should be a Row
                    if let Value::Array(row_items) = item {
                        let row_items = row_items.lock().unwrap();
                        for widget in row_items.iter() {
                            render_ui(ui, widget, on_event);
                        }
                        ui.end_row();
                    } else {
                        render_ui(ui, item, on_event);
                    }
                }
            }
        }
    });
}

fn render_slider(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let mut value = if let Some(Value::Float(v)) = map.get("value") {
        *v as f64
    } else if let Some(Value::Integer(v)) = map.get("value") {
        *v as f64
    } else {
        0.0
    };

    let min = if let Some(Value::Float(v)) = map.get("min") {
        *v as f64
    } else if let Some(Value::Integer(v)) = map.get("min") {
        *v as f64
    } else {
        0.0
    };

    let max = if let Some(Value::Float(v)) = map.get("max") {
        *v as f64
    } else if let Some(Value::Integer(v)) = map.get("max") {
        *v as f64
    } else {
        100.0
    };

    let text = if let Some(Value::String(t)) = map.get("text") {
        t.clone()
    } else {
        "".to_string()
    };

    if ui
        .add(egui::Slider::new(&mut value, min..=max).text(text))
        .changed()
    {
        if let Some(Value::String(action)) = map.get("on_change") {
            on_event(action.clone(), Some(value.to_string()));
        }
    }
}

fn render_progress_bar(ui: &mut Ui, map: &HashMap<String, Value>) {
    let progress = if let Some(Value::Float(v)) = map.get("value") {
        *v as f32
    } else if let Some(Value::Integer(v)) = map.get("value") {
        *v as f32
    } else {
        0.0
    };

    let mut pb = egui::ProgressBar::new(progress);

    if let Some(Value::String(text)) = map.get("text") {
        pb = pb.text(text);
    }

    if let Some(Value::Boolean(true)) = map.get("animate") {
        pb = pb.animate(true);
    }

    ui.add(pb);
}

fn render_combo_box(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let mut selected = if let Some(Value::String(v)) = map.get("value") {
        v.clone()
    } else {
        "".to_string()
    };

    let label = if let Some(Value::String(l)) = map.get("label") {
        l.clone()
    } else {
        "".to_string()
    };

    egui::ComboBox::from_label(label)
        .selected_text(&selected)
        .show_ui(ui, |ui| {
            if let Some(Value::Array(items)) = map.get("options") {
                let items = items.lock().unwrap();
                for item in items.iter() {
                    let item_str = item.to_string();
                    if ui
                        .selectable_value(&mut selected, item_str.clone(), item_str.clone())
                        .clicked()
                    {
                        if let Some(Value::String(action)) = map.get("on_change") {
                            on_event(action.clone(), Some(item_str));
                        }
                    }
                }
            }
        });
}

fn render_tabs(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let selected_tab = if let Some(Value::String(v)) = map.get("value") {
        v.clone()
    } else {
        "".to_string()
    };

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if let Some(Value::Array(options)) = map.get("options") {
                let options = options.lock().unwrap();
                for opt in options.iter() {
                    let opt_str = opt.to_string();
                    if ui
                        .selectable_label(selected_tab == opt_str, &opt_str)
                        .clicked()
                    {
                        if let Some(Value::String(action)) = map.get("on_change") {
                            on_event(action.clone(), Some(opt_str));
                        }
                    }
                }
            }
        });

        ui.separator();

        if let Some(Value::Object(tabs_content)) = map.get("content") {
            let content = tabs_content.lock().unwrap();
            if let Some(active_content) = content.get(&selected_tab) {
                render_ui(ui, active_content, on_event);
            }
        }
    });
}

fn render_table(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let header = if let Some(Value::Array(h)) = map.get("header") {
        h.lock().unwrap().clone()
    } else {
        vec![]
    };

    let rows = if let Some(Value::Array(r)) = map.get("rows") {
        r.lock().unwrap().clone()
    } else {
        vec![]
    };

    egui::Grid::new(egui::Id::new("zenith_table"))
        .striped(true)
        .show(ui, |ui| {
            // Header
            for col in header {
                ui.heading(col.to_string());
            }
            ui.end_row();

            // Rows
            for row in rows {
                if let Value::Array(row_data) = row {
                    let row_data = row_data.lock().unwrap();
                    for cell in row_data.iter() {
                        render_ui(ui, cell, on_event);
                    }
                    ui.end_row();
                }
            }
        });
}

fn render_window(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let title = if let Some(Value::String(t)) = map.get("title") {
        t.clone()
    } else {
        "Window".to_string()
    };

    let mut open = if let Some(Value::Boolean(o)) = map.get("open") {
        *o
    } else {
        true
    };

    if open {
        egui::Window::new(title)
            .open(&mut open)
            .show(ui.ctx(), |ui| {
                if let Some(child) = map.get("child") {
                    render_ui(ui, child, on_event);
                }
            });

        if !open {
            if let Some(Value::String(action)) = map.get("on_close") {
                on_event(action.clone(), None);
            }
        }
    }
}

fn render_collapsing_header(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let title = if let Some(Value::String(t)) = map.get("title") {
        t.clone()
    } else {
        "Header".to_string()
    };

    egui::CollapsingHeader::new(title).show(ui, |ui| {
        if let Some(child) = map.get("child") {
            render_ui(ui, child, on_event);
        }
    });
}
fn render_menu_bar(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    egui::menu::bar(ui, |ui| {
        if let Some(Value::Array(menus)) = map.get("menus") {
            let m_vec = menus.lock().unwrap();
            for menu in m_vec.iter() {
                if let Value::Object(m_obj) = menu {
                    let m_map = m_obj.lock().unwrap();
                    let title = m_map
                        .get("title")
                        .map(|v| v.to_string())
                        .unwrap_or_default();
                    ui.menu_button(title, |ui| {
                        if let Some(Value::Array(items)) = m_map.get("items") {
                            let i_vec = items.lock().unwrap();
                            for item in i_vec.iter() {
                                render_ui(ui, item, on_event);
                            }
                        }
                    });
                }
            }
        }
    });
}

fn render_status_bar(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    ui.horizontal(|ui| {
        ui.style_mut().visuals.widgets.noninteractive.bg_fill = Color32::from_gray(40);
        if let Some(children) = map.get("children") {
            render_ui(ui, children, on_event);
        }
    });
}

fn render_split_pane(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let orientation = if let Some(Value::String(o)) = map.get("orientation") {
        o.as_str()
    } else {
        "horizontal"
    };

    if orientation == "horizontal" {
        ui.columns(2, |columns| {
            if let Some(left) = map.get("left") {
                render_ui(&mut columns[0], left, on_event);
            }
            if let Some(right) = map.get("right") {
                render_ui(&mut columns[1], right, on_event);
            }
        });
    } else {
        ui.vertical(|ui| {
            if let Some(top) = map.get("top") {
                render_ui(ui, top, on_event);
            }
            ui.separator();
            if let Some(bottom) = map.get("bottom") {
                render_ui(ui, bottom, on_event);
            }
        });
    }
}

fn render_tree(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let label = map.get("label").map(|v| v.to_string()).unwrap_or_default();
    let default_open = if let Some(Value::Boolean(open)) = map.get("open") {
        *open
    } else {
        false
    };

    egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        ui.make_persistent_id(&label),
        default_open,
    )
    .show_header(ui, |ui| {
        ui.label(label);
    })
    .body(|ui| {
        if let Some(children) = map.get("children") {
            render_ui(ui, children, on_event);
        }
    });
}

fn render_code_editor(ui: &mut Ui, map: &HashMap<String, Value>, on_event: &EventHandler) {
    let mut text = if let Some(Value::String(val)) = map.get("value") {
        val.clone()
    } else {
        "".to_string()
    };

    let id = if let Some(Value::String(id_str)) = map.get("on_change") {
        egui::Id::new(id_str)
    } else {
        ui.make_persistent_id("code_editor_auto")
    };

    let text_edit = egui::TextEdit::multiline(&mut text)
        .code_editor()
        .desired_width(f32::INFINITY)
        .desired_rows(10)
        .id(id);

    let response = ui.add(text_edit);

    if response.changed() {
        if let Some(Value::String(action)) = map.get("on_change") {
            on_event(action.clone(), Some(text));
        }
    }
}
