//! Zenith Middle Intermediate Representation (MIR)
//!
//! MIR is a lower-level, simpler representation of Zenith code,
//! often represented as a Control Flow Graph (CFG) of basic blocks.

pub mod lower;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum MirType {
    Void,
    Int,
    Float,
    Bool,
    Pointer(Box<MirType>),
    Function {
        params: Vec<MirType>,
        return_type: Box<MirType>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    ConstantInt(i64),
    ConstantFloat(f64),
    ConstantBool(bool),
    Variable(String), // SSA variable
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Assign {
        dest: String,
        src: Operand,
    },
    Binary {
        dest: String,
        op: String,
        left: Operand,
        right: Operand,
    },
    Unary {
        dest: String,
        op: String,
        operand: Operand,
    },
    Call {
        dest: Option<String>,
        callee: String,
        args: Vec<Operand>,
    },
    Load {
        dest: String,
        ptr: String,
    },
    Store {
        ptr: String,
        value: Operand,
    },
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Return(Option<Operand>),
    Branch {
        condition: Operand,
        true_block: String,
        false_block: String,
    },
    Jump(String),
}

#[derive(Debug, Clone)]
pub struct MirFunction {
    pub name: String,
    pub params: Vec<(String, MirType)>,
    pub return_type: MirType,
    pub blocks: HashMap<String, BasicBlock>,
    pub start_block: String,
}

#[derive(Debug, Clone)]
pub struct MirModule {
    pub name: String,
    pub functions: HashMap<String, MirFunction>,
}
