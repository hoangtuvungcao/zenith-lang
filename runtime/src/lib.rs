//! Zenith Runtime System
//! Simple, working runtime implementation for Zenith programs

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::process::Command;
use std::sync::{mpsc, Arc, Mutex};
use zenith_lexer::TokenKind;
use zenith_parser::{Expression, Literal, Program, Statement};
use zenith_stdlib::core::color::Color;

// Core runtime types
pub type ZenithResult = Result<Value, RuntimeError>;

// Value types in Zenith
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Arc<Mutex<Vec<Value>>>),
    Object(Arc<Mutex<HashMap<String, Value>>>),
    Range(i64, i64),
    Color(Color),
    State(usize),
    NativeFunction(NativeFunction),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Instance {
        struct_name: String,
        fields: Arc<Mutex<HashMap<String, Value>>>,
    },
}

impl Value {
    pub fn from_json(json: serde_json::Value) -> Self {
        match json {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(b),
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    Value::Integer(n.as_i64().unwrap())
                } else {
                    Value::Float(n.as_f64().unwrap())
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(arr) => {
                let values: Vec<Value> = arr.into_iter().map(Value::from_json).collect();
                Value::Array(Arc::new(Mutex::new(values)))
            }
            serde_json::Value::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k, Value::from_json(v));
                }
                Value::Object(Arc::new(Mutex::new(map)))
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Integer(a), Value::Float(b)) => (*a as f64 - b).abs() < f64::EPSILON,
            (Value::Float(a), Value::Integer(b)) => (a - *b as f64).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => *a.lock().unwrap() == *b.lock().unwrap(),
            (Value::Object(a), Value::Object(b)) => {
                let a = a.lock().unwrap();
                let b = b.lock().unwrap();
                a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).map_or(false, |bv| v == bv))
            }
            (Value::Range(s1, e1), Value::Range(s2, e2)) => s1 == s2 && e1 == e2,
            (Value::Color(c1), Value::Color(c2)) => c1 == c2,
            (Value::NativeFunction(a), Value::NativeFunction(b)) => a == b,
            (Value::Function { name: an, .. }, Value::Function { name: bn, .. }) => an == bn,
            (Value::State(id1), Value::State(id2)) => id1 == id2,
            (
                Value::Instance {
                    struct_name: an,
                    fields: af,
                },
                Value::Instance {
                    struct_name: bn,
                    fields: bf,
                },
            ) => an == bn && *af.lock().unwrap() == *bf.lock().unwrap(),
            _ => false,
        }
    }
}

pub type NativeFunctionHandler = Arc<dyn Fn(&[Value]) -> ZenithResult + Send + Sync>;

#[derive(Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub handler: NativeFunctionHandler,
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeFunction")
            .field("name", &self.name)
            .field("arity", &self.arity)
            .finish()
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
    }
}

// Runtime error types
#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    TypeError(String),
    ReferenceError(String),
    IndexError(String),
    KeyError(String),
    DivisionByZero,
    StackOverflow,
    OutOfMemory,
    UserError(String),
    ReturnValue(Value),
}

#[derive(Debug, Clone)]
pub enum Event {
    Click { id: String },
    Input { id: String, value: String },
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::TypeError(msg) => write!(f, "TypeError: {}", msg),
            RuntimeError::ReferenceError(msg) => write!(f, "ReferenceError: {}", msg),
            RuntimeError::IndexError(msg) => write!(f, "IndexError: {}", msg),
            RuntimeError::KeyError(msg) => write!(f, "KeyError: {}", msg),
            RuntimeError::DivisionByZero => write!(f, "DivisionByZero"),
            RuntimeError::StackOverflow => write!(f, "StackOverflow"),
            RuntimeError::OutOfMemory => write!(f, "OutOfMemory"),
            RuntimeError::UserError(msg) => write!(f, "Error: {}", msg),
            RuntimeError::ReturnValue(_) => write!(f, "Return value"),
        }
    }
}

// State Management
pub struct StateStore {
    pub data: HashMap<usize, Value>,
    pub next_id: usize,
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            next_id: 0,
        }
    }
}

// Main Runtime Engine - simplified
pub struct Runtime {
    pub globals: HashMap<String, Value>,
    pub native_functions: HashMap<String, NativeFunction>,
    pub call_stack: Vec<HashMap<String, Value>>,
    pub state_store: Arc<Mutex<StateStore>>,
    pub active_ui: Arc<Mutex<Option<Value>>>,
    pub event_sender: mpsc::Sender<Event>,
    pub event_receiver: Arc<Mutex<mpsc::Receiver<Event>>>,
    pub struct_defs: HashMap<String, Vec<(String, String)>>,
    pub methods: HashMap<String, HashMap<String, Value>>,
    pub module_cache: HashMap<String, HashMap<String, Value>>,
    pub module_search_paths: Vec<String>,
    pub script_args: Vec<String>,
}

impl Runtime {
    pub fn new(script_args: Vec<String>) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut module_search_paths = vec!["lib".to_string(), ".".to_string()];

        // Check for ZENITH_LIB environment variable
        if let Ok(lib_path) = std::env::var("ZENITH_LIB") {
            module_search_paths.insert(0, lib_path);
        }

