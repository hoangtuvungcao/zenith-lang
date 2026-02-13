//! Zenith Language Lexer
//!
//! This module provides tokenization capabilities for the Zenith programming language.
//! It handles lexical analysis, breaking source code into tokens for parsing.

use std::fmt;
use std::str::CharIndices;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character '{0}' at line {1}, column {2}")]
    UnexpectedCharacter(char, usize, usize),

    #[error("Unterminated string starting at line {1}, column {2}")]
    UnterminatedString(String, usize, usize),

    #[error("Invalid number format '{0}' at line {1}, column {2}")]
    InvalidNumber(String, usize, usize),

    #[error("Unknown keyword '{0}' at line {1}, column {2}")]
    UnknownKeyword(String, usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Identifier(String),
    String(String),
    Number(String),
    Boolean(bool),

    // Keywords
    Func,
    Let,
    Var,
    If,
    Else,
    For,
    While,
    Return,
    Struct,
    Class,
    Enum,
    Trait,
    Impl,
    Async,
    Await,
    Match,
    Import,
    Export,
    In,
    As,
    Break,
    Continue,

    // Operators
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    Assign,        // =
    PlusEqual,     // +=
    MinusEqual,    // -=
    MultiplyEqual, // *=
    DivideEqual,   // /=

    Equal,        // ==
    NotEqual,     // !=
    LessThan,     // <
    GreaterThan,  // >
    LessEqual,    // <=
    GreaterEqual, // >=

    And, // &&
    Or,  // ||
    Not, // !

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    Comma,     // ,
    Dot,       // .
    Colon,     // :
    Semicolon, // ;
    Arrow,     // ->
    Range,     // ..

    // Special
    Newline,
    Indent,
    Dedent,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub text: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.start)
    }
}

