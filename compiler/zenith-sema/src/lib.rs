use std::collections::{HashMap, HashSet};
use thiserror::Error;
use zenith_lexer::{Span, TokenKind};
use zenith_parser::{Declaration, Expression, Literal, Program, Statement};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Void,
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Struct(String),
    Enum(String),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub type_: Type,
    pub is_mutable: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<usize>,
}

#[derive(Error, Debug, Clone)]
pub enum SemanticError {
    #[error("Undefined variable '{name}' at {span}")]
    UndefinedVariable { name: String, span: Span },

    #[error("Variable '{name}' already defined at {span}")]
    RedeclaredVariable { name: String, span: Span },

    #[error("Type mismatch: expected {expected:?}, found {found:?} at {span}")]
    TypeMismatch {
        expected: Type,
        found: Type,
        span: Span,
    },

    #[error("Invalid number of arguments: expected {expected}, found {found} at {span}")]
    ArgumentCountMismatch {
        expected: usize,
        found: usize,
        span: Span,
    },

    #[error("Cannot call non-function type {type_:?} at {span}")]
    NotCallable { type_: Type, span: Span },

    #[error("Break statement outside of loop at {span}")]
    BreakOutsideLoop { span: Span },

    #[error("Continue statement outside of loop at {span}")]
    ContinueOutsideLoop { span: Span },

    #[error("Return statement outside of function at {span}")]
    ReturnOutsideFunction { span: Span },

    #[error("Module not found: {name} at {span}")]
    ModuleNotFound { name: String, span: Span },
}

pub struct SemanticAnalyzer {
    pub scopes: Vec<Scope>,
    pub current_scope: usize,
    pub errors: Vec<SemanticError>,
    pub module_search_paths: Vec<String>,
    pub imported_modules: HashSet<String>,
    pub in_loop: bool,
    pub in_function: bool,
    pub pass: usize,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let global_scope = Scope {
            symbols: HashMap::new(),
            parent: None,
        };

        let mut analyzer = Self {
            scopes: vec![global_scope],
            current_scope: 0,
            errors: Vec::new(),
            module_search_paths: vec![".".to_string(), "lib".to_string()],
            imported_modules: HashSet::new(),
            in_loop: false,
            in_function: false,
            pass: 0,
        };