        let mut runtime = Self {
            globals: HashMap::new(),
            native_functions: HashMap::new(),
            call_stack: Vec::new(),
            state_store: Arc::new(Mutex::new(StateStore::new())),
            active_ui: Arc::new(Mutex::new(None)),
            event_sender: tx,
            event_receiver: Arc::new(Mutex::new(rx)),
            struct_defs: HashMap::new(),
            methods: HashMap::new(),
            module_cache: HashMap::new(),
            module_search_paths,
            script_args,
        };
        // Register native functions
        runtime.register_native_functions();
        runtime
    }

    pub fn with_ui(active_ui: Arc<Mutex<Option<Value>>>, script_args: Vec<String>) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut module_search_paths = vec!["lib".to_string(), ".".to_string()];

        // Check for ZENITH_LIB environment variable
        if let Ok(lib_path) = std::env::var("ZENITH_LIB") {
            module_search_paths.insert(0, lib_path);
        }

        let mut runtime = Self {
            globals: HashMap::new(),
            native_functions: HashMap::new(),
            call_stack: Vec::new(),
            state_store: Arc::new(Mutex::new(StateStore::new())),
            active_ui,
            event_sender: tx,
            event_receiver: Arc::new(Mutex::new(rx)),
            struct_defs: HashMap::new(),
            methods: HashMap::new(),
            module_cache: HashMap::new(),
            module_search_paths,
            script_args,
        };
        // Register native functions
        runtime.register_native_functions();
        runtime
    }

    pub fn set_script_args(&mut self, args: Vec<String>) {
        self.script_args = args;
    }

    pub fn add_search_path(&mut self, path: String) {
        if !self.module_search_paths.contains(&path) {
            self.module_search_paths.insert(0, path);
        }
    }

    pub fn define_native(
        &mut self,
        name: &str,
        arity: usize,
        handler: impl Fn(&[Value]) -> ZenithResult + Send + Sync + 'static,
    ) {
        self.native_functions.insert(
            name.to_string(),
            NativeFunction {
                name: name.to_string(),
                arity,
                handler: Arc::new(handler),
            },
        );
    }
    fn register_native_functions(&mut self) {
        // State Management
        {
            let store = self.state_store.clone();
            self.define_native("create_state", 1, move |args| {
                let mut state = store.lock().unwrap();
                let id = state.next_id;
                state.next_id += 1;
                state.data.insert(id, args[0].clone());
                Ok(Value::State(id))
            });
        }

        {
            let store = self.state_store.clone();
            self.define_native("get_state", 1, move |args| {
                if args.len() != 1 {
                    return Err(RuntimeError::TypeError("Expects 1 arg".to_string()));
                }
                let id = match args[0] {
                    Value::State(id) => id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "get_state expects State ID".to_string(),
                        ))
                    }
                };
                let state = store.lock().unwrap();
                match state.data.get(&id) {
                    Some(val) => Ok(val.clone()),
                    None => Err(RuntimeError::ReferenceError(format!(
                        "State ID {} not found",
                        id
                    ))),
                }
            });
        }

        {
            let store = self.state_store.clone();
            self.define_native("set_state", 2, move |args| {
                if args.len() != 2 {
                    return Err(RuntimeError::TypeError("Expects 2 args".to_string()));
                }
                let id = match args[0] {
                    Value::State(id) => id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "set_state expects State ID".to_string(),
                        ))
                    }
                };
                let new_val = args[1].clone();
                let mut state = store.lock().unwrap();
                if state.data.contains_key(&id) {
                    state.data.insert(id, new_val);
                    Ok(Value::Null)
                } else {
                    Err(RuntimeError::ReferenceError(format!(
                        "State ID {} not found",
                        id
                    )))
                }
            });
        }

        // GUI Events
        {
            let sender = self.event_sender.clone();
            self.define_native("emit_event", 1, move |args| {
                if let Value::String(id) = &args[0] {
                    let _ = sender.send(Event::Click { id: id.clone() });
                    Ok(Value::Null)
                } else {
                    Err(RuntimeError::TypeError("Expects string ID".to_string()))
                }
            });
        }

        {
            let active_ui = self.active_ui.clone();
            self.define_native("render_ui", 1, move |args| {
                if args.len() != 1 {
                    return Err(RuntimeError::TypeError(
                        "render_ui() expects 1 argument".to_string(),
                    ));
                }

                if let Ok(mut ui) = active_ui.lock() {
                    *ui = Some(args[0].clone());
                }
                Ok(Value::Null)
            });
        }

        {
            let receiver = self.event_receiver.clone();
            self.define_native("wait_for_event", 0, move |_args| {
                match receiver.lock().unwrap().recv() {
                    Ok(Event::Click { id }) => {
                        let mut map = HashMap::new();
                        map.insert("type".to_string(), Value::String("click".to_string()));
                        map.insert("id".to_string(), Value::String(id));
                        map.insert("value".to_string(), Value::Null);
                        Ok(Value::Object(Arc::new(Mutex::new(map))))
                    }
                    Ok(Event::Input { id, value }) => {
                        let mut map = HashMap::new();
                        map.insert("type".to_string(), Value::String("input".to_string()));
                        map.insert("id".to_string(), Value::String(id));
                        map.insert("value".to_string(), Value::String(value));
                        Ok(Value::Object(Arc::new(Mutex::new(map))))
                    }
                    Err(_) => Ok(Value::Null),
                }
            });
        }

        // OOP Helper
        self.define_native("new", 2, |args| {
            let name = match &args[0] {
                Value::String(s) => s.clone(),
                _ => {
                    return Err(RuntimeError::TypeError(
                        "First arg to new() must be struct name string".to_string(),
                    ))
                }
            };
            let fields_map = match &args[1] {
                Value::Object(map) => map.clone(),
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Second arg to new() must be object of fields".to_string(),
                    ))
                }
            };

            Ok(Value::Instance {
                struct_name: name,
                fields: fields_map,
            })
        });

        // StdLib Bridges
        self.define_native("print", 1, |args| {
            println!("{}", args[0]);
            Ok(Value::Null)
        });

        self.define_native("println", 1, |args| {
            println!("{}", args[0]);
            Ok(Value::Null)
        });

        // Length function
        self.define_native("len", 1, |args| {
            if args.len() != 1 {
                return Err(RuntimeError::TypeError(
                    "len() expects 1 argument".to_string(),
                ));
            }
            match &args[0] {
                Value::String(s) => Ok(Value::Integer(s.len() as i64)),
                Value::Array(arr) => Ok(Value::Integer(arr.lock().unwrap().len() as i64)),
                Value::Object(obj) => Ok(Value::Integer(obj.lock().unwrap().len() as i64)),
                _ => Err(RuntimeError::TypeError(
                    "len() expects string, array, or object".to_string(),
                )),
            }
        });

        // Array Helpers
        self.define_native("push", 2, |args| {
            if let Value::Array(arr) = &args[0] {
                arr.lock().unwrap().push(args[1].clone());
                // Return the array itself for functional style: arr = push(arr, item)
                Ok(args[0].clone())
            } else {
                Err(RuntimeError::TypeError(
                    "push expects array as first arg".to_string(),
                ))
            }
        });

        self.define_native("remove", 2, |args| {
            if let Value::Array(arr) = &args[0] {
                if let Value::Integer(idx) = &args[1] {
                    let mut a = arr.lock().unwrap();
                    if *idx >= 0 && (*idx as usize) < a.len() {
                        a.remove(*idx as usize);
                        // Return the array itself for functional style
                        return Ok(args[0].clone());
                    }
                }
                Ok(Value::Null)
            } else {
                Err(RuntimeError::TypeError(
                    "remove expects array and index".to_string(),
                ))
            }
        });

        // Type conversion
        self.define_native("str", 1, |args| Ok(Value::String(args[0].to_string())));

        self.define_native("parse_float", 1, |args| match &args[0] {
            Value::String(s) => s
                .parse::<f64>()
                .map(Value::Float)
                .map_err(|_| RuntimeError::UserError("Invalid float format".to_string())),
            Value::Float(f) => Ok(Value::Float(*f)),
            Value::Integer(i) => Ok(Value::Float(*i as f64)),
            _ => Err(RuntimeError::TypeError(
                "Expects string or number".to_string(),
            )),
        });

        self.define_native("parse_int", 1, |args| match &args[0] {
            Value::String(s) => s
                .parse::<i64>()
                .map(Value::Integer)
                .map_err(|_| RuntimeError::UserError("Invalid integer format".to_string())),
            Value::Integer(i) => Ok(Value::Integer(*i)),
            Value::Float(f) => Ok(Value::Integer(*f as i64)),
            _ => Err(RuntimeError::TypeError(
                "Expects string or number".to_string(),
            )),
        });

        self.define_native("type", 1, |args| {
            let type_name = match &args[0] {
                Value::Null => "null",
                Value::Boolean(_) => "boolean",
                Value::Integer(_) => "integer",
                Value::Float(_) => "float",
                Value::String(_) => "string",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
                Value::Function { .. } => "function",
                Value::NativeFunction(_) => "native_function",
                Value::Instance { .. } => "instance",
                Value::State(_) => "state",
                Value::Range(_, _) => "range",
                Value::Color(_) => "color",
            };
            Ok(Value::String(type_name.to_string()))
        });

        self.register_color_functions();
        self.register_io_functions();
        self.register_utils_functions();
        self.register_advanced_functions();
    }

    fn register_utils_functions(&mut self) {
        use zenith_stdlib::core::utils::*;

        // String functions
        self.define_native("str_upper", 1, |args| match &args[0] {
            Value::String(s) => Ok(Value::String(string_to_upper(s.as_str()))),
            _ => Err(RuntimeError::TypeError("Expects string".to_string())),
        });

        self.define_native("str_lower", 1, |args| match &args[0] {
            Value::String(s) => Ok(Value::String(string_to_lower(s.as_str()))),
            _ => Err(RuntimeError::TypeError("Expects string".to_string())),
        });

        self.define_native("parse_int", 1, |args| match &args[0] {
            Value::String(s) => s
                .parse::<i64>()
                .map(Value::Integer)
                .map_err(|_| RuntimeError::UserError("Invalid integer string".to_string())),
            _ => Err(RuntimeError::TypeError("Expects string".to_string())),
        });

        self.define_native("str_split", 2, |args| match (&args[0], &args[1]) {
            (Value::String(s), Value::String(sep)) => {
                let vec: Vec<Value> = string_split(s.as_str(), sep.as_str())
                    .into_iter()
                    .map(Value::String)
                    .collect();
                Ok(Value::Array(Arc::new(Mutex::new(vec))))
            }
            _ => Err(RuntimeError::TypeError("Expects two strings".to_string())),
        });

        self.define_native("str_contains", 2, |args| match (&args[0], &args[1]) {
            (Value::String(s), Value::String(sub)) => {
                Ok(Value::Boolean(string_contains(s.as_str(), sub.as_str())))
            }
            _ => Err(RuntimeError::TypeError("Expects two strings".to_string())),
        });

        self.define_native("str_replace", 3, |args| {
            match (&args[0], &args[1], &args[2]) {
                (Value::String(s), Value::String(from), Value::String(to)) => Ok(Value::String(
                    string_replace(s.as_str(), from.as_str(), to.as_str()),
                )),
                _ => Err(RuntimeError::TypeError("Expects three strings".to_string())),
            }
        });

        // Time functions
        self.define_native("time_now", 0, |_| Ok(Value::String(time_now_local())));

        self.define_native("time_utc", 0, |_| Ok(Value::String(time_now_utc())));

        self.define_native("time_format", 2, |args| match (&args[0], &args[1]) {
            (Value::String(t), Value::String(f)) => time_format(t.as_str(), f.as_str())
                .map(Value::String)
                .map_err(RuntimeError::UserError),
            _ => Err(RuntimeError::TypeError(
                "Expects time string and format string".to_string(),
            )),
        });
    }

    fn register_io_functions(&mut self) {
        // read_file
        self.define_native("read_file", 1, |args| match &args[0] {
            Value::String(path) => match zenith_stdlib::core::io::read_file(path.as_str()) {
                Ok(content) => Ok(Value::String(content)),
                Err(e) => Err(RuntimeError::UserError(e)),
            },
            _ => Err(RuntimeError::TypeError("Expects string path".to_string())),
        });

        // write_file
        self.define_native("write_file", 2, |args| match (&args[0], &args[1]) {
            (Value::String(path), Value::String(content)) => {
                match zenith_stdlib::core::io::write_file(path.as_str(), content.as_str()) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => Err(RuntimeError::UserError(e)),
                }
            }
            _ => Err(RuntimeError::TypeError(
                "Expects string path and content".to_string(),
            )),
        });

        // fs_read
        self.define_native("fs_read", 1, |args| {
            if let Value::String(path) = &args[0] {
                match fs::read_to_string(path) {
                    Ok(content) => Ok(Value::String(content)),
                    Err(_) => Ok(Value::Null),
                }
            } else {
                Err(RuntimeError::TypeError(
                    "fs_read expects string path".to_string(),
                ))
            }
        });

        // fs_write
        self.define_native("fs_write", 2, |args| {
            if args.len() != 2 {
                return Err(RuntimeError::TypeError("Expects 2 args".to_string()));
            }
            if let (Value::String(path), Value::String(content)) = (&args[0], &args[1]) {
                match fs::write(path, content) {
                    Ok(_) => Ok(Value::Boolean(true)),
                    Err(_) => Ok(Value::Boolean(false)),
                }
            } else {
                Err(RuntimeError::TypeError(
                    "Expects string path and content".to_string(),
                ))
            }
        });

        // fs_exists
        self.define_native("fs_exists", 1, |args| {
            if let Value::String(path) = &args[0] {
                Ok(Value::Boolean(std::path::Path::new(path).exists()))
            } else {
                Err(RuntimeError::TypeError(
                    "fs_exists expects string path".to_string(),
                ))
            }
        });

        // sys_exec
        self.define_native("sys_exec", 1, |args| {
            if let Value::String(cmd) = &args[0] {
                let output = Command::new("sh").arg("-c").arg(cmd).output();
                match output {
                    Ok(o) => Ok(Value::String(
                        String::from_utf8_lossy(&o.stdout).to_string(),
                    )),
                    Err(e) => Err(RuntimeError::UserError(format!("Exec failed: {}", e))),
                }
            } else {
                Err(RuntimeError::TypeError(
                    "sys_exec expects command string".to_string(),
                ))
            }
        });
    }

    fn register_advanced_functions(&mut self) {
        // ========== MATH ==========
        self.define_native("math_abs", 1, |args| match &args[0] {
            Value::Integer(n) => Ok(Value::Integer(n.abs())),
            Value::Float(n) => Ok(Value::Float(n.abs())),
            _ => Err(RuntimeError::TypeError("Expects number".to_string())),
        });

        self.define_native("math_sqrt", 1, |args| match &args[0] {
            Value::Integer(n) => Ok(Value::Float((*n as f64).sqrt())),
            Value::Float(n) => Ok(Value::Float(n.sqrt())),
            _ => Err(RuntimeError::TypeError("Expects number".to_string())),
        });

        self.define_native("math_pow", 2, |args| match (&args[0], &args[1]) {
            (Value::Integer(b), Value::Integer(e)) => Ok(Value::Integer(b.pow(*e as u32))),
            (Value::Float(b), Value::Integer(e)) => Ok(Value::Float(b.powi(*e as i32))),
            (Value::Float(b), Value::Float(e)) => Ok(Value::Float(b.powf(*e))),
            (Value::Integer(b), Value::Float(e)) => Ok(Value::Float((*b as f64).powf(*e))),
            _ => Err(RuntimeError::TypeError("Expects numbers".to_string())),
        });

        self.define_native("math_floor", 1, |args| match &args[0] {
            Value::Float(n) => Ok(Value::Integer(n.floor() as i64)),
            Value::Integer(n) => Ok(Value::Integer(*n)),
            _ => Err(RuntimeError::TypeError("Expects number".to_string())),
        });

        self.define_native("math_ceil", 1, |args| match &args[0] {
            Value::Float(n) => Ok(Value::Integer(n.ceil() as i64)),
            Value::Integer(n) => Ok(Value::Integer(*n)),
            _ => Err(RuntimeError::TypeError("Expects number".to_string())),
        });

        self.define_native("math_round", 1, |args| match &args[0] {
            Value::Float(n) => Ok(Value::Integer(n.round() as i64)),
            Value::Integer(n) => Ok(Value::Integer(*n)),
            _ => Err(RuntimeError::TypeError("Expects number".to_string())),
        });

        self.define_native("math_random", 0, |_| {
            use std::time::SystemTime;
            let seed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            let random = (seed as f64) / (u32::MAX as f64);
            Ok(Value::Float(random))
        });

        self.define_native("math_pi", 0, |_| Ok(Value::Float(std::f64::consts::PI)));

        self.define_native("math_e", 0, |_| Ok(Value::Float(std::f64::consts::E)));

        // ========== TYPE CONVERSION ==========
        self.define_native("to_float", 1, |args| match &args[0] {
            Value::Integer(n) => Ok(Value::Float(*n as f64)),
            Value::Float(n) => Ok(Value::Float(*n)),
            Value::String(s) => s
                .parse::<f64>()
                .map(Value::Float)
                .map_err(|_| RuntimeError::UserError("Invalid float".to_string())),
            _ => Err(RuntimeError::TypeError(
                "Cannot convert to float".to_string(),
            )),
        });

        self.define_native("to_int", 1, |args| match &args[0] {
            Value::Integer(n) => Ok(Value::Integer(*n)),
            Value::Float(n) => Ok(Value::Integer(*n as i64)),
            Value::String(s) => s
                .parse::<i64>()
                .map(Value::Integer)
                .map_err(|_| RuntimeError::UserError("Invalid integer".to_string())),
            Value::Boolean(b) => Ok(Value::Integer(if *b { 1 } else { 0 })),
            _ => Err(RuntimeError::TypeError("Cannot convert to int".to_string())),
        });

        self.define_native("to_string", 1, |args| {
            Ok(Value::String(format!("{}", args[0])))
        });

        // ========== TYPE CHECKING ==========
        self.define_native("type_of", 1, |args| {
            let t = match &args[0] {
                Value::Integer(_) => "int",
                Value::Float(_) => "float",
                Value::String(_) => "string",
                Value::Boolean(_) => "bool",
                Value::Null => "null",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
                Value::Function { .. } => "function",
                Value::NativeFunction(_) => "function",
                Value::State(_) => "state",
                Value::Instance { .. } => "instance",
                Value::Range(_, _) => "range",
                Value::Color(_) => "color",
            };
            Ok(Value::String(t.to_string()))
        });

        self.define_native("is_null", 1, |args| {
            Ok(Value::Boolean(args[0] == Value::Null))
        });
        self.define_native("is_array", 1, |args| {
            Ok(Value::Boolean(matches!(&args[0], Value::Array(_))))
        });
        self.define_native("is_object", 1, |args| {
            Ok(Value::Boolean(matches!(&args[0], Value::Object(_))))
        });
        self.define_native("is_string", 1, |args| {
            Ok(Value::Boolean(matches!(&args[0], Value::String(_))))
        });
        self.define_native("is_int", 1, |args| {
            Ok(Value::Boolean(matches!(&args[0], Value::Integer(_))))
        });
        self.define_native("is_float", 1, |args| {
            Ok(Value::Boolean(matches!(&args[0], Value::Float(_))))
        });

        // ========== ARRAY ==========
        self.define_native("arr_sort", 1, |args| match &args[0] {
            Value::Array(arr) => {
                let mut items = arr.lock().unwrap().clone();
                items.sort_by(|a, b| match (a, b) {
                    (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
                    (Value::String(x), Value::String(y)) => x.cmp(y),
                    _ => std::cmp::Ordering::Equal,
                });
                Ok(Value::Array(Arc::new(Mutex::new(items))))
            }
            _ => Err(RuntimeError::TypeError("Expects array".to_string())),
        });

        self.define_native("arr_reverse", 1, |args| match &args[0] {
            Value::Array(arr) => {
                let mut items = arr.lock().unwrap().clone();
                items.reverse();
                Ok(Value::Array(Arc::new(Mutex::new(items))))
            }
            _ => Err(RuntimeError::TypeError("Expects array".to_string())),
        });

        self.define_native("arr_slice", 3, |args| {
            match (&args[0], &args[1], &args[2]) {
                (Value::Array(arr), Value::Integer(start), Value::Integer(end)) => {
                    let items = arr.lock().unwrap();
                    let s = *start as usize;
                    let e = (*end as usize).min(items.len());
                    let sliced: Vec<Value> = items[s..e].to_vec();
                    Ok(Value::Array(Arc::new(Mutex::new(sliced))))
                }
                _ => Err(RuntimeError::TypeError(
                    "Expects (array, int, int)".to_string(),
                )),
            }
        });

        self.define_native("arr_concat", 2, |args| match (&args[0], &args[1]) {
            (Value::Array(a), Value::Array(b)) => {
                let mut items = a.lock().unwrap().clone();
                items.extend(b.lock().unwrap().clone());
                Ok(Value::Array(Arc::new(Mutex::new(items))))
            }
            _ => Err(RuntimeError::TypeError("Expects two arrays".to_string())),
        });

        self.define_native("arr_join", 2, |args| match (&args[0], &args[1]) {
            (Value::Array(arr), Value::String(sep)) => {
                let items = arr.lock().unwrap();
                let joined: String = items
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(sep);
                Ok(Value::String(joined))
            }
            _ => Err(RuntimeError::TypeError(
                "Expects (array, string)".to_string(),
            )),
        });

        self.define_native("arr_contains", 2, |args| match &args[0] {
            Value::Array(arr) => {
                let items = arr.lock().unwrap();
                Ok(Value::Boolean(items.contains(&args[1])))
            }
            _ => Err(RuntimeError::TypeError("Expects array".to_string())),
        });

        self.define_native("arr_flat", 1, |args| {
            fn flatten(val: &Value) -> Vec<Value> {
                match val {
                    Value::Array(arr) => {
                        let items = arr.lock().unwrap();
                        items.iter().flat_map(flatten).collect()
                    }
                    other => vec![other.clone()],
                }
            }
            Ok(Value::Array(Arc::new(Mutex::new(flatten(&args[0])))))
        });

        // ========== OBJECT ==========
        self.define_native("obj_keys", 1, |args| match &args[0] {
            Value::Object(map) => {
                let m = map.lock().unwrap();
                let keys: Vec<Value> = m.keys().map(|k| Value::String(k.clone())).collect();
                Ok(Value::Array(Arc::new(Mutex::new(keys))))
            }
            _ => Err(RuntimeError::TypeError("Expects object".to_string())),
        });

        self.define_native("obj_values", 1, |args| match &args[0] {
            Value::Object(map) => {
                let m = map.lock().unwrap();
                let vals: Vec<Value> = m.values().cloned().collect();
                Ok(Value::Array(Arc::new(Mutex::new(vals))))
            }
            _ => Err(RuntimeError::TypeError("Expects object".to_string())),
        });

        self.define_native("obj_has_key", 2, |args| match (&args[0], &args[1]) {
            (Value::Object(map), Value::String(key)) => {
                let m = map.lock().unwrap();
                Ok(Value::Boolean(m.contains_key(key)))
            }
            _ => Err(RuntimeError::TypeError(
                "Expects (object, string)".to_string(),
            )),
        });

        self.define_native("obj_merge", 2, |args| match (&args[0], &args[1]) {
            (Value::Object(a), Value::Object(b)) => {
                let mut merged = a.lock().unwrap().clone();
                merged.extend(b.lock().unwrap().clone());
                Ok(Value::Object(Arc::new(Mutex::new(merged))))
            }
            _ => Err(RuntimeError::TypeError("Expects two objects".to_string())),
        });

        // ========== JSON ==========
        self.define_native("json_stringify", 1, |args| {
            Ok(Value::String(format!("{}", args[0])))
        });

        self.define_native("json_parse", 1, |args| match &args[0] {
            Value::String(s) => match serde_json::from_str(s) {
                Ok(v) => Ok(Value::from_json(v)),
                Err(e) => Err(RuntimeError::UserError(format!("JSON parse error: {}", e))),
            },
            _ => Err(RuntimeError::TypeError("Expects string".to_string())),
        });

        // ========== OS ==========
        self.define_native("sys_cwd", 0, |_| match std::env::current_dir() {
            Ok(p) => Ok(Value::String(p.to_string_lossy().to_string())),
            Err(e) => Err(RuntimeError::UserError(format!("cwd failed: {}", e))),
        });

        self.define_native("sys_env_var", 1, |args| match &args[0] {
            Value::String(name) => match std::env::var(name) {
                Ok(val) => Ok(Value::String(val)),
                Err(_) => Ok(Value::Null),
            },
            _ => Err(RuntimeError::TypeError("Expects string".to_string())),
        });

        self.define_native("sys_platform", 0, |_| {
            Ok(Value::String(std::env::consts::OS.to_string()))
        });

        self.define_native("sys_arch", 0, |_| {
            Ok(Value::String(std::env::consts::ARCH.to_string()))
        });

        self.define_native("sys_sleep", 1, |args| match &args[0] {
            Value::Integer(ms) => {
                std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
                Ok(Value::Null)
            }
            _ => Err(RuntimeError::TypeError(
                "Expects integer milliseconds".to_string(),
            )),
        });

        self.define_native("sys_exit", 1, |args| match &args[0] {
            Value::Integer(code) => std::process::exit(*code as i32),
            _ => Err(RuntimeError::TypeError(
                "Expects integer exit code".to_string(),
            )),
        });

        self.define_native("time_millis", 0, |_| {
            use std::time::SystemTime;
            let ms = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            Ok(Value::Integer(ms))
        });

        let args_for_script = self.script_args.clone();
        self.define_native("sys_args", 0, move |_| {
            let args: Vec<Value> = args_for_script
                .iter()
                .map(|s| Value::String(s.clone()))
                .collect();
            Ok(Value::Array(Arc::new(Mutex::new(args))))
        });

        self.define_native("sys_env_vars", 0, |_| {
            let mut map = HashMap::new();
            for (key, value) in std::env::vars() {
                map.insert(key, Value::String(value));
            }
            Ok(Value::Object(Arc::new(Mutex::new(map))))
        });

        // ========== FS EXTRAS ==========
        self.define_native("fs_list", 1, |args| match &args[0] {
            Value::String(path) => match fs::read_dir(path) {
                Ok(entries) => {
                    let mut items = Vec::new();
                    for entry in entries {
                        if let Ok(e) = entry {
                            items.push(Value::String(e.file_name().to_string_lossy().to_string()));
                        }
                    }
                    Ok(Value::Array(Arc::new(Mutex::new(items))))
                }
                Err(e) => Err(RuntimeError::UserError(format!("fs_list failed: {}", e))),
            },
            _ => Err(RuntimeError::TypeError("Expects string path".to_string())),
        });

        self.define_native("fs_mkdir", 1, |args| match &args[0] {
            Value::String(path) => match fs::create_dir_all(path) {
                Ok(_) => Ok(Value::Boolean(true)),
                Err(_) => Ok(Value::Boolean(false)),
            },
            _ => Err(RuntimeError::TypeError("Expects string path".to_string())),
        });

        self.define_native("fs_remove", 1, |args| match &args[0] {
            Value::String(path) => {
                let p = std::path::Path::new(path);
                let result = if p.is_dir() {
                    fs::remove_dir_all(path)
                } else {
                    fs::remove_file(path)
                };
                match result {
                    Ok(_) => Ok(Value::Boolean(true)),
                    Err(_) => Ok(Value::Boolean(false)),
                }
            }
            _ => Err(RuntimeError::TypeError("Expects string path".to_string())),
        });

        self.define_native("fs_copy", 2, |args| match (&args[0], &args[1]) {
            (Value::String(src), Value::String(dst)) => match fs::copy(src, dst) {
                Ok(_) => Ok(Value::Boolean(true)),
                Err(_) => Ok(Value::Boolean(false)),
            },
            _ => Err(RuntimeError::TypeError(
                "Expects (src, dst) strings".to_string(),
            )),
        });

        self.define_native("fs_is_dir", 1, |args| match &args[0] {
            Value::String(path) => Ok(Value::Boolean(std::path::Path::new(path).is_dir())),
            _ => Err(RuntimeError::TypeError("Expects string path".to_string())),
        });

        self.define_native("fs_is_file", 1, |args| match &args[0] {
            Value::String(path) => Ok(Value::Boolean(std::path::Path::new(path).is_file())),
            _ => Err(RuntimeError::TypeError("Expects string path".to_string())),
        });

        // ========== STRING EXTRAS ==========
        self.define_native("str_trim", 1, |args| match &args[0] {
            Value::String(s) => Ok(Value::String(s.trim().to_string())),
            _ => Err(RuntimeError::TypeError("Expects string".to_string())),
        });

        self.define_native("str_starts_with", 2, |args| match (&args[0], &args[1]) {
            (Value::String(s), Value::String(prefix)) => {
                Ok(Value::Boolean(s.starts_with(prefix.as_str())))
            }
            _ => Err(RuntimeError::TypeError("Expects two strings".to_string())),
        });

        self.define_native("str_ends_with", 2, |args| match (&args[0], &args[1]) {
            (Value::String(s), Value::String(suffix)) => {
                Ok(Value::Boolean(s.ends_with(suffix.as_str())))
            }
            _ => Err(RuntimeError::TypeError("Expects two strings".to_string())),
        });

        self.define_native("str_len", 1, |args| match &args[0] {
            Value::String(s) => Ok(Value::Integer(s.len() as i64)),
            _ => Err(RuntimeError::TypeError("Expects string".to_string())),
        });

        self.define_native("str_repeat", 2, |args| match (&args[0], &args[1]) {
            (Value::String(s), Value::Integer(n)) => Ok(Value::String(s.repeat(*n as usize))),
            _ => Err(RuntimeError::TypeError("Expects (string, int)".to_string())),
        });

        self.define_native("str_substr", 3, |args| {
            match (&args[0], &args[1], &args[2]) {
                (Value::String(s), Value::Integer(start), Value::Integer(len)) => {
                    let sub: String = s
                        .chars()
                        .skip(*start as usize)
                        .take(*len as usize)
                        .collect();
                    Ok(Value::String(sub))
                }
                _ => Err(RuntimeError::TypeError(
                    "Expects (string, int, int)".to_string(),
                )),
            }
        });

        self.define_native("char_at", 2, |args| match (&args[0], &args[1]) {
            (Value::String(s), Value::Integer(idx)) => match s.chars().nth(*idx as usize) {
                Some(c) => Ok(Value::String(c.to_string())),
                None => Ok(Value::Null),
            },
            _ => Err(RuntimeError::TypeError("Expects (string, int)".to_string())),
        });

        // ========== COLLECTION UTILITY ==========
        self.define_native("range", 2, |args| match (&args[0], &args[1]) {
            (Value::Integer(start), Value::Integer(end)) => Ok(Value::Range(*start, *end)),
            _ => Err(RuntimeError::TypeError("Expects two integers".to_string())),
        });

        self.define_native("enumerate", 1, |args| match &args[0] {
            Value::Array(arr) => {
                let items = arr.lock().unwrap();
                let enumerated: Vec<Value> = items
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        let pair = vec![Value::Integer(i as i64), v.clone()];
                        Value::Array(Arc::new(Mutex::new(pair)))
                    })
                    .collect();
                Ok(Value::Array(Arc::new(Mutex::new(enumerated))))
            }
            _ => Err(RuntimeError::TypeError("Expects array".to_string())),
        });
    }

    fn register_color_functions(&mut self) {
        // color_from_hex
        self.define_native("color_from_hex", 1, |args| {
            if args.len() != 1 {
                return Err(RuntimeError::TypeError("Expects 1 arg".to_string()));
            }
            match &args[0] {
                Value::String(hex) => match Color::from_hex(hex) {
                    Ok(c) => Ok(Value::Color(c)),
                    Err(e) => Err(RuntimeError::UserError(e)),
                },
                _ => Err(RuntimeError::TypeError("Expects string".to_string())),
            }
        });

        // color_rgb
        self.define_native("color_rgb", 3, |args| {
            let r = match args[0] {
                Value::Integer(i) => i as u8,
                _ => return Err(RuntimeError::TypeError("r must be int".to_string())),
            };
            let g = match args[1] {
                Value::Integer(i) => i as u8,
                _ => return Err(RuntimeError::TypeError("g must be int".to_string())),
            };
            let b = match args[2] {
                Value::Integer(i) => i as u8,
                _ => return Err(RuntimeError::TypeError("b must be int".to_string())),
            };
            Ok(Value::Color(Color::rgb(r, g, b)))
        });

        // color_rgba
        self.define_native("color_rgba", 4, |args| {
            let r = match args[0] {
                Value::Integer(i) => i as u8,
                _ => return Err(RuntimeError::TypeError("r must be int".to_string())),
            };
            let g = match args[1] {
                Value::Integer(i) => i as u8,
                _ => return Err(RuntimeError::TypeError("g must be int".to_string())),
            };
            let b = match args[2] {
                Value::Integer(i) => i as u8,
                _ => return Err(RuntimeError::TypeError("b must be int".to_string())),
            };
            let a = match args[3] {
                Value::Integer(i) => i as u8,
                _ => return Err(RuntimeError::TypeError("a must be int".to_string())),
            };
            Ok(Value::Color(Color::rgba(r, g, b, a)))
        });

        // color_to_hex
        self.define_native("color_to_hex", 1, |args| match &args[0] {
            Value::Color(c) => Ok(Value::String(c.to_hex())),
            _ => Err(RuntimeError::TypeError("Expects Color".to_string())),
        });

        // color_with_opacity
        self.define_native("color_with_opacity", 2, |args| match (&args[0], &args[1]) {
            (Value::Color(c), Value::Float(o)) => Ok(Value::Color(c.with_opacity(*o as f32))),
            (Value::Color(c), Value::Integer(o)) => Ok(Value::Color(c.with_opacity(*o as f32))),
            _ => Err(RuntimeError::TypeError(
                "Expects Color and number".to_string(),
            )),
        });
    }

    pub fn execute_program(&mut self, program: &Program) -> ZenithResult {
        let mut last_value = Value::Null;
        for declaration in &program.declarations {
            match self.evaluate_declaration(declaration) {
                Ok(val) => last_value = val,
                Err(RuntimeError::ReturnValue(val)) => return Ok(val),
                Err(e) => return Err(e),
            }
        }

        // After evaluating top-level declarations, try to call 'main'
        if self.globals.contains_key("main") {
            return self.call_function("main", &[]);
        }

        Ok(last_value)
    }

    fn evaluate_declaration(&mut self, declaration: &zenith_parser::Declaration) -> ZenithResult {
        match declaration {
            zenith_parser::Declaration::Function(stmt) => self.evaluate_statement(stmt),
            zenith_parser::Declaration::Variable(stmt) => self.evaluate_statement(stmt),
            zenith_parser::Declaration::Statement(stmt) => self.evaluate_statement(stmt),
            zenith_parser::Declaration::Struct { name, fields, .. } => {
                self.struct_defs.insert(name.clone(), fields.clone());
                Ok(Value::Null)
            }
            zenith_parser::Declaration::Impl {
                target, methods, ..
            } => {
                let type_methods = self
                    .methods
                    .entry(target.clone())
                    .or_insert_with(HashMap::new);
                for method in methods {
                    if let Statement::FuncDeclaration {
                        name,
                        parameters,
                        body,
                        ..
                    } = method
                    {
                        let params = parameters.iter().map(|(p, _)| p.clone()).collect();
                        let function = Value::Function {
                            name: name.clone(),
                            params,
                            body: body.clone(),
                        };
                        type_methods.insert(name.clone(), function);
                    }
                }
                Ok(Value::Null)
            }
            zenith_parser::Declaration::Import { path, .. } => self.resolve_import(path),
            _ => Ok(Value::Null),
        }
    }

    fn resolve_import(&mut self, module_path: &str) -> ZenithResult {
        // Check cache first
        if let Some(cached) = self.module_cache.get(module_path) {
            let cached_clone = cached.clone();
            for (name, value) in cached_clone {
                self.globals.insert(name, value);
            }
            return Ok(Value::Null);
        }

        // Convert dotted path to file path: "std.math" -> "std/math.zn"
        let relative_path = module_path.replace('.', "/") + ".zn";

        // Search across module search paths
        let mut found_path: Option<String> = None;
        let search_paths = self.module_search_paths.clone();
        for base in &search_paths {
            let full_path = format!("{}/{}", base, relative_path);
            if std::path::Path::new(&full_path).exists() {
                found_path = Some(full_path);
                break;
            }
        }

        let file_path = match found_path {
            Some(p) => p,
            None => {
                return Err(RuntimeError::UserError(format!(
                    "Module '{}' not found (searched: {})",
                    module_path,
                    search_paths.join(", ")
                )));
            }
        };

        // Read and parse the module file
        let source = match fs::read_to_string(&file_path) {
            Ok(s) => s,
            Err(e) => {
                return Err(RuntimeError::UserError(format!(
                    "Failed to read module '{}': {}",
                    file_path, e
                )));
            }
        };

        let lexer = zenith_lexer::Lexer::new(&source);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                return Err(RuntimeError::UserError(format!(
                    "Lexer error in module '{}': {:?}",
                    module_path, e
                )));
            }
        };

        let mut parser = zenith_parser::Parser::new(tokens);
        let program = match parser.parse() {
            Ok(p) => p,
            Err(e) => {
                return Err(RuntimeError::UserError(format!(
                    "Parse error in module '{}': {:?}",
                    module_path, e
                )));
            }
        };

        // Save current globals, execute module, collect new symbols
        let globals_before: std::collections::HashSet<String> =
            self.globals.keys().cloned().collect();

        for decl in &program.declarations {
            self.evaluate_declaration(decl)?;
        }

        // Collect newly defined symbols (exported by the module)
        let mut exported: HashMap<String, Value> = HashMap::new();
        for (key, value) in &self.globals {
            if !globals_before.contains(key) {
                exported.insert(key.clone(), value.clone());
            }
        }

        // Cache the module
        self.module_cache.insert(module_path.to_string(), exported);

        Ok(Value::Null)
    }

    fn evaluate_statement(&mut self, statement: &Statement) -> ZenithResult {
        match statement {
            Statement::Expression(expr) => self.evaluate_expression(expr),
            Statement::VarDeclaration {
                name, initializer, ..
            } => {
                let value = if let Some(init) = initializer {
                    self.evaluate_expression(init)?
                } else {
                    Value::Null
                };
                self.set_variable(name, value);
                Ok(Value::Null)
            }
            Statement::FuncDeclaration {
                name,
                parameters,
                body,
                ..
            } => {
                let params = parameters.iter().map(|(p, _)| p.clone()).collect();
                let function = Value::Function {
                    name: name.clone(),
                    params,
                    body: body.clone(),
                };
                self.set_variable(name, function);
                Ok(Value::Null)
            }
            Statement::Block(statements) => {
                let mut last_value = Value::Null;
                for stmt in statements {
                    last_value = self.evaluate_statement(stmt)?;
                }
                Ok(last_value)
            }
            Statement::ReturnStatement { value, .. } => {
                let val = if let Some(expr) = value {
                    self.evaluate_expression(expr)?
                } else {
                    Value::Null
                };
                Err(RuntimeError::ReturnValue(val))
            }
            Statement::IfStatement {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                let cond_val = self.evaluate_expression(condition)?;
                if Self::is_truthy(&cond_val) {
                    let mut last_val = Value::Null;
                    for stmt in then_branch {
                        last_val = self.evaluate_statement(stmt)?;
                    }
                    Ok(last_val)
                } else if let Some(else_stmts) = else_branch {
                    let mut last_val = Value::Null;
                    for stmt in else_stmts {
                        last_val = self.evaluate_statement(stmt)?;
                    }
                    Ok(last_val)
                } else {
                    Ok(Value::Null)
                }
            }
            Statement::WhileStatement {
                condition, body, ..
            } => {
                let mut last_val = Value::Null;
                loop {
                    let cond_val = self.evaluate_expression(condition)?;
                    if !Self::is_truthy(&cond_val) {
                        break;
                    }
                    for stmt in body {
                        last_val = self.evaluate_statement(stmt)?;
                    }
                }
                Ok(last_val)
            }
            Statement::ForStatement {
                variable,
                iterable,
                body,
                ..
            } => {
                let iter_val = self.evaluate_expression(iterable)?;
                let mut last_val = Value::Null;

                match iter_val {
                    Value::Range(start, end) => {
                        for i in start..end {
                            if let Some(locals) = self.call_stack.last_mut() {
                                locals.insert(variable.clone(), Value::Integer(i));
                            } else {
                                self.globals.insert(variable.clone(), Value::Integer(i));
                            }

                            for stmt in body {
                                last_val = self.evaluate_statement(stmt)?;
                            }
                        }
                    }
                    Value::Array(arr) => {
                        let elements = arr.lock().unwrap().clone();
                        for val in elements {
                            if let Some(locals) = self.call_stack.last_mut() {
                                locals.insert(variable.clone(), val);
                            } else {
                                self.globals.insert(variable.clone(), val);
                            }

                            for stmt in body {
                                last_val = self.evaluate_statement(stmt)?;
                            }
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Iterable must be a range or array".to_string(),
                        ))
                    }
                }
                Ok(last_val)
            }
            _ => Ok(Value::Null), // Other statements unimplemented
        }
    }

    fn evaluate_expression(&mut self, expression: &Expression) -> ZenithResult {
        match expression {
            Expression::Literal(lit, _) => self.evaluate_literal(lit),
            Expression::Identifier(name, _) => self.get_variable(name),
            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                if let TokenKind::Assign = operator {
                    let val = self.evaluate_expression(right)?;
                    match &**left {
                        Expression::Identifier(name, _) => {
                            self.set_variable(name, val.clone());
                            return Ok(val);
                        }
                        Expression::Index { object, index, .. } => {
                            let obj = self.evaluate_expression(object)?;
                            let idx = self.evaluate_expression(index)?;
                            match (obj, idx) {
                                (Value::Array(arr), Value::Integer(i)) => {
                                    let mut a = arr.lock().unwrap();
                                    if i >= 0 && (i as usize) < a.len() {
                                        a[i as usize] = val.clone();
                                        return Ok(val);
                                    } else {
                                        return Err(RuntimeError::IndexError(format!(
                                            "Index {} out of bounds",
                                            i
                                        )));
                                    }
                                }
                                (Value::Object(map), Value::String(s)) => {
                                    map.lock().unwrap().insert(s, val.clone());
                                    return Ok(val);
                                }
                                _ => {
                                    return Err(RuntimeError::TypeError(
                                        "Assignment index requires array[int] or object[string]"
                                            .to_string(),
                                    ))
                                }
                            }
                        }
                        Expression::MemberAccess {
                            object, property, ..
                        } => {
                            let obj = self.evaluate_expression(object)?;
                            match obj {
                                Value::Object(map) => {
                                    map.lock().unwrap().insert(property.clone(), val.clone());
                                    return Ok(val);
                                }
                                Value::Instance { fields, .. } => {
                                    fields.lock().unwrap().insert(property.clone(), val.clone());
                                    return Ok(val);
                                }
                                _ => {
                                    return Err(RuntimeError::TypeError(
                                        "Cannot assign property to non-object".to_string(),
                                    ))
                                }
                            }
                        }
                        _ => {
                            return Err(RuntimeError::ReferenceError(
                                "Cannot assign to non-identifier".to_string(),
                            ))
                        }
                    }
                }

                // Short-circuiting logic
                if let TokenKind::And = operator {
                    let left_val = self.evaluate_expression(left)?;
                    if !Self::is_truthy(&left_val) {
                        return Ok(Value::Boolean(false));
                    }
                    let right_val = self.evaluate_expression(right)?;
                    return Ok(Value::Boolean(Self::is_truthy(&right_val)));
                }
                if let TokenKind::Or = operator {
                    let left_val = self.evaluate_expression(left)?;
                    if Self::is_truthy(&left_val) {
                        return Ok(Value::Boolean(true));
                    }
                    let right_val = self.evaluate_expression(right)?;
                    return Ok(Value::Boolean(Self::is_truthy(&right_val)));
                }

                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(left_val, operator, right_val)
            }
            Expression::Call {
                callee, arguments, ..
            } => {
                let mut evaluated_args = Vec::new();
                for arg in arguments {
                    evaluated_args.push(self.evaluate_expression(arg)?);
                }

                match &**callee {
                    Expression::Identifier(name, _) => self.call_function(name, &evaluated_args),
                    Expression::MemberAccess {
                        object, property, ..
                    } => {
                        let obj_val = self.evaluate_expression(object)?;
                        self.call_method(obj_val, property, &evaluated_args)
                    }
                    _ => Err(RuntimeError::TypeError(
                        "Callable must be an identifier or member access".to_string(),
                    )),
                }
            }
            Expression::MemberAccess {
                object, property, ..
            } => {
                let prop_obj = self.evaluate_expression(object)?;
                match prop_obj {
                    Value::Object(map) => {
                        map.lock().unwrap().get(property).cloned().ok_or_else(|| {
                            RuntimeError::KeyError(format!("Property '{}' not found", property))
                        })
                    }
                    Value::Instance {
                        struct_name,
                        fields,
                    } => {
                        let fields_lock = fields.lock().unwrap();
                        if let Some(val) = fields_lock.get(property) {
                            return Ok(val.clone());
                        }

                        // Try methods (return as bound function or just function?)
                        // For now just return the function.
                        if let Some(type_methods) = self.methods.get(&struct_name) {
                            if let Some(method) = type_methods.get(property) {
                                return Ok(method.clone());
                            }
                        }

                        Err(RuntimeError::KeyError(format!(
                            "Member {} not found on {}",
                            property, struct_name
                        )))
                    }
                    _ => Err(RuntimeError::TypeError(format!(
                        "Cannot access property '{}' of {}",
                        property, prop_obj
                    ))),
                }
            }
            Expression::Index { object, index, .. } => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;
                match (obj, idx) {
                    (Value::Array(arr), Value::Integer(i)) => {
                        let a = arr.lock().unwrap();
                        if i >= 0 && (i as usize) < a.len() {
                            Ok(a[i as usize].clone())
                        } else {
                            Err(RuntimeError::IndexError(format!(
                                "Index {} out of bounds",
                                i
                            )))
                        }
                    }
                    (Value::Object(map), Value::String(s)) => map
                        .lock()
                        .unwrap()
                        .get(&s)
                        .cloned()
                        .ok_or_else(|| RuntimeError::KeyError(format!("Key '{}' not found", s))),
                    _ => Err(RuntimeError::TypeError(
                        "Indexing requires array[int] or object[string]".to_string(),
                    )),
                }
            }
            Expression::Range { start, end, .. } => {
                let start_val = self.evaluate_expression(start)?;
                let end_val = self.evaluate_expression(end)?;

                match (start_val, end_val) {
                    (Value::Integer(s), Value::Integer(e)) => Ok(Value::Range(s, e)),
                    _ => Err(RuntimeError::TypeError(
                        "Range bounds must be integers".to_string(),
                    )),
                }
            }
            Expression::Array(elements, _) => {
                let mut vals = Vec::new();
                for expr in elements {
                    vals.push(self.evaluate_expression(expr)?);
                }
                Ok(Value::Array(Arc::new(Mutex::new(vals))))
            }
            Expression::Object(properties, _) => {
                let mut obj = HashMap::new();
                for (key, expr) in properties {
                    obj.insert(key.clone(), self.evaluate_expression(expr)?);
                }
                Ok(Value::Object(Arc::new(Mutex::new(obj))))
            }
            Expression::Unary {
                operator, operand, ..
            } => {
                let val = self.evaluate_expression(operand)?;
                match operator {
                    TokenKind::Not => Ok(Value::Boolean(!Self::is_truthy(&val))),
                    TokenKind::Minus => match val {
                        Value::Integer(i) => Ok(Value::Integer(-i)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Err(RuntimeError::TypeError(
                            "Unary minus requires number".to_string(),
                        )),
                    },
                    _ => Err(RuntimeError::TypeError(
                        "Unsupported unary operator".to_string(),
                    )),
                }
            }
            _ => Ok(Value::Null),
        }
    }

    fn call_method(&mut self, receiver: Value, method: &str, args: &[Value]) -> ZenithResult {
        match &receiver {
            Value::Object(obj) => {
                // Clone the method value first to avoid borrowing issues
                let method_val = obj.lock().unwrap().get(method).cloned();

                if let Some(Value::NativeFunction(func)) = method_val {
                    let mut all_args = vec![receiver.clone()];
                    all_args.extend_from_slice(args);
                    (func.handler)(&all_args)
                } else if let Some(Value::Function { params, body, .. }) = method_val {
                    self.call_function_with_this(receiver, &params, &body, args)
                } else {
                    Err(RuntimeError::KeyError(format!(
                        "Method '{}' not found",
                        method
                    )))
                }
            }
            Value::Instance {
                struct_name,
                fields,
            } => {
                // Check if it's a field that is a function
                let field_val = fields.lock().unwrap().get(method).cloned();
                if let Some(val) = field_val {
                    match val {
                        Value::NativeFunction(func) => {
                            let mut all_args = vec![receiver.clone()];
                            all_args.extend_from_slice(args);
                            return (func.handler)(&all_args);
                        }
                        Value::Function { params, body, .. } => {
                            return self.call_function_with_this(receiver, &params, &body, args);
                        }
                        _ => {}
                    }
                }

                // Check type methods
                let method_to_call = if let Some(type_methods) = self.methods.get(struct_name) {
                    type_methods.get(method).cloned()
                } else {
                    None
                };

                if let Some(val) = method_to_call {
                    match val {
                        Value::Function { params, body, .. } => {
                            return self.call_function_with_this(receiver, &params, &body, args);
                        }
                        _ => {}
                    }
                }

                Err(RuntimeError::KeyError(format!(
                    "Method '{}' not found on {}",
                    method, struct_name
                )))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot call method '{}' on {}",
                method, receiver
            ))),
        }
    }

    fn call_function_with_this(
        &mut self,
        this: Value,
        params: &[String],
        body: &[Statement],
        args: &[Value],
    ) -> ZenithResult {
        let mut scope = HashMap::new();
        scope.insert("this".to_string(), this.clone());
        scope.insert("self".to_string(), this);
        for (i, param) in params.iter().enumerate() {
            if i < args.len() {
                scope.insert(param.clone(), args[i].clone());
            }
        }
        self.call_stack.push(scope);

        let mut last_val = Value::Null;
        for stmt in body {
            match self.evaluate_statement(stmt) {
                Ok(val) => last_val = val,
                Err(RuntimeError::ReturnValue(val)) => {
                    last_val = val;
                    break;
                }
                Err(e) => {
                    self.call_stack.pop();
                    return Err(e);
                }
            }
        }

        self.call_stack.pop();
        Ok(last_val)
    }
    fn evaluate_literal(&self, literal: &Literal) -> ZenithResult {
        match literal {
            Literal::Number(s) => {
                if s.contains('.') {
                    Ok(Value::Float(s.parse::<f64>().map_err(|_| {
                        RuntimeError::TypeError("Invalid float literal".to_string())
                    })?))
                } else {
                    Ok(Value::Integer(s.parse::<i64>().map_err(|_| {
                        RuntimeError::TypeError("Invalid integer literal".to_string())
                    })?))
                }
            }
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Boolean(b) => Ok(Value::Boolean(*b)),
            Literal::Null => Ok(Value::Null),
        }
    }

    fn evaluate_binary_op(&self, left: Value, op: &TokenKind, right: Value) -> ZenithResult {
        // Clone op to match on value instead of reference
        let op = op.clone();
        match (left, op, right) {
            // Integer Arithmetic
            (Value::Integer(a), TokenKind::Plus, Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Integer(a), TokenKind::Minus, Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Integer(a), TokenKind::Multiply, Value::Integer(b)) => {
                Ok(Value::Integer(a * b))
            }
            (Value::Integer(a), TokenKind::Divide, Value::Integer(b)) => {
                if b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Integer(a / b))
                }
            }
            (Value::Integer(a), TokenKind::Modulo, Value::Integer(b)) => {
                if b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Integer(a % b))
                }
            }

            // Float & Mixed Arithmetic
            (Value::Float(a), TokenKind::Plus, Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Integer(a), TokenKind::Plus, Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
            (Value::Float(a), TokenKind::Plus, Value::Integer(b)) => Ok(Value::Float(a + b as f64)),

            (Value::Float(a), TokenKind::Minus, Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Integer(a), TokenKind::Minus, Value::Float(b)) => {
                Ok(Value::Float(a as f64 - b))
            }
            (Value::Float(a), TokenKind::Minus, Value::Integer(b)) => {
                Ok(Value::Float(a - b as f64))
            }

            (Value::Float(a), TokenKind::Multiply, Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Integer(a), TokenKind::Multiply, Value::Float(b)) => {
                Ok(Value::Float(a as f64 * b))
            }
            (Value::Float(a), TokenKind::Multiply, Value::Integer(b)) => {
                Ok(Value::Float(a * b as f64))
            }

            (Value::Float(a), TokenKind::Divide, Value::Float(b)) => Ok(Value::Float(a / b)),
            (Value::Integer(a), TokenKind::Divide, Value::Float(b)) => {
                Ok(Value::Float(a as f64 / b))
            }
            (Value::Float(a), TokenKind::Divide, Value::Integer(b)) => {
                Ok(Value::Float(a / b as f64))
            }

            (Value::Float(a), TokenKind::Modulo, Value::Float(b)) => Ok(Value::Float(a % b)),
            (Value::Integer(a), TokenKind::Modulo, Value::Float(b)) => {
                Ok(Value::Float(a as f64 % b))
            }
            (Value::Float(a), TokenKind::Modulo, Value::Integer(b)) => {
                Ok(Value::Float(a % b as f64))
            }

            // Integer Comparisons
            (Value::Integer(a), TokenKind::LessThan, Value::Integer(b)) => {
                Ok(Value::Boolean(a < b))
            }
            (Value::Integer(a), TokenKind::GreaterThan, Value::Integer(b)) => {
                Ok(Value::Boolean(a > b))
            }
            (Value::Integer(a), TokenKind::LessEqual, Value::Integer(b)) => {
                Ok(Value::Boolean(a <= b))
            }
            (Value::Integer(a), TokenKind::GreaterEqual, Value::Integer(b)) => {
                Ok(Value::Boolean(a >= b))
            }

            // Float & Mixed Comparisons
            (Value::Float(a), TokenKind::LessThan, Value::Float(b)) => Ok(Value::Boolean(a < b)),
            (Value::Integer(a), TokenKind::LessThan, Value::Float(b)) => {
                Ok(Value::Boolean((a as f64) < b))
            }
            (Value::Float(a), TokenKind::LessThan, Value::Integer(b)) => {
                Ok(Value::Boolean(a < (b as f64)))
            }

            (Value::Float(a), TokenKind::GreaterThan, Value::Float(b)) => Ok(Value::Boolean(a > b)),
            (Value::Integer(a), TokenKind::GreaterThan, Value::Float(b)) => {
                Ok(Value::Boolean((a as f64) > b))
            }
            (Value::Float(a), TokenKind::GreaterThan, Value::Integer(b)) => {
                Ok(Value::Boolean(a > (b as f64)))
            }

            (Value::Float(a), TokenKind::LessEqual, Value::Float(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Integer(a), TokenKind::LessEqual, Value::Float(b)) => {
                Ok(Value::Boolean((a as f64) <= b))
            }
            (Value::Float(a), TokenKind::LessEqual, Value::Integer(b)) => {
                Ok(Value::Boolean(a <= (b as f64)))
            }

            (Value::Float(a), TokenKind::GreaterEqual, Value::Float(b)) => {
                Ok(Value::Boolean(a >= b))
            }
            (Value::Integer(a), TokenKind::GreaterEqual, Value::Float(b)) => {
                Ok(Value::Boolean((a as f64) >= b))
            }
            (Value::Float(a), TokenKind::GreaterEqual, Value::Integer(b)) => {
                Ok(Value::Boolean(a >= (b as f64)))
            }

            // Equality (Uses the updated PartialEq)
            (left, TokenKind::Equal, right) => Ok(Value::Boolean(left == right)),
            (left, TokenKind::NotEqual, right) => Ok(Value::Boolean(left != right)),

            // String concatenation
            (Value::String(a), TokenKind::Plus, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::String(a), TokenKind::Plus, Value::Null) => Ok(Value::String(a)),
            (Value::Null, TokenKind::Plus, Value::String(b)) => Ok(Value::String(b)),
            (Value::String(a), TokenKind::Plus, other) => {
                Ok(Value::String(format!("{}{}", a, other)))
            }
            (other, TokenKind::Plus, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", other, b)))
            }

            (_l, _op, _r) => Err(RuntimeError::TypeError(
                "Unsupported binary operation".to_string(),
            )),
        }
    }

    fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }

    pub fn execute(&mut self, code: &str) -> ZenithResult {
        // This would parse and execute Zenith code
        // For now, just return a placeholder
        println!("Executing Zenith code: {}", code);
        Ok(Value::Null)
    }

    pub fn call_function(&mut self, callee: &str, arguments: &[Value]) -> ZenithResult {
        // Try native functions first
        if let Some(native_fn) = self.native_functions.get(callee) {
            if native_fn.arity != arguments.len() {
                return Err(RuntimeError::TypeError(format!(
                    "Native function {} expects {} arguments, got {}",
                    native_fn.name,
                    native_fn.arity,
                    arguments.len()
                )));
            }
            return (native_fn.handler)(arguments);
        }

        // Try user-defined functions (stored in variables)
        let func_val = self.get_variable(callee)?;
        match func_val {
            Value::Function { params, body, .. } => {
                if params.len() != arguments.len() {
                    return Err(RuntimeError::TypeError(format!(
                        "Function {} expects {} arguments, got {}",
                        callee,
                        params.len(),
                        arguments.len()
                    )));
                }

                // Create new scope for function call
                let mut locals = HashMap::new();
                for (name, val) in params.iter().zip(arguments.iter()) {
                    locals.insert(name.clone(), val.clone());
                }

                self.call_stack.push(locals);

                let mut last_val = Value::Null;
                for stmt in body {
                    match self.evaluate_statement(&stmt) {
                        Ok(val) => last_val = val,
                        Err(RuntimeError::ReturnValue(val)) => {
                            last_val = val;
                            break;
                        }
                        Err(e) => {
                            self.call_stack.pop();
                            return Err(e);
                        }
                    }
                }

                self.call_stack.pop();
                Ok(last_val)
            }
            _ => Err(RuntimeError::TypeError(format!(
                "{} is not a function",
                callee
            ))),
        }
    }

    pub fn get_variable(&self, name: &str) -> ZenithResult {
        // Handle literal identifiers (null, true, false)
        match name {
            "null" => return Ok(Value::Null),
            "true" => return Ok(Value::Boolean(true)),
            "false" => return Ok(Value::Boolean(false)),
            _ => {}
        }

        // Check locals (top Down)
        if let Some(locals) = self.call_stack.last() {
            if let Some(val) = locals.get(name) {
                return Ok(val.clone());
            }
        }

        // Check globals
        self.globals
            .get(name)
            .cloned()
            .ok_or_else(|| RuntimeError::ReferenceError(format!("Undefined variable: {}", name)))
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        if let Some(locals) = self.call_stack.last_mut() {
            locals.insert(name.to_string(), value);
        } else {
            self.globals.insert(name.to_string(), value);
        }
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Range(start, end) => format!("{}..{}", start, end),
            Value::Color(c) => c.to_hex(),
            Value::State(id) => format!("<State #{}>", id),
            Value::Array(arr) => {
                let a = arr.lock().unwrap();
                let elements: Vec<String> = a.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Object(obj) => {
                let o = obj.lock().unwrap();
                let properties: Vec<String> = o
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", properties.join(", "))
            }
            Value::NativeFunction(native) => format!("<native function {}>", native.name),
            Value::Function { name, .. } => format!("<function {}>", name),
            Value::Instance {
                struct_name,
                fields,
            } => {
                let fields = fields.lock().unwrap();
                let properties: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{} {{{}}}", struct_name, properties.join(", "))
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(a) => {
                let arr = a.lock().unwrap();
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Object(o) => {
                let obj = o.lock().unwrap();
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Range(s, e) => write!(f, "{}..{}", s, e),
            Value::Color(c) => write!(f, "Color({:?})", c),
            Value::State(id) => write!(f, "State({})", id),
            Value::NativeFunction(nf) => write!(f, "<native fn {}>", nf.name),
            Value::Function { name, .. } => write!(f, "<fn {}>", name),
            Value::Instance { struct_name, .. } => write!(f, "<instance of {}>", struct_name),
        }
    }
}

// Custom Serialize for Value to handle Mutexes
impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        match self {
            Value::Null => {
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "Null")?;
                s.serialize_field("value", &())?;
                s.end()
            }
            Value::Boolean(b) => {
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "Boolean")?;
                s.serialize_field("value", b)?;
                s.end()
            }
            Value::Integer(i) => {
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "Integer")?;
                s.serialize_field("value", i)?;
                s.end()
            }
            Value::Float(f) => {
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "Float")?;
                s.serialize_field("value", f)?;
                s.end()
            }
            Value::String(st) => {
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "String")?;
                s.serialize_field("value", st)?;
                s.end()
            }
            Value::Array(a) => {
                let arr = a.lock().unwrap();
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "Array")?;
                s.serialize_field("value", &*arr)?;
                s.end()
            }
            Value::Object(o) => {
                let obj = o.lock().unwrap();
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "Object")?;
                s.serialize_field("value", &*obj)?;
                s.end()
            }
            Value::Range(st, e) => {
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "Range")?;
                s.serialize_field("value", &(st, e))?;
                s.end()
            }
            Value::Color(c) => {
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "Color")?;
                s.serialize_field("value", c)?;
                s.end()
            }
            Value::State(id) => {
                let mut s = serializer.serialize_struct("Value", 2)?;
                s.serialize_field("type", "State")?;
                s.serialize_field("value", id)?;
                s.end()
            }
            _ => serializer.serialize_none(),
        }
    }
}

