//! Zenith Core Primitives
//! Enhanced value types for Zenith standard library

use std::collections::HashMap;
use std::fmt;

/// Enhanced Value enum for Zenith standard library
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Function(String, Vec<String>),
    NativeFunction(String, usize),
    Class(String),
    Instance(String, HashMap<String, Value>),
    State(usize),
}

/// AST Statement nodes for Zenith
#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration(String, Expression),
    FunctionDeclaration(String, Vec<String>),
    Return(Expression),
    If(Expression, Vec<Statement>, Vec<Statement>),
    While(Expression, Vec<Statement>),
    For(String, Expression, Expression, Vec<Statement>),
    Break,
    Continue,
    Block(Vec<Statement>),
}

/// AST Expression nodes for Zenith
#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Value),
    Variable(String),
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        member: Box<Expression>,
    },
    IndexAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Not,
    BitwiseNot,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
            Value::Function(name, _) => write!(f, "<function {}>", name),
            Value::NativeFunction(name, _) => write!(f, "<native function {}>", name),
            Value::Class(name) => write!(f, "<class {}>", name),
            Value::Instance(name, _) => write!(f, "<instance of {}>", name),
            Value::State(id) => write!(f, "<state #{}>", id),
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op_str = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanOrEqual => ">=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op_str = match self {
            UnaryOperator::Negate => "-",
            UnaryOperator::Not => "!",
            UnaryOperator::BitwiseNot => "~",
        };
        write!(f, "{}", op_str)
    }
}

/// Core primitive operations for Zenith
pub struct Primitives;

impl Primitives {
    /// Type checking and conversion utilities
    pub fn type_of(value: &Value) -> &'static str {
        match value {
            Value::Null => "null",
            Value::Boolean(_) => "boolean",
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Function(_, _) => "function",
            Value::NativeFunction(_, _) => "native_function",
            Value::Class(_) => "class",
            Value::Instance(_, _) => "instance",
            Value::State(_) => "state",
        }
    }

    /// Check if a value is truthy
    pub fn is_truthy(value: &Value) -> bool {
        !matches!(value, Value::Null | Value::Boolean(false))
    }

    /// Check if a value is falsy
    pub fn is_falsy(value: &Value) -> bool {
        matches!(value, Value::Null | Value::Boolean(false))
    }

    /// Convert value to boolean
    pub fn to_boolean(value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
            _ => false,
        }
    }

    /// Convert value to integer with type checking
    pub fn to_integer(value: &Value) -> Option<i64> {
        match value {
            Value::Integer(i) => Some(*i),
            Value::Float(f) => Some(*f as i64),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Convert value to float with type checking
    pub fn to_float(value: &Value) -> Option<f64> {
        match value {
            Value::Float(f) => Some(*f),
            Value::Integer(i) => Some(*i as f64),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Convert value to string with type checking
    pub fn to_string(value: &Value) -> Option<String> {
        match value {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Deep clone a value (handles nested structures)
    pub fn deep_clone(value: &Value) -> Value {
        match value {
            Value::Array(arr) => {
                let cloned_items: Vec<Value> =
                    arr.iter().map(|v| Primitives::deep_clone(v)).collect();
                Value::Array(cloned_items)
            }
            Value::Object(obj) => {
                let cloned_items: HashMap<String, Value> = obj
                    .iter()
                    .map(|(k, v)| (k.clone(), Primitives::deep_clone(v)))
                    .collect();
                Value::Object(cloned_items)
            }
            _ => value.clone(),
        }
    }

    /// Compare two values for equality
    pub fn equals(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Integer(ai), Value::Integer(bi)) => ai == bi,
            (Value::Float(af), Value::Float(bf)) => (af - bf).abs() < f64::EPSILON,
            (Value::String(as_str), Value::String(bs)) => as_str == bs,
            (Value::Array(aa), Value::Array(ba)) => {
                aa.len() == ba.len()
                    && aa
                        .iter()
                        .zip(ba.iter())
                        .all(|(a, b)| Primitives::equals(a, b))
            }
            (Value::Object(ao), Value::Object(bo)) => {
                ao.len() == bo.len()
                    && ao
                        .iter()
                        .all(|(k, v)| bo.get(k).map_or(false, |bv| Primitives::equals(v, bv)))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_operations() {
        let int_val = Value::Integer(42);
        let float_val = Value::Float(3.14);
        let string_val = Value::String("hello".to_string());
        let bool_val = Value::Boolean(true);
        let null_val = Value::Null;

        // Test type checking
        assert_eq!(Primitives::type_of(&int_val), "integer");
        assert_eq!(Primitives::type_of(&float_val), "float");
        assert_eq!(Primitives::type_of(&string_val), "string");
        assert_eq!(Primitives::type_of(&bool_val), "boolean");
        assert_eq!(Primitives::type_of(&null_val), "null");

        // Test truthy/falsy checks
        assert!(Primitives::is_truthy(&bool_val));
        assert!(Primitives::is_truthy(&int_val));
        assert!(!Primitives::is_falsy(&bool_val));
        assert!(!Primitives::is_falsy(&int_val));
        assert!(Primitives::is_falsy(&null_val));
        assert!(Primitives::is_falsy(&string_val));

        // Test conversions
        assert_eq!(Primitives::to_boolean(&bool_val), true);
        assert_eq!(Primitives::to_boolean(&int_val), true);
        assert_eq!(Primitives::to_boolean(&float_val), true);
        assert_eq!(Primitives::to_boolean(&string_val), true);
        assert_eq!(Primitives::to_boolean(&null_val), false);

        assert_eq!(Primitives::to_integer(&int_val), Some(42));
        assert_eq!(Primitives::to_integer(&float_val), Some(3));
        assert_eq!(Primitives::to_float(&int_val), Some(42.0));
        assert_eq!(Primitives::to_float(&float_val), Some(3.14));

        // Test equality
        assert!(Primitives::equals(&int_val, &int_val));
        assert!(Primitives::equals(&string_val, &string_val));
        assert!(!Primitives::equals(&int_val, &float_val));

        // Test deep clone
        let arr = Value::Array(vec![
            Value::Integer(1),
            Value::String("nested".to_string()),
            Value::Array(vec![Value::Integer(2), Value::Integer(3)]),
        ]);
        let cloned = Primitives::deep_clone(&arr);
        if let Value::Array(cloned_arr) = cloned {
            assert_eq!(cloned_arr.len(), 3);
            if let Value::String(nested) = &cloned_arr[1] {
                assert_eq!(nested, "nested");
            }
            if let Value::Array(nested_arr) = &cloned_arr[2] {
                if let Value::Integer(i2) = &nested_arr[0] {
                    assert_eq!(*i2, 1);
                }
                if let Value::Integer(i3) = &nested_arr[1] {
                    assert_eq!(*i3, 2);
                }
            }
        }
    }
}
