use clap::Parser;
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use zenith_desktop::ZenithRenderer;
use zenith_runtime::{Runtime, Value};
use zenith_sema::SemanticAnalyzer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Path to the Zenith script to run")]
    script: String,

    #[arg(
        help = "Arguments to pass to the script",
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    trailing_args: Vec<String>,
}

struct ZenithApp {
    active_ui: Arc<Mutex<Option<Value>>>,
    event_sender: std::sync::mpsc::Sender<zenith_runtime::Event>,
}

impl ZenithApp {
    fn new(
        active_ui: Arc<Mutex<Option<Value>>>,
        event_sender: std::sync::mpsc::Sender<zenith_runtime::Event>,
    ) -> Self {
        Self {
            active_ui,
            event_sender,
        }
    }
}

impl eframe::App for ZenithApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let ui_root = {
            let ui = self.active_ui.lock().unwrap();
            ui.clone()
        };

        if let Some(root) = ui_root {
            let renderer = ZenithRenderer::new(root, self.event_sender.clone());
            renderer.render(ctx);
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Waiting for Zenith UI...");
                });
            });
        }

        // Keep the UI responsive
        ctx.request_repaint();
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let script_content = std::fs::read_to_string(&args.script)?;

    let active_ui = Arc::new(Mutex::new(None));
    let ui_clone_for_runtime = active_ui.clone();

    // Initialize runtime here to get the event_sender
    let mut runtime = Runtime::with_ui(ui_clone_for_runtime, args.trailing_args.clone());
    let event_sender = runtime.event_sender.clone();

    let _ = ZenithApp::new(active_ui.clone(), event_sender.clone());

    // Run the runtime in a separate thread
    thread::spawn(move || {
        // Setup lexer, parser, sema
        let lexer = zenith_lexer::Lexer::new(&script_content);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                return;
            }
        };

        let mut parser = zenith_parser::Parser::new(tokens);
        let program = match parser.parse() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Parser error: {:?}", e);
                return;
            }
        };

        // Semantic Analysis
        let mut analyzer = SemanticAnalyzer::new();
        if let Err(e) = analyzer.analyze(&program) {
            eprintln!("Semantic error: {:?}", e);
            return;
        }

        if let Err(e) = runtime.execute_program(&program) {
            eprintln!("Runtime error: {}", e);
        }
    });

    // Run UI on main thread
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Zenith Desktop",
        options,
        Box::new(|_cc| Box::new(ZenithApp::new(active_ui, event_sender))),
    )
    .map_err(|e| anyhow::anyhow!("Eframe error: {}", e))?;

    Ok(())
}