// Public API
pub fn create_runtime() -> Runtime {
    Runtime::new(vec![])
}

pub fn execute_code(runtime: &mut Runtime, code: &str) -> ZenithResult {
    runtime.execute(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = Runtime::new(vec![]);
        assert_eq!(runtime.globals.len(), 0);
        assert!(runtime.native_functions.contains_key("print"));
    }

    #[test]
    fn test_native_functions() {
        let mut runtime = Runtime::new(vec![]);

        // Test print function
        let result = runtime.call_function("print", &[Value::String("Hello, World!".to_string())]);
        assert!(result.is_ok());

        // Test len function
        let result = runtime.call_function(
            "len",
            &[Value::Array(Arc::new(Mutex::new(vec![
                Value::Integer(1),
                Value::Integer(2),
            ])))],
        );
        assert_eq!(result, Ok(Value::Integer(2)));

        // Test type function
        let result = runtime.call_function("type", &[Value::Integer(42)]);
        assert_eq!(result, Ok(Value::String("integer".to_string())));
    }

    #[test]
    fn test_array_operations() {
        let mut runtime = Runtime::new(vec![]);

        // Test push operation
        let _arr = Value::Array(Arc::new(Mutex::new(vec![
            Value::Integer(1),
            Value::Integer(2),
        ])));
        let result = runtime.call_function(
            "push",
            &[
                Value::Array(Arc::new(Mutex::new(vec![Value::Integer(1)]))),
                Value::Integer(2),
            ],
        );
        assert!(result.is_ok());

        if let Ok(Value::Array(new_arr)) = result {
            assert_eq!(new_arr.lock().unwrap().len(), 2); // Corrected expected length
        }
    }

    #[test]
    fn test_object_operations() {
        let mut runtime = Runtime::new(vec![]);

        // Test get operation
        let mut obj_map = HashMap::new();
        obj_map.insert("name".to_string(), Value::String("Alice".to_string()));
        let obj_value = Value::Object(Arc::new(Mutex::new(obj_map)));

        let result = runtime.call_function("len", &[obj_value.clone()]); // Use clone for len call
        assert_eq!(result, Ok(Value::Integer(1))); // Assuming len works on objects

        // Test getting existing key
        let result = runtime.call_function(
            "get",
            &[obj_value.clone(), Value::String("name".to_string())],
        );
        assert_eq!(result, Ok(Value::String("Zenith".to_string())));

        // Test getting non-existing key
        let result = runtime.call_function(
            "get",
            &[obj_value, Value::String("nonexistent".to_string())],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_variable_operations() {
        let mut runtime = Runtime::new(vec![]);

        // Test setting and getting variables
        runtime.set_variable("x", Value::Integer(42));
        let result = runtime.get_variable("x");
        assert_eq!(result, Ok(Value::Integer(42)));

        // Test undefined variable
        let result = runtime.get_variable("undefined");
        assert!(result.is_err());
    }
}
