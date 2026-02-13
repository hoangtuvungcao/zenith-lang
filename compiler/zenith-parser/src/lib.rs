//! Zenith Language Parser
//!
//! This module provides parsing capabilities for the Zenith programming language.
//! It converts tokens from the lexer into an Abstract Syntax Tree (AST).

use thiserror::Error;
use zenith_lexer::{Lexer, Position, Span, Token, TokenKind};
// use std::io::Write;
use std::collections::VecDeque;

#[derive(Error, Debug, Clone)]
pub enum ParserError {
    #[error("Unexpected token: expected {expected:?}, found {found:?} at {span}")]
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
        span: Span,
    },

    #[error("Unexpected end of file at position {position:?}")]
    UnexpectedEOF {
        expected: TokenKind,
        position: Position,
    },

    #[error("Invalid syntax: {message} at {}", span)]
    InvalidSyntax { message: String, span: Span },

    #[error("Indentation error: {message} at {}", span)]
    IndentationError { message: String, span: Span },

    #[error("Multiple errors occurred")]
    MultipleErrors(Vec<ParserError>),
}

// AST Node Types

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal, Span),
    Identifier(String, Span),
    Binary {
        left: Box<Expression>,
        operator: TokenKind,
        right: Box<Expression>,
        span: Span,
    },
    Unary {
        operator: TokenKind,
        operand: Box<Expression>,
        span: Span,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },
    Lambda {
        parameters: Vec<String>,
        body: Box<Statement>,
        span: Span,
    },
    MemberAccess {
        object: Box<Expression>,
        property: String,
        span: Span,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
        span: Span,
    },
    Array(Vec<Expression>, Span),
    Object(Vec<(String, Expression)>, Span),
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        span: Span,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VarDeclaration {
        name: String,
        type_annotation: Option<String>,
        initializer: Option<Expression>,
        span: Span,
    },
    FuncDeclaration {
        name: String,
        parameters: Vec<(String, Option<String>)>,
        return_type: Option<String>,
        body: Vec<Statement>,
        span: Span,
    },
    ReturnStatement {
        value: Option<Expression>,
        span: Span,
    },
    IfStatement {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
        span: Span,
    },
    WhileStatement {
        condition: Expression,
        body: Vec<Statement>,
        span: Span,
    },
    ForStatement {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
        span: Span,
    },
    ForCStyle {
        initializer: Box<Statement>,
        condition: Expression,
        increment: Box<Statement>,
        body: Vec<Statement>,
        span: Span,
    },
    MatchStatement {
        expression: Expression,
        arms: Vec<(Expression, Vec<Statement>)>,
        span: Span,
    },
    Import {
        path: String,
        symbols: Vec<String>,
        span: Span,
    },
    Break(Span),
    Continue(Span),
    Block(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Function(Statement),
    Variable(Statement),
    Statement(Statement),
    Struct {
        name: String,
        fields: Vec<(String, String)>,
        span: Span,
    },
    Enum {
        name: String,
        variants: Vec<String>,
        span: Span,
    },
    Trait {
        name: String,
        methods: Vec<Statement>,
        span: Span,
    },
    Impl {
        target: String,
        methods: Vec<Statement>,
        span: Span,
    },
    Import {
        path: String,
        symbols: Vec<String>,
        alias: Option<String>,
        span: Span,
    },
    Export {
        declaration: Box<Declaration>,
        span: Span,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    errors: Vec<ParserError>,
    _current_indent: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
            errors: Vec::new(),
            _current_indent: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, Vec<ParserError>> {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            // Skip newlines between declarations
            while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) {
                self.advance();
            }

            if self.is_at_end() {
                break;
            }

            if let Some(declaration) = self.parse_declaration() {
                declarations.push(declaration);
            } else {
                // If we couldn't parse a declaration, skip the current token to avoid infinite loops
                self.advance();
            }
        }

        if self.errors.is_empty() {
            Ok(Program { declarations })
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        match self.peek()?.kind.clone() {
            TokenKind::Func => self.parse_function_declaration(),
            TokenKind::Struct => self.parse_struct_declaration(),
            TokenKind::Enum => self.parse_enum_declaration(),
            TokenKind::Trait => self.parse_trait_declaration(),
            TokenKind::Impl => self.parse_impl_declaration(),
            TokenKind::Import => self.parse_import_declaration(),
            TokenKind::Export => self.parse_export_declaration(),
            _ => self.parse_statement_declaration(),
        }
    }

    fn parse_function_declaration(&mut self) -> Option<Declaration> {
        let start_span = self.peek()?.span;
        self.advance(); // consume 'func'

        let name = match self.peek()?.kind.clone() {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => {
                self.errors.push(ParserError::UnexpectedToken {
                    expected: TokenKind::Identifier("".to_string()),
                    found: self.peek().unwrap().kind.clone(),
                    span: self.peek().unwrap().span,
                });
                return None;
            }
        };

        // Skip newlines before '('
        while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) {
            self.advance();
        }

        if self.peek()?.kind != TokenKind::LeftParen {
            return None;
        }
        self.advance(); // consume '('

        let mut parameters = Vec::new();
        while !self.is_at_end() && self.peek()?.kind != TokenKind::RightParen {
            if let TokenKind::Identifier(param_name) = &self.peek()?.kind {
                let param_name = param_name.clone();
                self.advance();

                // Optional type annotation
                let type_annotation = if self.peek()?.kind == TokenKind::Colon {
                    self.advance(); // consume ':'
                    if let TokenKind::Identifier(type_name) = &self.peek()?.kind {
                        let type_name = type_name.clone();
                        self.advance();
                        Some(type_name)
                    } else {
                        None
                    }
                } else {
                    None
                };

                parameters.push((param_name, type_annotation));

                if self.peek()?.kind == TokenKind::Comma {
                    self.advance();
                }
            } else {
                break;
            }
        }

        if self.peek()?.kind != TokenKind::RightParen {
            return None;
        }
        self.advance(); // consume ')'

        // Optional return type
        let return_type = if self.peek()?.kind == TokenKind::Colon {
            self.advance(); // consume ':'
            if let TokenKind::Identifier(type_name) = &self.peek()?.kind {
                let type_name = type_name.clone();
                self.advance();
                Some(type_name)
            } else {
                None
            }
        } else {
            None
        };

        // Skip newlines before body
        while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) {
            self.advance();
        }

        // Parse function body - either a block or a single expression
        let body = if self.peek()?.kind == TokenKind::LeftBrace
            || self.peek()?.kind == TokenKind::Indent
        {
            // Block-based function
            self.parse_block()?
        } else if self.peek()?.kind == TokenKind::Arrow {
            // Expression-based function - consume the arrow and parse the expression
            self.advance(); // consume '->'
            match self.parse_expression() {
                Some(expr) => vec![Statement::Expression(expr)],
                None => return None,
            }
        } else {
            // No body found (or signature only)
            vec![]
        };

        let result = Some(Declaration::Function(Statement::FuncDeclaration {
            name,
            parameters,
            return_type,
            body,
            span: start_span,
        }));
        result
    }

    fn parse_struct_declaration(&mut self) -> Option<Declaration> {
        let start_span = self.peek()?.span;
        self.advance(); // consume 'struct'

        let name = match self.peek()?.kind.clone() {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => {
                self.errors.push(ParserError::UnexpectedToken {
                    expected: TokenKind::Identifier("".to_string()),
                    found: self.peek().unwrap().kind.clone(),
                    span: self.peek().unwrap().span,
                });
                return None;
            }
        };

        if self.peek()?.kind != TokenKind::LeftBrace {
            return None;
        }
        self.advance(); // consume '{'

        let mut fields = Vec::new();
        while !self.is_at_end() && self.peek()?.kind != TokenKind::RightBrace {
            // Skip whitespace/newlines
            if matches!(
                self.peek()?.kind,
                TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
            ) {
                self.advance();
                continue;
            }

            if let TokenKind::Identifier(field_name) = &self.peek()?.kind {
                let field_name = field_name.clone();
                self.advance();

                if self.peek()?.kind != TokenKind::Colon {
                    return None;
                }
                self.advance(); // consume ':'

                if let TokenKind::Identifier(type_name) = &self.peek()?.kind {
                    let type_name = type_name.clone();
                    self.advance();
                    fields.push((field_name, type_name));
                }

                if self.peek()?.kind == TokenKind::Comma {
                    self.advance();
                }
            } else {
                break;
            }
        }

        if self.peek()?.kind != TokenKind::RightBrace {
            return None;
        }
        self.advance(); // consume '}'

        Some(Declaration::Struct {
            name,
            fields,
            span: start_span,
        })
    }

    fn parse_enum_declaration(&mut self) -> Option<Declaration> {
        let start_span = self.peek()?.span;
        self.advance(); // consume 'enum'

        let name = match self.peek()?.kind.clone() {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => {
                self.errors.push(ParserError::UnexpectedToken {
                    expected: TokenKind::Identifier("".to_string()),
                    found: self.peek().unwrap().kind.clone(),
                    span: self.peek().unwrap().span,
                });
                return None;
            }
        };

        if self.peek()?.kind != TokenKind::LeftBrace {
            return None;
        }
        self.advance(); // consume '{'

        let mut variants = Vec::new();
        while !self.is_at_end() && self.peek()?.kind != TokenKind::RightBrace {
            if let TokenKind::Identifier(variant_name) = &self.peek()?.kind {
                let variant_name = variant_name.clone();
                self.advance();
                variants.push(variant_name);

                if self.peek()?.kind == TokenKind::Comma {
                    self.advance();
                }
            } else {
                break;
            }
        }

        if self.peek()?.kind != TokenKind::RightBrace {
            return None;
        }
        self.advance(); // consume '}'

        Some(Declaration::Enum {
            name,
            variants,
            span: start_span,
        })
    }

    fn parse_trait_declaration(&mut self) -> Option<Declaration> {
        // Placeholder for trait declaration
        None
    }

    fn parse_impl_declaration(&mut self) -> Option<Declaration> {
        let _start_span = self.peek()?.span;
        self.advance(); // consume 'impl'

        let name = match self.peek()?.kind.clone() {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return None,
        };

        if self.peek()?.kind != TokenKind::LeftBrace {
            return None;
        }
        self.advance(); // consume '{'

        let mut functions = Vec::new();
        while !self.is_at_end() && self.peek()?.kind != TokenKind::RightBrace {
            // Skip whitespace/newlines
            if matches!(
                self.peek()?.kind,
                TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
            ) {
                self.advance();
                continue;
            }

            if let Some(Declaration::Function(stmt)) = self.parse_function_declaration() {
                functions.push(stmt);
            } else {
                // Skip unexpected tokens to avoid infinite loops
                self.advance();
            }
        }

        if self.peek()?.kind != TokenKind::RightBrace {
            return None;
        }
        self.advance(); // consume '}'

        Some(Declaration::Impl {
            target: name,
            methods: functions,
            span: _start_span,
        })
    }

    fn parse_import_declaration(&mut self) -> Option<Declaration> {
        let start_span = self.peek()?.span;
        self.advance(); // consume 'import'

        let mut path = String::new();
        let mut loop_count = 0;
        loop {
            loop_count += 1;
            if loop_count > 100 {
                break;
            }

            if let TokenKind::Identifier(part) = &self.peek()?.kind {
                path.push_str(part);
                self.advance();
            } else {
                break;
            }

            if self.peek().map_or(false, |t| t.kind == TokenKind::Dot) {
                path.push('.');
                self.advance();
            } else {
                break;
            }
        }

        let mut symbols = Vec::new();
        // Optional { symbols }
        if self
            .peek()
            .map_or(false, |t| t.kind == TokenKind::LeftBrace)
        {
            self.advance(); // consume '{'
            while !self.is_at_end() && self.peek()?.kind != TokenKind::RightBrace {
                if let TokenKind::Identifier(sym) = &self.peek()?.kind {
                    symbols.push(sym.clone());
                    self.advance();
                    if self.peek().map_or(false, |t| t.kind == TokenKind::Comma) {
                        self.advance();
                    }
                } else {
                    break;
                }
            }
            if self
                .peek()
                .map_or(false, |t| t.kind == TokenKind::RightBrace)
            {
                self.advance(); // consume '}'
            }
            // For now, combine path and symbols into the string path or handle separately
            // Since Declaration::Import only has path: String, let's just use the path for now
            // and maybe append symbols to it or ignore them for the demo.
        }

        Some(Declaration::Import {
            path,
            symbols,
            alias: None,
            span: start_span,
        })
    }

    fn parse_export_declaration(&mut self) -> Option<Declaration> {
        // Placeholder for export declaration
        None
    }

    fn parse_statement_declaration(&mut self) -> Option<Declaration> {
        // Skip newlines before parsing
        while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) {
            self.advance();
        }

        if let Some(statement) = self.parse_statement() {
            match statement {
                Statement::FuncDeclaration { .. } => Some(Declaration::Function(statement)),
                Statement::VarDeclaration { .. } => Some(Declaration::Variable(statement)),
                Statement::Expression(_)
                | Statement::IfStatement { .. }
                | Statement::WhileStatement { .. }
                | Statement::ForStatement { .. }
                | Statement::ForCStyle { .. }
                | Statement::Block(_) => Some(Declaration::Statement(statement)),
                _ => None, // Other statements aren't top-level declarations
            }
        } else {
            None
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek()?.kind.clone() {
            TokenKind::Let | TokenKind::Var => self.parse_var_declaration(),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::While => self.parse_while_statement(),
            TokenKind::For => self.parse_for_statement(),
            TokenKind::Match => self.parse_match_statement(),
            TokenKind::Import => self.parse_local_import_statement(),
            TokenKind::LeftBrace | TokenKind::Indent => self.parse_block_statement(),
            TokenKind::Break => {
                let span = self.peek()?.span;
                self.advance();
                Some(Statement::Break(span))
            }
            TokenKind::Continue => {
                let span = self.peek()?.span;
                self.advance();
                Some(Statement::Continue(span))
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_var_declaration(&mut self) -> Option<Statement> {
        let start_span = self.peek()?.span;
        let _is_mutable = self.peek()?.kind == TokenKind::Var;
        self.advance(); // consume 'let' or 'var'

        let name = match self.peek()?.kind.clone() {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => {
                self.errors.push(ParserError::UnexpectedToken {
                    expected: TokenKind::Identifier("".to_string()),
                    found: self.peek().unwrap().kind.clone(),
                    span: self.peek().unwrap().span,
                });
                return None;
            }
        };

        // Optional type annotation
        let type_annotation = if self.peek()?.kind == TokenKind::Colon {
            self.advance(); // consume ':'
            if let TokenKind::Identifier(type_name) = &self.peek()?.kind {
                let type_name = type_name.clone();
                self.advance();
                Some(type_name)
            } else {
                None
            }
        } else {
            None
        };

        // Optional initializer
        let initializer = if self.peek()?.kind == TokenKind::Assign {
            self.advance(); // consume '='
            self.parse_expression()
        } else {
            None
        };

        Some(Statement::VarDeclaration {
            name,
            type_annotation,
            initializer,
            span: start_span,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let start_span = self.peek()?.span;
        self.advance(); // consume 'return'

        let value = if !self.is_at_end()
            && self.peek()?.kind != TokenKind::Newline
            && self.peek()?.kind != TokenKind::EOF
        {
            self.parse_expression()
        } else {
            None
        };

        Some(Statement::ReturnStatement {
            value,
            span: start_span,
        })
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        let start_span = self.peek()?.span;
        self.advance(); // consume 'if'

while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) { self.advance(); }
        let condition = self.parse_expression()?;

        let then_branch = self.parse_block()?;

        let mut else_branch = None;

        // Skip newlines/indentation before else
        while self.peek().map_or(false, |t| {
            matches!(
                t.kind,
                TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
            )
        }) {
            self.advance();
        }

        if self.peek().map_or(false, |t| t.kind == TokenKind::Else) {
            self.advance(); // consume 'else'
            if self.peek().map_or(false, |t| t.kind == TokenKind::If) {
                // else if
                if let Some(if_stmt) = self.parse_if_statement() {
                    else_branch = Some(vec![if_stmt]);
                }
            } else {
                // else block
                else_branch = self.parse_block();
            }
        }

        Some(Statement::IfStatement {
            condition,
            then_branch,
            else_branch,
            span: start_span,
        })
    }

    fn parse_while_statement(&mut self) -> Option<Statement> {
        let start_span = self.peek()?.span;
        self.advance(); // consume 'while'

while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) { self.advance(); }
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;

        Some(Statement::WhileStatement {
            condition,
            body,
            span: start_span,
        })
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
        let start_span = self.peek()?.span;
        self.advance(); // consume 'for'

        // Check if it's a C-style for loop or foreach-style
        let token_after_ident = if let Some(t) = self.tokens.get(1) {
            Some(t.kind.clone())
        } else {
            None
        };

        // Also check if current is 'var' or 'let' for C-style
        let is_c_style = if let Some(TokenKind::Assign) = token_after_ident {
            true
        } else {
            self.peek().map_or(false, |t| {
                t.kind == TokenKind::Var || t.kind == TokenKind::Let
            })
        };

        if is_c_style {
            // C-style for loop: for i = 0; i < 10; i = i + 1
            // OR for var i = 0; ...
            let initializer =
                if self.peek()?.kind == TokenKind::Var || self.peek()?.kind == TokenKind::Let {
                    self.parse_var_declaration()?
                } else {
                    Statement::Expression(self.parse_expression()?)
                };

            // Expect semicolon if not already consumed
            if self.peek()?.kind == TokenKind::Semicolon {
                self.advance(); // consume ';'
            }

while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) { self.advance(); }
            let condition = self.parse_expression()?;

            // Expect semicolon
            if self.peek()?.kind == TokenKind::Semicolon {
                self.advance(); // consume ';'
            }

while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) { self.advance(); }
            let increment = self.parse_expression()?;

            // Optional newline before block
            while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) {
                self.advance();
            }

            let body = self.parse_block()?;

            return Some(Statement::ForCStyle {
                initializer: Box::new(initializer),
                condition,
                increment: Box::new(Statement::Expression(increment)),
                body,
                span: start_span,
            });
        }

        // Foreach-style: for item in array
        if let TokenKind::Identifier(item) = self.peek()?.kind.clone() {
            self.advance();
            if self.peek()?.kind == TokenKind::In {
                self.advance(); // consume 'in'
                let iterable = self.parse_expression()?;

                // Optional newline before block
                while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) {
                    self.advance();
                }

                let body = self.parse_block()?;

                return Some(Statement::ForStatement {
                    variable: item,
                    iterable,
                    body,

                    span: start_span,
                });
            }
        }

        None
    }

    fn parse_match_statement(&mut self) -> Option<Statement> {
        // Placeholder for match statement
        None
    }

    fn parse_block_statement(&mut self) -> Option<Statement> {
        Some(Statement::Block(self.parse_block()?))
    }

    fn parse_block(&mut self) -> Option<Vec<Statement>> {
        let mut statements = Vec::new();

        // Handle both brace-based and indentation-based blocks
        if self.peek()?.kind == TokenKind::LeftBrace {
            self.advance(); // consume '{'

            while !self.is_at_end() && self.peek()?.kind != TokenKind::RightBrace {
                // Skip newlines and indentation
                while self.peek().map_or(false, |t| {
                    matches!(
                        t.kind,
                        TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                    )
                }) {
                    self.advance();
                }
                if self
                    .peek()
                    .map_or(false, |t| t.kind == TokenKind::RightBrace)
                {
                    break;
                }

                if let Some(statement) = self.parse_statement() {
                    statements.push(statement);
                } else {
                    break;
                }

                // Skip newlines and indentation between statements
                while self.peek().map_or(false, |t| {
                    matches!(
                        t.kind,
                        TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                    )
                }) {
                    self.advance();
                }
            }

            if self.peek()?.kind != TokenKind::RightBrace {
                return None;
            }
            self.advance(); // consume '}'
        } else if self.peek()?.kind == TokenKind::Indent {
            self.advance(); // consume indent

            while !self.is_at_end() && self.peek()?.kind != TokenKind::Dedent {
                if let Some(statement) = self.parse_statement() {
                    statements.push(statement);
                } else {
                    break;
                }

                // Skip newlines between statements
                while self.peek().map_or(false, |t| t.kind == TokenKind::Newline) {
                    self.advance();
                }
            }

            if self.peek()?.kind != TokenKind::Dedent {
                return None;
            }
            self.advance(); // consume dedent
        }

        Some(statements)
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        if let Some(expr) = self.parse_expression() {
            Some(Statement::Expression(expr))
        } else {
            None
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<Expression> {
        let mut left = self.parse_primary_expression()?;
        let mut loop_count = 0;
        const MAX_LOOP_COUNT: usize = 1000; // Safety check

        while !self.is_at_end() && loop_count < MAX_LOOP_COUNT {
            loop_count += 1;
            let token_span = self.peek()?.span;
            let token_kind = self.peek()?.kind.clone();

            // Stop parsing expression at newline
            if token_kind == TokenKind::Newline {
                break;
            }

            let token_precedence = self.get_precedence(&token_kind);

            // Don't process tokens with max precedence (like RightParen, EOF)
            if token_precedence == 255 {
                break;
            }

            if token_precedence < precedence {
                break;
            }

            self.advance();

            if token_kind == TokenKind::Range {
                // Determine precedence for the right side
                let next_precedence = if token_precedence >= 255 {
                    255
                } else {
                    token_precedence + 1
                };
                let right = self.parse_binary_expression(next_precedence)?;

                left = Expression::Range {
                    start: Box::new(left),
                    end: Box::new(right),
                    span: token_span,
                };
            } else if token_kind == TokenKind::LeftBracket {
                // Index
                let index = self.parse_expression()?;
                if self.peek()?.kind != TokenKind::RightBracket {
                    return None;
                }
                self.advance(); // ]
                left = Expression::Index {
                    object: Box::new(left),
                    index: Box::new(index),
                    span: token_span,
                };
            } else if token_kind == TokenKind::Dot {
                // Member Access
                let member_name = match self.peek()?.kind.clone() {
                    TokenKind::Identifier(n) => {
                        self.advance();
                        n
                    }
                    _ => return None,
                };
                left = Expression::MemberAccess {
                    object: Box::new(left),
                    property: member_name,
                    span: token_span,
                };
            } else if token_kind == TokenKind::LeftParen {
                // Call
                let mut arguments = Vec::new();
                while !self.is_at_end() && self.peek()?.kind != TokenKind::RightParen {
                    if let Some(arg) = self.parse_expression() {
                        arguments.push(arg);
                    } else {
                        break;
                    }
                    if self.peek().map_or(false, |t| t.kind == TokenKind::Comma) {
                        self.advance();
                    }
                }
                if self.peek()?.kind != TokenKind::RightParen {
                    return None;
                }
                self.advance(); // )
                left = Expression::Call {
                    callee: Box::new(left),
                    arguments,
                    span: token_span,
                };
            } else {
                // Binary Op
                let next_precedence = if token_precedence >= 255 {
                    255
                } else {
                    token_precedence + 1
                };
                let right = self.parse_binary_expression(next_precedence)?;
                left = Expression::Binary {
                    left: Box::new(left),
                    operator: token_kind,
                    right: Box::new(right),
                    span: token_span,
                };
            }
        }

        Some(left)
    }

    fn parse_interpolated_string(&self, content: &str, span: Span) -> Expression {
        let parts: Vec<&str> = content.split("${").collect();
        if parts.len() == 1 {
            return Expression::Literal(Literal::String(content.to_string()), span);
        }

        let mut expression_parts = Vec::new();
        // First part is literal string
        if !parts[0].is_empty() {
            expression_parts.push(Expression::Literal(
                Literal::String(parts[0].to_string()),
                span,
            ));
        }

        for part in parts.iter().skip(1) {
            if let Some(end_idx) = part.find('}') {
                let expr_str = &part[..end_idx];
                let rest_str = &part[end_idx + 1..];

                // Parse expr_str
                let lexer = Lexer::new(expr_str);
                if let Ok(tokens) = lexer.tokenize() {
                    let mut sub_parser = Parser::new(tokens);
                    if let Some(expr) = sub_parser.parse_expression() {
                        expression_parts.push(expr);
                    } else {
                        // If parsing fails, treat as literal string
                        expression_parts.push(Expression::Literal(
                            Literal::String(format!("${{{}}}", expr_str)),
                            span,
                        ));
                    }
                } else {
                    expression_parts.push(Expression::Literal(
                        Literal::String(format!("${{{}}}", expr_str)),
                        span,
                    ));
                }

                if !rest_str.is_empty() {
                    expression_parts.push(Expression::Literal(
                        Literal::String(rest_str.to_string()),
                        span,
                    ));
                }
            } else {
                // Unclosed brace, treat as literal
                expression_parts.push(Expression::Literal(
                    Literal::String(format!("${{{}", part)),
                    span,
                ));
            }
        }

        // Combine all parts with Plus
        if expression_parts.is_empty() {
            return Expression::Literal(Literal::String("".to_string()), span);
        }

        let mut final_expr = expression_parts[0].clone();
        for expr in expression_parts.iter().skip(1) {
            final_expr = Expression::Binary {
                left: Box::new(final_expr),
                operator: TokenKind::Plus,
                right: Box::new(expr.clone()),
                span,
            };
        }

        final_expr
    }

    fn parse_primary_expression(&mut self) -> Option<Expression> {
        let token = self.peek()?;
        // println!("Parsing primary expression: {:?}", token.kind);
        match &token.kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                let span = token.span;
                self.advance();
                Some(Expression::Identifier(name, span))
            }

            TokenKind::String(s) => {
                let span = token.span;
                let s = s.clone();
                self.advance();

                if s.contains("${") {
                    Some(self.parse_interpolated_string(&s, span))
                } else {
                    Some(Expression::Literal(Literal::String(s), span))
                }
            }

            TokenKind::Number(n) => {
                let value = Literal::Number(n.clone());
                let span = token.span;
                self.advance();
                Some(Expression::Literal(value, span))
            }

            TokenKind::Boolean(b) => {
                let value = Literal::Boolean(*b);
                let span = token.span;
                self.advance();
                Some(Expression::Literal(value, span))
            }

            TokenKind::LeftParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression();
                if self.peek()?.kind != TokenKind::RightParen {
                    return None;
                }
                self.advance(); // consume ')'
                expr
            }

            TokenKind::EOF => None,

            TokenKind::Plus | TokenKind::Minus | TokenKind::Not => {
                let operator = token.kind.clone();
                let span = token.span;
                self.advance();

                if let Some(operand) = self.parse_binary_expression(8) {
                    Some(Expression::Unary {
                        operator,
                        operand: Box::new(operand),
                        span,
                    })
                } else {
                    None
                }
            }
            TokenKind::LeftBracket => self.parse_array_literal(),
            TokenKind::LeftBrace => self.parse_object_literal(),
            _ => None,
        }
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        let start_span = self.peek()?.span;
        self.advance(); // [

        let mut elements = Vec::new();
        while !self.is_at_end() && self.peek()?.kind != TokenKind::RightBracket {
            // Skip newlines and indentation
            while self.peek().map_or(false, |t| {
                matches!(
                    t.kind,
                    TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                )
            }) {
                self.advance();
            }
            if self
                .peek()
                .map_or(false, |t| t.kind == TokenKind::RightBracket)
            {
                break;
            }

            if let Some(expr) = self.parse_expression() {
                elements.push(expr);
            } else {
                break;
            }

            // Skip newlines, indentation, and comma
            while self.peek().map_or(false, |t| {
                matches!(
                    t.kind,
                    TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                )
            }) {
                self.advance();
            }
            if self.peek().map_or(false, |t| t.kind == TokenKind::Comma) {
                self.advance();
            }
            while self.peek().map_or(false, |t| {
                matches!(
                    t.kind,
                    TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                )
            }) {
                self.advance();
            }
        }

        if self
            .peek()
            .map_or(true, |t| t.kind != TokenKind::RightBracket)
        {
            return None;
        }
        self.advance(); // ]

        Some(Expression::Array(elements, start_span))
    }

    fn parse_object_literal(&mut self) -> Option<Expression> {
        // println!("Parsing object literal");
        let start_span = self.peek()?.span;
        self.advance(); // {

        let mut properties = Vec::new();
        while !self.is_at_end() && self.peek()?.kind != TokenKind::RightBrace {
            // Skip newlines and indentation
            while self.peek().map_or(false, |t| {
                matches!(
                    t.kind,
                    TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                )
            }) {
                self.advance();
            }
            if self
                .peek()
                .map_or(false, |t| t.kind == TokenKind::RightBrace)
            {
                break;
            }

            // Support both "key": val and key: val
            let key = match self.peek()?.kind.clone() {
                TokenKind::String(s) => {
                    self.advance();
                    s
                }
                TokenKind::Identifier(id) => {
                    self.advance();
                    id
                }
                _ => break,
            };

            // Skip newlines and indentation before colon
            while self.peek().map_or(false, |t| {
                matches!(
                    t.kind,
                    TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                )
            }) {
                self.advance();
            }
            if self.peek().map_or(false, |t| t.kind != TokenKind::Colon) {
                break;
            }
            self.advance(); // :

            // Skip newlines and indentation after colon
            while self.peek().map_or(false, |t| {
                matches!(
                    t.kind,
                    TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                )
            }) {
                self.advance();
            }

            if let Some(expr) = self.parse_expression() {
                properties.push((key, expr));
            } else {
                break;
            }

            // Skip newlines, indentation, and comma
            while self.peek().map_or(false, |t| {
                matches!(
                    t.kind,
                    TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                )
            }) {
                self.advance();
            }
            if self.peek().map_or(false, |t| t.kind == TokenKind::Comma) {
                self.advance();
            }
            while self.peek().map_or(false, |t| {
                matches!(
                    t.kind,
                    TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
                )
            }) {
                self.advance();
            }
        }

        if self
            .peek()
            .map_or(true, |t| t.kind != TokenKind::RightBrace)
        {
            return None;
        }
        self.advance(); // }

        Some(Expression::Object(properties, start_span))
    }

    fn get_precedence(&self, token: &TokenKind) -> u8 {
        match token {
            TokenKind::Dot | TokenKind::LeftBracket | TokenKind::LeftParen => 8,
            TokenKind::Multiply | TokenKind::Divide | TokenKind::Modulo => 7,
            TokenKind::Plus | TokenKind::Minus => 6,
            TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual => 5,
            TokenKind::Equal | TokenKind::NotEqual => 4,
            TokenKind::And => 3,
            TokenKind::Or => 2,
            TokenKind::Range => 1,
            TokenKind::Assign
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::MultiplyEqual
            | TokenKind::DivideEqual => 0,
            TokenKind::Newline
            | TokenKind::RightParen
            | TokenKind::RightBracket
            | TokenKind::EOF
            | TokenKind::LeftBrace
            | TokenKind::RightBrace
            | TokenKind::Comma
            | TokenKind::Semicolon
            | TokenKind::Colon
            | TokenKind::Indent
            | TokenKind::Dedent => 255, // Give highest precedence to stop parsing
            _ => 0,
        }
    }

    // Helper methods
    fn peek(&self) -> Option<&Token> {
        self.tokens.front()
    }

    fn advance(&mut self) -> Option<&Token> {
        self.tokens.pop_front();
        self.peek()
    }

    #[allow(dead_code)]
    fn expect(&mut self, kind: TokenKind) -> Result<&Token, ParserError> {
        if let Some(token) = self.peek() {
            if std::mem::discriminant(&token.kind) == std::mem::discriminant(&kind) {
                Ok(token)
            } else {
                Err(ParserError::UnexpectedToken {
                    expected: kind,
                    found: token.kind.clone(),
                    span: token.span,
                })
            }
        } else {
            Err(ParserError::UnexpectedEOF {
                expected: kind,
                position: Position {
                    line: 0,
                    column: 0,
                    offset: 0,
                },
            })
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().map_or(true, |t| t.kind == TokenKind::EOF)
    }
    fn parse_local_import_statement(&mut self) -> Option<Statement> {
        let span = self.peek()?.span;
        self.advance(); // consume 'import'

        let mut path = String::new();
        while !self.is_at_end() {
            if let TokenKind::Identifier(name) = &self.peek()?.kind {
                path.push_str(name);
                self.advance();
            } else {
                break;
            }

            if self.peek().map_or(false, |t| t.kind == TokenKind::Dot) {
                path.push('.');
                self.advance();
            } else {
                break;
            }
        }

        let mut symbols = Vec::new();
        if self
            .peek()
            .map_or(false, |t| t.kind == TokenKind::LeftBrace)
        {
            self.advance(); // consume '{'
            while !self.is_at_end() && self.peek()?.kind != TokenKind::RightBrace {
                if let TokenKind::Identifier(sym) = &self.peek()?.kind {
                    symbols.push(sym.clone());
                    self.advance();
                    if self.peek().map_or(false, |t| t.kind == TokenKind::Comma) {
                        self.advance();
                    }
                } else {
                    break;
                }
            }
            if self
                .peek()
                .map_or(false, |t| t.kind == TokenKind::RightBrace)
            {
                self.advance(); // consume '}'
            }
        }

        Some(Statement::Import {
            path,
            symbols,
            span,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zenith_lexer::Lexer;

    #[test]
    fn test_parse_simple_expression() {
        let input = "1 + 2 * 3";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.declarations.len(), 0);
    }

    #[test]
    fn test_parse_variable_declaration() {
        let input = "let x = 42";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.declarations.len(), 0);
    }

    #[test]
    fn test_parse_function_declaration() {
        let input = "func add(a, b) -> a + b";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.declarations.len(), 1);
    }
}