pub struct Lexer<'a> {
    _input: &'a str,
    chars: CharIndices<'a>,
    current_char: Option<char>,
    current_pos: Position,
    tokens: Vec<Token>,
    indent_stack: Vec<usize>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.char_indices();
        let current_char = chars.next().map(|(_, c)| c);

        Self {
            _input: input,
            chars,
            current_char,
            current_pos: Position {
                line: 1,
                column: 1,
                offset: 0,
            },
            tokens: Vec::new(),
            indent_stack: vec![0],
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' => self.skip_whitespace()?,
                '\n' => self.handle_newline(),
                '/' => self.handle_slash(),
                '"' => self.handle_string()?,
                '\'' => self.handle_char()?,
                '0'..='9' => self.handle_number()?,
                'a'..='z' | 'A'..='Z' | '_' => self.handle_identifier(),
                '+' => self.handle_plus(),
                '-' => self.handle_minus(),
                '*' => self.handle_multiply(),
                '%' => self.handle_modulo(),
                '=' => self.handle_equal(),
                '!' => self.handle_bang(),
                '<' => self.handle_less(),
                '>' => self.handle_greater(),
                '&' => self.handle_and(),
                '|' => self.handle_or(),
                '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' | ';' => {
                    self.handle_single_char_delimiter();
                }
                '.' => self.handle_dot(),
                _ => {
                    return Err(LexerError::UnexpectedCharacter(
                        c,
                        self.current_pos.line,
                        self.current_pos.column,
                    ));
                }
            }
        }

        // Add EOF token
        self.tokens.push(Token {
            kind: TokenKind::EOF,
            span: Span {
                start: self.current_pos,
                end: self.current_pos,
            },
            text: String::new(),
        });

        Ok(self.tokens)
    }

    fn advance(&mut self) {
        if let Some((offset, c)) = self.chars.next() {
            self.current_char = Some(c);
            self.current_pos.offset = offset;
            self.current_pos.column += 1;
        } else {
            self.current_char = None;
        }
    }

    fn skip_whitespace(&mut self) -> Result<(), LexerError> {
        while let Some(c) = self.current_char {
            if c == ' ' || c == '\t' {
                self.advance();
            } else {
                break;
            }
        }
        Ok(())
    }

    fn handle_newline(&mut self) {
        let start_pos = self.current_pos;
        self.advance();
        self.current_pos.line += 1;
        self.current_pos.column = 1;

        self.tokens.push(Token {
            kind: TokenKind::Newline,
            span: Span {
                start: start_pos,
                end: self.current_pos,
            },
            text: "\n".to_string(),
        });

        // Handle indentation for next line
        self.handle_indentation();
    }

    fn handle_indentation(&mut self) {
        let mut indent_level = 0;
        let start_pos = self.current_pos;

        while let Some(c) = self.current_char {
            if c == ' ' {
                indent_level += 1;
                self.advance();
            } else if c == '\t' {
                indent_level += 4; // Assume tab = 4 spaces
                self.advance();
            } else {
                break;
            }
        }

        let current_indent = *self.indent_stack.last().unwrap();

        if indent_level > current_indent {
            self.indent_stack.push(indent_level);
            self.tokens.push(Token {
                kind: TokenKind::Indent,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: " ".repeat(indent_level - current_indent),
            });
        } else if indent_level < current_indent {
            while *self.indent_stack.last().unwrap() > indent_level {
                self.indent_stack.pop();
                self.tokens.push(Token {
                    kind: TokenKind::Dedent,
                    span: Span {
                        start: start_pos,
                        end: self.current_pos,
                    },
                    text: String::new(),
                });
            }
        }
    }

    fn handle_string(&mut self) -> Result<(), LexerError> {
        let start_pos = self.current_pos;
        self.advance(); // Skip opening quote

        let mut string_content = String::new();

        while let Some(c) = self.current_char {
            if c == '"' {
                let end_pos = self.current_pos;
                self.advance(); // Skip closing quote

                self.tokens.push(Token {
                    kind: TokenKind::String(string_content.clone()),
                    span: Span {
                        start: start_pos,
                        end: end_pos,
                    },
                    text: format!("\"{}\"", string_content),
                });
                return Ok(());
            } else if c == '\\' {
                self.advance();
                if let Some(escaped_char) = self.current_char {
                    string_content.push(match escaped_char {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        _ => escaped_char,
                    });
                    self.advance();
                }
            } else {
                string_content.push(c);
                self.advance();
            }
        }

        Err(LexerError::UnterminatedString(
            string_content,
            start_pos.line,
            start_pos.column,
        ))
    }

    fn handle_char(&mut self) -> Result<(), LexerError> {
        let start_pos = self.current_pos;
        self.advance(); // Skip opening quote

        let char_content = if let Some(c) = self.current_char {
            let content = if c == '\\' {
                self.advance();
                if let Some(escaped_char) = self.current_char {
                    match escaped_char {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '\'' => '\'',
                        _ => escaped_char,
                    }
                } else {
                    return Err(LexerError::UnterminatedString(
                        "".to_string(),
                        start_pos.line,
                        start_pos.column,
                    ));
                }
            } else {
                c
            };
            self.advance();
            content
        } else {
            return Err(LexerError::UnterminatedString(
                "".to_string(),
                start_pos.line,
                start_pos.column,
            ));
        };

        if self.current_char != Some('\'') {
            return Err(LexerError::UnterminatedString(
                char_content.to_string(),
                start_pos.line,
                start_pos.column,
            ));
        }

        let end_pos = self.current_pos;
        self.advance(); // Skip closing quote

        self.tokens.push(Token {
            kind: TokenKind::String(char_content.to_string()),
            span: Span {
                start: start_pos,
                end: end_pos,
            },
            text: format!("'{}'", char_content),
        });

        Ok(())
    }

    fn handle_number(&mut self) -> Result<(), LexerError> {
        let start_pos = self.current_pos;
        let mut number_text = String::new();

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() || c == '_' {
                number_text.push(c);
                self.advance();
            } else if c == '.' {
                // Check if it's a floating point or a range
                let mut peek_chars = self.chars.clone();
                if let Some((_, next_c)) = peek_chars.next() {
                    if next_c == '.' {
                        // Range operator, stop here
                        break;
                    }
                    if next_c.is_ascii_digit() {
                        number_text.push(c);
                        self.advance();
                        continue;
                    }
                }
                // Just a dot (member access) or trailing dot
                break;
            } else {
                break;
            }
        }

        self.tokens.push(Token {
            kind: TokenKind::Number(number_text.clone()),
            span: Span {
                start: start_pos,
                end: self.current_pos,
            },
            text: number_text,
        });

        Ok(())
    }

    fn handle_identifier(&mut self) {
        let start_pos = self.current_pos;
        let mut identifier = String::new();

        while let Some(c) = self.current_char {
            if c.is_ascii_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let kind = match identifier.as_str() {
            "func" => TokenKind::Func,
            "let" => TokenKind::Let,
            "var" => TokenKind::Var,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "struct" => TokenKind::Struct,
            "class" => TokenKind::Class,
            "enum" => TokenKind::Enum,
            "trait" => TokenKind::Trait,
            "impl" => TokenKind::Impl,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "match" => TokenKind::Match,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "in" => TokenKind::In,
            "as" => TokenKind::As,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            _ => TokenKind::Identifier(identifier.clone()),
        };

        self.tokens.push(Token {
            kind,
            span: Span {
                start: start_pos,
                end: self.current_pos,
            },
            text: identifier,
        });
    }

    fn handle_plus(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('=') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::PlusEqual,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "+=".to_string(),
            });
        } else {
            self.tokens.push(Token {
                kind: TokenKind::Plus,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "+".to_string(),
            });
        }
    }

    fn handle_minus(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('=') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::MinusEqual,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "-=".to_string(),
            });
        } else if self.current_char == Some('>') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::Arrow,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "->".to_string(),
            });
        } else {
            self.tokens.push(Token {
                kind: TokenKind::Minus,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "-".to_string(),
            });
        }
    }

    fn handle_multiply(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('=') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::MultiplyEqual,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "*=".to_string(),
            });
        } else {
            self.tokens.push(Token {
                kind: TokenKind::Multiply,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "*".to_string(),
            });
        }
    }

    fn handle_divide(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('=') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::DivideEqual,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "/=".to_string(),
            });
        } else {
            self.tokens.push(Token {
                kind: TokenKind::Divide,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "/".to_string(),
            });
        }
    }

    fn handle_modulo(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        self.tokens.push(Token {
            kind: TokenKind::Modulo,
            span: Span {
                start: start_pos,
                end: self.current_pos,
            },
            text: "%".to_string(),
        });
    }

    fn handle_equal(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('=') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::Equal,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "==".to_string(),
            });
        } else {
            self.tokens.push(Token {
                kind: TokenKind::Assign,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "=".to_string(),
            });
        }
    }

    fn handle_bang(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('=') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::NotEqual,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "!=".to_string(),
            });
        } else {
            self.tokens.push(Token {
                kind: TokenKind::Not,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "!".to_string(),
            });
        }
    }

    fn handle_less(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('=') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::LessEqual,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "<=".to_string(),
            });
        } else {
            self.tokens.push(Token {
                kind: TokenKind::LessThan,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "<".to_string(),
            });
        }
    }

    fn handle_greater(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('=') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::GreaterEqual,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: ">=".to_string(),
            });
        } else {
            self.tokens.push(Token {
                kind: TokenKind::GreaterThan,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: ">".to_string(),
            });
        }
    }

    fn handle_and(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('&') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::And,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "&&".to_string(),
            });
        }
    }

    fn handle_or(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('|') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::Or,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "||".to_string(),
            });
        }
    }

    fn handle_slash(&mut self) {
        let _start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('/') {
            // Line comment
            self.advance();
            while let Some(c) = self.current_char {
                if c == '\n' {
                    break;
                }
                self.advance();
            }
        } else if self.current_char == Some('*') {
            // Block comment
            self.advance();
            let mut depth = 1;
            while depth > 0 && self.current_char.is_some() {
                if self.current_char == Some('/') {
                    self.advance();
                    if self.current_char == Some('*') {
                        self.advance();
                        depth += 1;
                    }
                } else if self.current_char == Some('*') {
                    self.advance();
                    if self.current_char == Some('/') {
                        self.advance();
                        depth -= 1;
                    }
                } else {
                    self.advance();
                }
            }
        } else {
            self.handle_divide();
        }
    }

    fn handle_dot(&mut self) {
        let start_pos = self.current_pos;
        self.advance();

        if self.current_char == Some('.') {
            self.advance();
            self.tokens.push(Token {
                kind: TokenKind::Range,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: "..".to_string(),
            });
        } else {
            self.tokens.push(Token {
                kind: TokenKind::Dot,
                span: Span {
                    start: start_pos,
                    end: self.current_pos,
                },
                text: ".".to_string(),
            });
        }
    }

    fn handle_single_char_delimiter(&mut self) {
        let start_pos = self.current_pos;
        let c = self.current_char.unwrap();
        self.advance();

        let kind = match c {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            _ => unreachable!(),
        };

        self.tokens.push(Token {
            kind,
            span: Span {
                start: start_pos,
                end: self.current_pos,
            },
            text: c.to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "let x = 42";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Assign);
        assert_eq!(tokens[3].kind, TokenKind::Number("42".to_string()));
        assert_eq!(tokens[4].kind, TokenKind::EOF);
    }

    #[test]
    fn test_string_literals() {
        let input = "\"Hello, World!\"";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].kind,
            TokenKind::String("Hello, World!".to_string())
        );
    }

    #[test]
    fn test_function_declaration() {
        let input = "func add(a, b) -> a + b";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Func);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("add".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::LeftParen);
        assert_eq!(tokens[3].kind, TokenKind::Identifier("a".to_string()));
        assert_eq!(tokens[4].kind, TokenKind::Comma);
        assert_eq!(tokens[5].kind, TokenKind::Identifier("b".to_string()));
        assert_eq!(tokens[6].kind, TokenKind::RightParen);
        assert_eq!(tokens[7].kind, TokenKind::Arrow);
    }
}
