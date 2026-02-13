//! Zenith High-level Intermediate Representation (HIR)
//!
//! This module defines the HIR for the Zenith programming language.
//! HIR is a type-checked, validated intermediate representation that sits
//! between the AST and lower-level representations like MIR or LLVM IR.

use std::collections::HashMap;
use zenith_lexer::Span;
use zenith_parser::Program;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Int,
    Float,
    String,
    Bool,
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Unknown,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Function {
                params,
                return_type,
            } => {
                write!(
                    f,
                    "({}) -> {}",
                    params
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    return_type
                )
            }
            Type::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal, Type),
    Identifier(String, Type),
    Binary {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
        type_: Type,
    },
    Unary {
        operator: String,
        operand: Box<Expression>,
        type_: Type,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        type_: Type,
    },
    Lambda {
        parameters: Vec<(String, Type)>,
        body: Box<Statement>,
        type_: Type,
    },
    MemberAccess {
        object: Box<Expression>,
        property: String,
        type_: Type,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
        type_: Type,
    },
    Array(Vec<Expression>, Type),
    Object(Vec<(String, Expression)>, Type),
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        type_: Type,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDeclaration {
        name: String,
        type_: Type,
        initializer: Option<Expression>,
    },
    FuncDeclaration {
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    },
    ReturnStatement {
        value: Option<Expression>,
    },
    IfStatement {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    WhileStatement {
        condition: Expression,
        body: Box<Statement>,
    },
    ForStatement {
        variable: String,
        iterable: Expression,
        body: Box<Statement>,
    },
    ForCStyle {
        initializer: Box<Statement>,
        condition: Expression,
        increment: Box<Statement>,
        body: Box<Statement>,
    },
    MatchStatement {
        expression: Expression,
        arms: Vec<(Expression, Vec<Statement>)>,
    },
    Block(Vec<Statement>),
    Import {
        path: String,
        symbols: Vec<String>,
    },
    Expression(Expression),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Function(Statement),
    Variable(Statement),
    Statement(Statement),
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Enum {
        name: String,
        variants: Vec<String>,
    },
    Trait {
        name: String,
        methods: Vec<Statement>,
    },
    Impl {
        target: String,
        methods: Vec<Statement>,
    },
    Import {
        path: String,
        symbols: Vec<String>,
        alias: Option<String>,
    },
    Export {
        declaration: Box<Declaration>,
    },
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub declarations: Vec<Declaration>,
    pub symbols: HashMap<String, Symbol>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub type_: Type,
    pub is_mutable: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct HirProgram {
    pub modules: Vec<Module>,
    pub main_module: String,
}

pub struct HirBuilder {
    _current_module: String,
    symbols: HashMap<String, Symbol>,
}

impl HirBuilder {
    pub fn new() -> Self {
        Self {
            _current_module: "main".to_string(),
            symbols: HashMap::new(),
        }
    }

    pub fn build(&mut self, ast_program: &Program) -> HirProgram {
        let mut hir_declarations = Vec::new();

        // Convert AST declarations to HIR
        for declaration in &ast_program.declarations {
            let hir_decl = self.lower_declaration(declaration);
            hir_declarations.push(hir_decl);
        }

        let main_module = Module {
            name: "main".to_string(),
            declarations: hir_declarations,
            symbols: self.symbols.clone(),
        };

        HirProgram {
            modules: vec![main_module],
            main_module: "main".to_string(),
        }
    }

    fn lower_declaration(&mut self, declaration: &zenith_parser::Declaration) -> Declaration {
        match declaration {
            zenith_parser::Declaration::Function(func) => self.lower_function_declaration(func),
            zenith_parser::Declaration::Variable(var) => self.lower_variable_declaration(var),
            zenith_parser::Declaration::Statement(stmt) => {
                // Handle statement declarations
                match stmt {
                    zenith_parser::Statement::FuncDeclaration { .. } => {
                        self.lower_function_declaration(stmt)
                    }
                    zenith_parser::Statement::VarDeclaration { .. } => {
                        self.lower_variable_declaration(stmt)
                    }
                    _ => Declaration::Statement(self.lower_statement(stmt)),
                }
            }
            zenith_parser::Declaration::Struct {
                name,
                fields,
                span: _,
            } => {
                let hir_fields = fields
                    .iter()
                    .map(|(name, type_)| (name.clone(), self.parse_type(type_)))
                    .collect();

                Declaration::Struct {
                    name: name.clone(),
                    fields: hir_fields,
                }
            }
            zenith_parser::Declaration::Enum {
                name,
                variants,
                span: _,
            } => Declaration::Enum {
                name: name.clone(),
                variants: variants.clone(),
            },
            zenith_parser::Declaration::Trait {
                name,
                methods,
                span: _,
            } => {
                let hir_methods = methods
                    .iter()
                    .map(|stmt| self.lower_statement(stmt))
                    .collect();

                Declaration::Trait {
                    name: name.clone(),
                    methods: hir_methods,
                }
            }
            zenith_parser::Declaration::Impl {
                target,
                methods,
                span: _,
            } => {
                let hir_methods = methods
                    .iter()
                    .map(|stmt| self.lower_statement(stmt))
                    .collect();

                Declaration::Impl {
                    target: target.clone(),
                    methods: hir_methods,
                }
            }
            zenith_parser::Declaration::Import {
                path,
                symbols,
                alias,
                span: _,
            } => Declaration::Import {
                path: path.clone(),
                symbols: symbols.clone(),
                alias: alias.clone(),
            },
            zenith_parser::Declaration::Export {
                declaration,
                span: _,
            } => {
                let hir_decl = self.lower_declaration(declaration);
                Declaration::Export {
                    declaration: Box::new(hir_decl),
                }
            }
        }
    }

    fn lower_function_declaration(&mut self, func: &zenith_parser::Statement) -> Declaration {
        match func {
            zenith_parser::Statement::FuncDeclaration {
                name,
                parameters,
                return_type,
                body,
                span,
            } => {
                let param_types: Vec<(String, Type)> = parameters
                    .iter()
                    .map(|(name, type_annotation)| {
                        let type_ = type_annotation
                            .as_ref()
                            .map(|t| self.parse_type(t))
                            .unwrap_or(Type::Unknown);
                        (name.clone(), type_)
                    })
                    .collect();

                let return_type = return_type
                    .as_ref()
                    .map(|t| self.parse_type(t))
                    .unwrap_or(Type::Unknown);

                let func_type = Type::Function {
                    params: param_types.iter().map(|(_, t)| t.clone()).collect(),
                    return_type: Box::new(return_type.clone()),
                };

                // Add function to symbols
                self.symbols.insert(
                    name.clone(),
                    Symbol {
                        name: name.clone(),
                        type_: func_type,
                        is_mutable: false,
                        span: *span,
                    },
                );

                let hir_body = self.lower_statements(body);

                Declaration::Function(Statement::FuncDeclaration {
                    name: name.clone(),
                    parameters: param_types,
                    return_type,
                    body: hir_body,
                })
            }
            _ => panic!("Expected function declaration"),
        }
    }

    fn lower_variable_declaration(&mut self, var: &zenith_parser::Statement) -> Declaration {
        match var {
            zenith_parser::Statement::VarDeclaration {
                name,
                type_annotation,
                initializer,
                span,
            } => {
                let type_ = type_annotation
                    .as_ref()
                    .map(|t| self.parse_type(t))
                    .unwrap_or(Type::Unknown);

                let hir_initializer = initializer.as_ref().map(|expr| self.lower_expression(expr));

                // Add variable to symbols
                self.symbols.insert(
                    name.clone(),
                    Symbol {
                        name: name.clone(),
                        type_: type_.clone(),
                        is_mutable: true,
                        span: *span,
                    },
                );

                Declaration::Variable(Statement::VarDeclaration {
                    name: name.clone(),
                    type_,
                    initializer: hir_initializer,
                })
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[allow(dead_code)]
    fn lower_struct_declaration(&self, struct_decl: &zenith_parser::Declaration) -> Declaration {
        match struct_decl {
            zenith_parser::Declaration::Struct {
                name,
                fields,
                span: _,
            } => {
                let hir_fields = fields
                    .iter()
                    .map(|(name, type_)| (name.clone(), self.parse_type(type_)))
                    .collect();

                Declaration::Struct {
                    name: name.clone(),
                    fields: hir_fields,
                }
            }
            _ => panic!("Expected struct declaration"),
        }
    }

    #[allow(dead_code)]
    fn lower_enum_declaration(&self, enum_decl: &zenith_parser::Declaration) -> Declaration {
        match enum_decl {
            zenith_parser::Declaration::Enum {
                name,
                variants,
                span: _,
            } => Declaration::Enum {
                name: name.clone(),
                variants: variants.clone(),
            },
            _ => panic!("Expected enum declaration"),
        }
    }

    #[allow(dead_code)]
    fn lower_trait_declaration(&mut self, trait_decl: &zenith_parser::Declaration) -> Declaration {
        match trait_decl {
            zenith_parser::Declaration::Trait {
                name,
                methods,
                span: _,
            } => {
                let hir_methods = methods
                    .iter()
                    .map(|stmt| self.lower_statement(stmt))
                    .collect();

                Declaration::Trait {
                    name: name.clone(),
                    methods: hir_methods,
                }
            }
            _ => panic!("Expected trait declaration"),
        }
    }

    #[allow(dead_code)]
    fn lower_impl_declaration(&mut self, impl_decl: &zenith_parser::Declaration) -> Declaration {
        match impl_decl {
            zenith_parser::Declaration::Impl {
                target,
                methods,
                span: _,
            } => {
                let hir_methods = methods
                    .iter()
                    .map(|stmt| self.lower_statement(stmt))
                    .collect();

                Declaration::Impl {
                    target: target.clone(),
                    methods: hir_methods,
                }
            }
            _ => panic!("Expected impl declaration"),
        }
    }

    #[allow(dead_code)]
    fn lower_import_declaration(&self, import_decl: &zenith_parser::Declaration) -> Declaration {
        match import_decl {
            zenith_parser::Declaration::Import {
                path,
                symbols,
                alias,
                span: _,
            } => Declaration::Import {
                path: path.clone(),
                symbols: symbols.clone(),
                alias: alias.clone(),
            },
            _ => panic!("Expected import declaration"),
        }
    }

    #[allow(dead_code)]
    fn lower_export_declaration(
        &mut self,
        export_decl: &zenith_parser::Declaration,
    ) -> Declaration {
        match export_decl {
            zenith_parser::Declaration::Export {
                declaration,
                span: _,
            } => {
                let hir_decl = self.lower_declaration(declaration);
                Declaration::Export {
                    declaration: Box::new(hir_decl),
                }
            }
            _ => panic!("Expected export declaration"),
        }
    }

    fn lower_statements(&mut self, statements: &[zenith_parser::Statement]) -> Vec<Statement> {
        statements
            .iter()
            .map(|stmt| self.lower_statement(stmt))
            .collect()
    }

    fn lower_statement(&mut self, statement: &zenith_parser::Statement) -> Statement {
        match statement {
            zenith_parser::Statement::VarDeclaration {
                name,
                type_annotation,
                initializer,
                span,
            } => {
                let type_ = type_annotation
                    .as_ref()
                    .map(|t| self.parse_type(t))
                    .unwrap_or(Type::Unknown);

                let hir_initializer = initializer.as_ref().map(|expr| self.lower_expression(expr));

                // Add variable to symbols
                self.symbols.insert(
                    name.clone(),
                    Symbol {
                        name: name.clone(),
                        type_: type_.clone(),
                        is_mutable: true,
                        span: *span,
                    },
                );

                Statement::VarDeclaration {
                    name: name.clone(),
                    type_,
                    initializer: hir_initializer,
                }
            }
            zenith_parser::Statement::FuncDeclaration {
                name,
                parameters,
                return_type,
                body,
                span: _,
            } => {
                let param_types: Vec<(String, Type)> = parameters
                    .iter()
                    .map(|(name, type_annotation)| {
                        let type_ = type_annotation
                            .as_ref()
                            .map(|t| self.parse_type(t))
                            .unwrap_or(Type::Unknown);
                        (name.clone(), type_)
                    })
                    .collect();

                let return_type = return_type
                    .as_ref()
                    .map(|t| self.parse_type(t))
                    .unwrap_or(Type::Void);

                let hir_body = body.iter().map(|stmt| self.lower_statement(stmt)).collect();

                Statement::FuncDeclaration {
                    name: name.clone(),
                    parameters: param_types,
                    return_type,
                    body: hir_body,
                }
            }
            zenith_parser::Statement::ReturnStatement { value, span: _ } => {
                let hir_value = value.as_ref().map(|expr| self.lower_expression(expr));

                Statement::ReturnStatement { value: hir_value }
            }
            zenith_parser::Statement::IfStatement {
                condition,
                then_branch,
                else_branch,
                span: _,
            } => {
                let hir_condition = self.lower_expression(condition);
                let hir_then = self.lower_statements(then_branch);
                let hir_else = else_branch
                    .as_ref()
                    .map(|stmts| self.lower_statements(stmts));

                Statement::IfStatement {
                    condition: hir_condition,
                    then_branch: Box::new(Statement::Block(hir_then)),
                    else_branch: hir_else.map(|stmts| Box::new(Statement::Block(stmts))),
                }
            }
            zenith_parser::Statement::WhileStatement {
                condition,
                body,
                span: _,
            } => {
                let hir_condition = self.lower_expression(condition);
                let hir_body = self.lower_statements(body);

                Statement::WhileStatement {
                    condition: hir_condition,
                    body: Box::new(Statement::Block(hir_body)),
                }
            }
            zenith_parser::Statement::ForStatement {
                variable,
                iterable,
                body,
                span: _,
            } => {
                let hir_iterable = self.lower_expression(iterable);
                let hir_body = self.lower_statements(body);

                Statement::ForStatement {
                    variable: variable.clone(),
                    iterable: hir_iterable,
                    body: Box::new(Statement::Block(hir_body)),
                }
            }
            zenith_parser::Statement::ForCStyle {
                initializer,
                condition,
                increment,
                body,
                span: _,
            } => {
                let hir_initializer = self.lower_statement(initializer);
                let hir_condition = self.lower_expression(condition);
                let hir_increment = self.lower_statement(increment);
                let hir_body = self.lower_statements(body);

                Statement::ForCStyle {
                    initializer: Box::new(hir_initializer),
                    condition: hir_condition,
                    increment: Box::new(hir_increment),
                    body: Box::new(Statement::Block(hir_body)),
                }
            }
            zenith_parser::Statement::MatchStatement {
                expression,
                arms,
                span: _,
            } => {
                let hir_expression = self.lower_expression(expression);
                let hir_arms = arms
                    .iter()
                    .map(|(expr, stmts)| {
                        let hir_expr = self.lower_expression(expr);
                        let hir_stmts = stmts
                            .iter()
                            .map(|stmt| self.lower_statement(stmt))
                            .collect();
                        (hir_expr, hir_stmts)
                    })
                    .collect();

                Statement::MatchStatement {
                    expression: hir_expression,
                    arms: hir_arms,
                }
            }
            zenith_parser::Statement::Block(statements) => {
                let hir_statements = statements
                    .iter()
                    .map(|stmt| self.lower_statement(stmt))
                    .collect();

                Statement::Block(hir_statements)
            }
            zenith_parser::Statement::Expression(expr) => {
                let hir_expr = self.lower_expression(expr);
                Statement::Expression(hir_expr)
            }
            zenith_parser::Statement::Import { path, symbols, .. } => Statement::Import {
                path: path.clone(),
                symbols: symbols.clone(),
            },
            zenith_parser::Statement::Break(_) => Statement::Break,
            zenith_parser::Statement::Continue(_) => Statement::Continue,
        }
    }

    fn convert_literal(&self, lit: &zenith_parser::Literal) -> Literal {
        match lit {
            zenith_parser::Literal::String(s) => Literal::String(s.clone()),
            zenith_parser::Literal::Number(n) => Literal::Number(n.clone()),
            zenith_parser::Literal::Boolean(b) => Literal::Boolean(*b),
            zenith_parser::Literal::Null => Literal::Null,
        }
    }

    fn lower_expression(&mut self, expression: &zenith_parser::Expression) -> Expression {
        match expression {
            zenith_parser::Expression::Literal(lit, _span) => {
                let type_ = self.infer_literal_type(lit);
                Expression::Literal(self.convert_literal(lit), type_)
            }
            zenith_parser::Expression::Identifier(name, _span) => {
                let type_ = self.lookup_symbol_type(name).unwrap_or(Type::Unknown);
                Expression::Identifier(name.clone(), type_)
            }
            zenith_parser::Expression::Binary {
                left,
                operator,
                right,
                span: _,
            } => {
                let hir_left = Box::new(self.lower_expression(left));
                let hir_right = Box::new(self.lower_expression(right));
                let type_ = self.infer_binary_type(&hir_left, operator, &hir_right);

                Expression::Binary {
                    left: hir_left,
                    operator: format!("{:?}", operator),
                    right: hir_right,
                    type_,
                }
            }
            zenith_parser::Expression::Unary {
                operator,
                operand,
                span: _,
            } => {
                let hir_operand = Box::new(self.lower_expression(operand));
                let type_ = self.infer_unary_type(&hir_operand, operator);

                Expression::Unary {
                    operator: format!("{:?}", operator),
                    operand: hir_operand,
                    type_,
                }
            }
            zenith_parser::Expression::Range { start, end, .. } => {
                let hir_start = Box::new(self.lower_expression(start));
                let hir_end = Box::new(self.lower_expression(end));

                Expression::Range {
                    start: hir_start,
                    end: hir_end,
                    type_: Type::Unknown, // TODO: Add Type::Range
                }
            }
            zenith_parser::Expression::Call {
                callee,
                arguments,
                span: _,
            } => {
                let hir_callee = Box::new(self.lower_expression(callee));
                let hir_arguments: Vec<Expression> = arguments
                    .iter()
                    .map(|arg| self.lower_expression(arg))
                    .collect();
                let type_ = self.infer_call_type(&hir_callee, &hir_arguments);

                Expression::Call {
                    callee: hir_callee,
                    arguments: hir_arguments,
                    type_,
                }
            }
            zenith_parser::Expression::Lambda {
                parameters,
                body,
                span: _,
            } => {
                let hir_body = Box::new(self.lower_statement(body));
                let hir_parameters: Vec<(String, Type)> = parameters
                    .iter()
                    .map(|param| (param.clone(), Type::Unknown))
                    .collect();
                let type_ = Type::Unknown; // TODO: Implement proper lambda type inference

                Expression::Lambda {
                    parameters: hir_parameters,
                    body: hir_body,
                    type_,
                }
            }
            zenith_parser::Expression::MemberAccess {
                object,
                property,
                span: _,
            } => {
                let hir_object = Box::new(self.lower_expression(object));
                let type_ = Type::Unknown; // TODO: Implement proper member access type inference

                Expression::MemberAccess {
                    object: hir_object,
                    property: property.clone(),
                    type_,
                }
            }
            zenith_parser::Expression::Index {
                object,
                index,
                span: _,
            } => {
                let hir_object = Box::new(self.lower_expression(object));
                let hir_index = Box::new(self.lower_expression(index));
                let type_ = Type::Unknown; // TODO: Implement proper index type inference

                Expression::Index {
                    object: hir_object,
                    index: hir_index,
                    type_,
                }
            }
            zenith_parser::Expression::Array(elements, _) => {
                let hir_elements = elements.iter().map(|e| self.lower_expression(e)).collect();
                Expression::Array(hir_elements, Type::Unknown)
            }
            zenith_parser::Expression::Object(properties, _) => {
                let hir_properties = properties
                    .iter()
                    .map(|(k, v)| (k.clone(), self.lower_expression(v)))
                    .collect();
                Expression::Object(hir_properties, Type::Unknown)
            }
        }
    }

    fn parse_type(&self, type_str: &str) -> Type {
        match type_str {
            "int" => Type::Int,
            "float" => Type::Float,
            "string" => Type::String,
            "bool" => Type::Bool,
            "void" => Type::Void,
            _ => Type::Unknown,
        }
    }

    fn infer_literal_type(&self, literal: &zenith_parser::Literal) -> Type {
        match literal {
            zenith_parser::Literal::String(_) => Type::String,
            zenith_parser::Literal::Number(_) => Type::Int, // TODO: Distinguish int vs float
            zenith_parser::Literal::Boolean(_) => Type::Bool,
            zenith_parser::Literal::Null => Type::Unknown,
        }
    }

    fn lookup_symbol_type(&self, name: &str) -> Option<Type> {
        self.symbols.get(name).map(|symbol| symbol.type_.clone())
    }

    fn infer_binary_type(
        &self,
        _left: &Expression,
        _operator: &zenith_lexer::TokenKind,
        _right: &Expression,
    ) -> Type {
        // TODO: Implement proper binary operation type inference
        Type::Unknown
    }

    fn infer_unary_type(&self, _operand: &Expression, _operator: &zenith_lexer::TokenKind) -> Type {
        // TODO: Implement proper unary operation type inference
        Type::Unknown
    }

    fn infer_call_type(&self, _callee: &Expression, _arguments: &[Expression]) -> Type {
        // TODO: Implement proper function call type inference
        Type::Unknown
    }
}
