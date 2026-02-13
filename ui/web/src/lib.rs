use eframe::egui;
use wasm_bindgen::prelude::*;
use std::sync::{Arc, Mutex};
use zenith_runtime::Value;
use zenith_ui_renderer::{render_ui, EventHandler};

// Entry point for WASM
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Setup panic hook for better error messages in browser console
    console_error_panic_hook::set_once();
    
    // Setup tracing for browser console
    tracing_wasm::set_as_global_default();
    
    Ok(())
}

struct ZenithWebApp {
    ui_tree: Value,
    counter: i64,
}

impl Default for ZenithWebApp {
    fn default() -> Self {
        Self { 
            ui_tree: create_mock_ui(0),
            counter: 0,
        }
    }
}

// Helper to create a Zenith UI tree manually (simulating runtime output)
fn create_mock_ui(count: i64) -> Value {
    let mut root = std::collections::HashMap::new();
    root.insert("type".to_string(), Value::String("Center".to_string()));
    
    let mut card = std::collections::HashMap::new();
    card.insert("type".to_string(), Value::String("Card".to_string()));
    
    let mut col = std::collections::HashMap::new();
    col.insert("type".to_string(), Value::String("Column".to_string()));
    col.insert("spacing".to_string(), Value::Integer(20));
    
    let mut children = Vec::new();
    
    // Heading
    let mut heading = std::collections::HashMap::new();
    heading.insert("type".to_string(), Value::String("Heading".to_string()));
    heading.insert("text".to_string(), Value::String("Zenith Web Demo".to_string()));
    heading.insert("level".to_string(), Value::Integer(1));
    children.push(Value::Object(Arc::new(Mutex::new(heading))));
    
    // Counter Text
    let mut text = std::collections::HashMap::new();
    text.insert("type".to_string(), Value::String("Text".to_string()));
    text.insert("text".to_string(), Value::String(format!("Current Count: {}", count)));
    text.insert("size".to_string(), Value::Integer(24));
    text.insert("bold".to_string(), Value::Boolean(true));
    text.insert("color".to_string(), Value::String("blue".to_string()));
    children.push(Value::Object(Arc::new(Mutex::new(text))));
    
    // Button Row
    let mut row = std::collections::HashMap::new();
    row.insert("type".to_string(), Value::String("Row".to_string()));
    row.insert("spacing".to_string(), Value::Integer(10));
    
    let mut row_children = Vec::new();
    
    // Inc Button
    let mut btn_inc = std::collections::HashMap::new();
    btn_inc.insert("type".to_string(), Value::String("Button".to_string()));
    btn_inc.insert("label".to_string(), Value::String("INCREMENT".to_string()));
    btn_inc.insert("style".to_string(), Value::String("success".to_string()));
    btn_inc.insert("on_click".to_string(), Value::String("inc".to_string()));
    row_children.push(Value::Object(Arc::new(Mutex::new(btn_inc))));
    
    // Dec Button
    let mut btn_dec = std::collections::HashMap::new();
    btn_dec.insert("type".to_string(), Value::String("Button".to_string()));
    btn_dec.insert("label".to_string(), Value::String("DECREMENT".to_string()));
    btn_dec.insert("style".to_string(), Value::String("danger".to_string()));
    btn_dec.insert("on_click".to_string(), Value::String("dec".to_string()));
    row_children.push(Value::Object(Arc::new(Mutex::new(btn_dec))));
    
    row.insert("children".to_string(), Value::Array(Arc::new(Mutex::new(row_children))));
    children.push(Value::Object(Arc::new(Mutex::new(row))));
    
    col.insert("children".to_string(), Value::Array(Arc::new(Mutex::new(children))));
    card.insert("child".to_string(), Value::Object(Arc::new(Mutex::new(col))));
    root.insert("child".to_string(), Value::Object(Arc::new(Mutex::new(card))));
    
    Value::Object(Arc::new(Mutex::new(root)))
}

impl eframe::App for ZenithWebApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Shared state for events captured by closure
        // We use RefCell to allow mutation from Fn closure
        let events = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let events_clone = events.clone();
        
        // Explicitly type the Box<dyn Fn...> to match EventHandler alias
        let event_handler: EventHandler = Box::new(move |action: String, payload: Option<String>| {
            events_clone.borrow_mut().push((action, payload));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Render the Zenith UI Tree using our shared renderer!
            render_ui(ui, &self.ui_tree, &event_handler);
        });
        
        // Process events
        let mut needs_update = false;
        for (action, _) in events.borrow().iter() {
            if action == "inc" {
                self.counter += 1;
                needs_update = true;
            } else if action == "dec" {
                self.counter -= 1;
                needs_update = true;
            }
        }
        
        if needs_update {
            self.ui_tree = create_mock_ui(self.counter);
        }
    }
}

// Async entry point for eframe web
#[wasm_bindgen]
pub async fn start_app(canvas_id: &str) -> Result<(), JsValue> {
    let web_options = eframe::WebOptions::default();
    
    eframe::WebRunner::new()
        .start(
            canvas_id, // "zenith-canvas"
            web_options,
            Box::new(|_cc| Box::new(ZenithWebApp::default())),
        )
        .await
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}