        analyzer.define_builtins();
        analyzer
    }

    fn define_builtins(&mut self) {
        let builtins = vec![
            (
                "print",
                Type::Function {
                    params: vec![Type::Unknown],
                    return_type: Box::new(Type::Void),
                },
            ),
            (
                "len",
                Type::Function {
                    params: vec![Type::Unknown],
                    return_type: Box::new(Type::Int),
                },
            ),
            (
                "str",
                Type::Function {
                    params: vec![Type::Unknown],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "str_split",
                Type::Function {
                    params: vec![Type::String, Type::String],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "split",
                Type::Function {
                    params: vec![Type::String, Type::String],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "str_contains",
                Type::Function {
                    params: vec![Type::String, Type::String],
                    return_type: Box::new(Type::Bool),
                },
            ),
            (
                "str_replace",
                Type::Function {
                    params: vec![Type::String, Type::String, Type::String],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "str_substr",
                Type::Function {
                    params: vec![Type::String, Type::Int, Type::Int],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "substring",
                Type::Function {
                    params: vec![Type::String, Type::Int, Type::Int],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "char_code",
                Type::Function {
                    params: vec![Type::String, Type::Int],
                    return_type: Box::new(Type::Int),
                },
            ),
            (
                "char",
                Type::Function {
                    params: vec![Type::Int],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "hex_digit",
                Type::Function {
                    params: vec![Type::Int],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "hex_char_to_int",
                Type::Function {
                    params: vec![Type::String],
                    return_type: Box::new(Type::Int),
                },
            ),
            (
                "abs",
                Type::Function {
                    params: vec![Type::Int],
                    return_type: Box::new(Type::Int),
                },
            ),
            (
                "random",
                Type::Function {
                    params: vec![Type::Int, Type::Int],
                    return_type: Box::new(Type::Int),
                },
            ),
            (
                "random_int",
                Type::Function {
                    params: vec![Type::Int, Type::Int],
                    return_type: Box::new(Type::Int),
                },
            ),
            (
                "timestamp",
                Type::Function {
                    params: vec![],
                    return_type: Box::new(Type::Int),
                },
            ),
            (
                "to_int",
                Type::Function {
                    params: vec![Type::Unknown],
                    return_type: Box::new(Type::Int),
                },
            ),
            (
                "parse_int",
                Type::Function {
                    params: vec![Type::String],
                    return_type: Box::new(Type::Int),
                },
            ),
            (
                "parse_float",
                Type::Function {
                    params: vec![Type::String],
                    return_type: Box::new(Type::Float),
                },
            ),
            (
                "push",
                Type::Function {
                    params: vec![Type::Unknown, Type::Unknown],
                    return_type: Box::new(Type::Void),
                },
            ),
            (
                "pop",
                Type::Function {
                    params: vec![Type::Unknown],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "remove",
                Type::Function {
                    params: vec![Type::Unknown, Type::Int],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "contains",
                Type::Function {
                    params: vec![Type::Unknown, Type::Unknown],
                    return_type: Box::new(Type::Bool),
                },
            ),
            (
                "cos",
                Type::Function {
                    params: vec![Type::Float],
                    return_type: Box::new(Type::Float),
                },
            ),
            (
                "sin",
                Type::Function {
                    params: vec![Type::Float],
                    return_type: Box::new(Type::Float),
                },
            ),
            (
                "sqrt",
                Type::Function {
                    params: vec![Type::Float],
                    return_type: Box::new(Type::Float),
                },
            ),
            (
                "sys_cwd",
                Type::Function {
                    params: vec![],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "sys_platform",
                Type::Function {
                    params: vec![],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "sys_arch",
                Type::Function {
                    params: vec![],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "create_state",
                Type::Function {
                    params: vec![Type::String, Type::Unknown],
                    return_type: Box::new(Type::Void),
                },
            ),
            (
                "get_state",
                Type::Function {
                    params: vec![Type::String],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "set_state",
                Type::Function {
                    params: vec![Type::String, Type::Unknown],
                    return_type: Box::new(Type::Void),
                },
            ),
            (
                "html_tag",
                Type::Function {
                    params: vec![Type::String, Type::String, Type::String],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "html_div",
                Type::Function {
                    params: vec![Type::String, Type::String, Type::String],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "html_span",
                Type::Function {
                    params: vec![Type::String, Type::String, Type::String],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "html_input",
                Type::Function {
                    params: vec![Type::String, Type::String, Type::String, Type::String],
                    return_type: Box::new(Type::String),
                },
            ),
            (
                "gui_button",
                Type::Function {
                    params: vec![Type::String, Type::Unknown],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "gui_text",
                Type::Function {
                    params: vec![Type::String],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "gui_row",
                Type::Function {
                    params: vec![Type::Unknown],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "gui_column",
                Type::Function {
                    params: vec![Type::Unknown],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "gui_slider",
                Type::Function {
                    params: vec![Type::Float, Type::Float, Type::Float, Type::Unknown],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "gui_checkbox",
                Type::Function {
                    params: vec![Type::String, Type::Bool, Type::Unknown],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "gui_spacer",
                Type::Function {
                    params: vec![Type::Float],
                    return_type: Box::new(Type::Unknown),
                },
            ),
            (
                "render_ui",
                Type::Function {
                    params: vec![Type::Unknown],
                    return_type: Box::new(Type::Void),
                },
            ),
            (
                "wait_for_event",
                Type::Function {
                    params: vec![],
                    return_type: Box::new(Type::Unknown),
                },
            ),
        ];

        for (name, type_) in builtins {
            self.define(Symbol {
                name: name.to_string(),
                type_,
                is_mutable: false,
                span: Default::default(),
            })
            .ok();
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        self.pass = 1;
        for decl in &program.declarations {
            self.analyze_declaration_pass1(decl);
        }

        self.pass = 2;
        for decl in &program.declarations {
            self.analyze_declaration_pass2(decl);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn analyze_declaration_pass1(&mut self, decl: &Declaration) {
        match decl {
            Declaration::Function(stmt) => {
                if let Statement::FuncDeclaration {
                    name,
                    parameters,
                    span,
                    ..
                } = stmt
                {
                    let _ = self.define(Symbol {
                        name: name.clone(),
                        type_: Type::Function {
                            params: vec![Type::Unknown; parameters.len()],
                            return_type: Box::new(Type::Unknown),
                        },
                        is_mutable: false,
                        span: *span,
                    });
                }
            }
            Declaration::Variable(stmt) => {
                if let Statement::VarDeclaration { name, span, .. } = stmt {
                    let _ = self.define(Symbol {
                        name: name.clone(),
                        type_: Type::Unknown,
                        is_mutable: true,
                        span: *span,
                    });
                }
            }
            Declaration::Import { path, .. } => {
                self.resolve_import_symbols(path);
            }
            _ => {}
        }
    }

    fn analyze_declaration_pass2(&mut self, decl: &Declaration) {
        match decl {
            Declaration::Function(stmt) => self.analyze_statement(stmt),
            Declaration::Variable(stmt) => self.analyze_statement(stmt),
            Declaration::Statement(stmt) => self.analyze_statement(stmt),
            _ => {}
        }
    }

    fn resolve_import_symbols(&mut self, module_path: &str) {
        if self.imported_modules.contains(module_path) {
            return;
        }
        self.imported_modules.insert(module_path.to_string());

        let relative_path = module_path.replace('.', "/") + ".zn";
        let mut found_path = None;
        for base in &self.module_search_paths.clone() {
            let full_path = format!("{}/{}", base, relative_path);
            if std::path::Path::new(&full_path).exists() {
                found_path = Some(full_path);
                break;
            }
        }

        if let Some(file_path) = found_path {
            if let Ok(source) = std::fs::read_to_string(&file_path) {
                let lexer = zenith_lexer::Lexer::new(&source);
                if let Ok(tokens) = lexer.tokenize() {
                    let mut parser = zenith_parser::Parser::new(tokens);
                    if let Ok(program) = parser.parse() {
                        for decl in &program.declarations {
                            self.analyze_declaration_pass1(decl);
                        }
                    }
                }
            }
        }
    }

    fn analyze_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VarDeclaration {
                name,
                initializer,
                span,
                ..
            } => {
                if let Some(init) = initializer {
                    self.analyze_expression(init);
                }

                let _ = self.define(Symbol {
                    name: name.clone(),
                    type_: Type::Unknown,
                    is_mutable: true,
                    span: *span,
                });
            }
            Statement::FuncDeclaration {
                name: _,
                parameters,
                body,
                span,
                ..
            } => {
                if self.pass == 2 {
                    self.enter_scope();
                    let old_in_func = self.in_function;
                    self.in_function = true;

                    for (param_name, _) in parameters {
                        let _ = self.define(Symbol {
                            name: param_name.clone(),
                            type_: Type::Unknown,
                            is_mutable: true,
                            span: *span,
                        });
                    }

                    for inner_stmt in body {
                        self.analyze_statement(inner_stmt);
                    }

                    self.in_function = old_in_func;
                    self.exit_scope();
                }
            }
            Statement::ReturnStatement { value, span } => {
                if !self.in_function {
                    self.errors
                        .push(SemanticError::ReturnOutsideFunction { span: *span });
                }
                if let Some(expr) = value {
                    self.analyze_expression(expr);
                }
            }
            Statement::IfStatement {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.analyze_expression(condition);
                self.enter_scope();
                for inner_stmt in then_branch {
                    self.analyze_statement(inner_stmt);
                }
                self.exit_scope();

                if let Some(branch) = else_branch {
                    self.enter_scope();
                    for inner_stmt in branch {
                        self.analyze_statement(inner_stmt);
                    }
                    self.exit_scope();
                }
            }
            Statement::WhileStatement {
                condition, body, ..
            } => {
                self.analyze_expression(condition);
                let old_loop = self.in_loop;
                self.in_loop = true;
                self.enter_scope();
                for inner_stmt in body {
                    self.analyze_statement(inner_stmt);
                }
                self.exit_scope();
                self.in_loop = old_loop;
            }
            Statement::ForStatement {
                variable,
                iterable,
                body,
                ..
            } => {
                self.analyze_expression(iterable);
                let old_loop = self.in_loop;
                self.in_loop = true;
                self.enter_scope();

                let _ = self.define(Symbol {
                    name: variable.clone(),
                    type_: Type::Unknown,
                    is_mutable: true,
                    span: Default::default(),
                });

                for inner_stmt in body {
                    self.analyze_statement(inner_stmt);
                }
                self.exit_scope();
                self.in_loop = old_loop;
            }
            Statement::ForCStyle {
                initializer,
                condition,
                increment,
                body,
                ..
            } => {
                let old_loop = self.in_loop;
                self.in_loop = true;
                self.enter_scope();

                self.analyze_statement(initializer);
                self.analyze_expression(condition);
                self.analyze_statement(increment);

                for inner_stmt in body {
                    self.analyze_statement(inner_stmt);
                }
                self.exit_scope();
                self.in_loop = old_loop;
            }
            Statement::Break(span) => {
                if !self.in_loop {
                    self.errors
                        .push(SemanticError::BreakOutsideLoop { span: *span });
                }
            }
            Statement::Continue(span) => {
                if !self.in_loop {
                    self.errors
                        .push(SemanticError::ContinueOutsideLoop { span: *span });
                }
            }
            Statement::Expression(expr) => {
                self.analyze_expression(expr);
            }
            Statement::Block(stmts) => {
                self.enter_scope();
                for inner_stmt in stmts {
                    self.analyze_statement(inner_stmt);
                }
                self.exit_scope();
            }
            _ => {}
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Literal(lit, _) => match lit {
                Literal::Number(_) => Type::Int,
                Literal::String(_) => Type::String,
                Literal::Boolean(_) => Type::Bool,
                Literal::Null => Type::Unknown,
            },
            Expression::Identifier(name, span) => {
                if name == "null" {
                    return Type::Unknown;
                }
                if let Some(symbol) = self.resolve(name) {
                    symbol.type_.clone()
                } else {
                    self.errors.push(SemanticError::UndefinedVariable {
                        name: name.clone(),
                        span: *span,
                    });
                    Type::Unknown
                }
            }
            Expression::Binary {
                left,
                right,
                operator,
                span,
                ..
            } => {
                let _left_type = self.analyze_expression(left);
                let _right_type = self.analyze_expression(right);

                if matches!(
                    operator,
                    TokenKind::Assign
                        | TokenKind::PlusEqual
                        | TokenKind::MinusEqual
                        | TokenKind::MultiplyEqual
                        | TokenKind::DivideEqual
                ) {
                    if let Expression::Identifier(name, _) = &**left {
                        if self.resolve(name).is_none() {
                            self.errors.push(SemanticError::UndefinedVariable {
                                name: name.clone(),
                                span: *span,
                            });
                        }
                    }
                }
                Type::Unknown
            }
            Expression::Call {
                callee, arguments, ..
            } => {
                let _callee_type = self.analyze_expression(callee);
                for arg in arguments {
                    self.analyze_expression(arg);
                }
                Type::Unknown
            }
            Expression::Array(elements, _) => {
                for elem in elements {
                    self.analyze_expression(elem);
                }
                Type::Array(Box::new(Type::Unknown))
            }
            Expression::Object(props, _) => {
                for (_, val) in props {
                    self.analyze_expression(val);
                }
                Type::Unknown
            }
            Expression::Index { object, index, .. } => {
                self.analyze_expression(object);
                self.analyze_expression(index);
                Type::Unknown
            }
            Expression::MemberAccess { object, .. } => {
                self.analyze_expression(object);
                Type::Unknown
            }
            Expression::Unary { operand, .. } => self.analyze_expression(operand),
            Expression::Range { start, end, .. } => {
                self.analyze_expression(start);
                self.analyze_expression(end);
                Type::Unknown
            }
            _ => Type::Unknown,
        }
    }

    fn enter_scope(&mut self) {
        let new_scope = Scope {
            symbols: HashMap::new(),
            parent: Some(self.current_scope),
        };
        self.scopes.push(new_scope);
        self.current_scope = self.scopes.len() - 1;
    }

    fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    fn define(&mut self, symbol: Symbol) -> Result<(), ()> {
        let scope = &mut self.scopes[self.current_scope];
        if self.pass == 2 && scope.symbols.contains_key(&symbol.name) && self.current_scope != 0 {
            return Err(());
        }
        scope.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    fn resolve(&self, name: &str) -> Option<&Symbol> {
        let mut scope_idx = Some(self.current_scope);
        while let Some(idx) = scope_idx {
            let scope = &self.scopes[idx];
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
            scope_idx = scope.parent;
        }
        None
    }
}
