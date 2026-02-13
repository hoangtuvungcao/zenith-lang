//! Lowering pass from HIR to MIR

use crate::*;
use std::collections::HashMap;
use zenith_hir as hir;

pub struct Lowerer {
    _next_var_id: usize,
    next_block_id: usize,
}

impl Lowerer {
    pub fn new() -> Self {
        Self {
            _next_var_id: 0,
            next_block_id: 0,
        }
    }

    #[allow(dead_code)]
    fn new_var(&mut self) -> String {
        let id = format!("v{}", self._next_var_id);
        self._next_var_id += 1;
        id
    }

    fn new_block_id(&mut self) -> String {
        let id = format!("bb{}", self.next_block_id);
        self.next_block_id += 1;
        id
    }

    pub fn lower_module(&mut self, hir_module: &hir::Module) -> MirModule {
        let mut mir_functions = HashMap::new();

        for decl in &hir_module.declarations {
            if let hir::Declaration::Function(hir::Statement::FuncDeclaration {
                name,
                parameters,
                return_type,
                body,
            }) = decl
            {
                let mir_func = self.lower_function(name, parameters, return_type, body);
                mir_functions.insert(name.clone(), mir_func);
            }
        }

        MirModule {
            name: hir_module.name.clone(),
            functions: mir_functions,
        }
    }

    fn lower_function(
        &mut self,
        _name: &str,
        _params: &[(String, hir::Type)],
        _return_type: &hir::Type,
        _body: &[hir::Statement],
    ) -> MirFunction {
        // Basic placeholder for function lowering
        let start_block_id = self.new_block_id();
        let mut blocks = HashMap::new();

        blocks.insert(
            start_block_id.clone(),
            BasicBlock {
                id: start_block_id.clone(),
                instructions: Vec::new(),
                terminator: Terminator::Return(None),
            },
        );

        MirFunction {
            name: _name.to_string(),
            params: _params
                .iter()
                .map(|(n, t)| (n.clone(), self.lower_type(t)))
                .collect(),
            return_type: self.lower_type(_return_type),
            blocks,
            start_block: start_block_id,
        }
    }

    fn lower_type(&self, hir_type: &hir::Type) -> MirType {
        match hir_type {
            hir::Type::Void => MirType::Void,
            hir::Type::Int => MirType::Int,
            hir::Type::Float => MirType::Float,
            hir::Type::Bool => MirType::Bool,
            hir::Type::String => MirType::Pointer(Box::new(MirType::Int)), // Simplified
            hir::Type::Function {
                params,
                return_type,
            } => MirType::Function {
                params: params.iter().map(|p| self.lower_type(p)).collect(),
                return_type: Box::new(self.lower_type(return_type)),
            },
            hir::Type::Unknown => MirType::Void,
        }
    }
}
