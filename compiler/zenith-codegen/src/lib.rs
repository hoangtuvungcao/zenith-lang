//! Zenith LLVM Code Generator
//!
//! This module handles the translation from MIR to LLVM IR.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use thiserror::Error;
use zenith_mir as mir;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("LLVM Error: {0}")]
    LlvmError(String),
}

pub struct Codegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        Self {
            context,
            module,
            builder,
        }
    }

    pub fn gen_module(&self, mir_module: &mir::MirModule) -> Result<(), CodegenError> {
        for (_name, mir_func) in &mir_module.functions {
            self.gen_function(mir_func)?;
        }
        Ok(())
    }

    fn gen_function(
        &self,
        mir_func: &mir::MirFunction,
    ) -> Result<FunctionValue<'ctx>, CodegenError> {
        let fn_type = self.context.void_type().fn_type(&[], false); // Placeholder
        let function = self.module.add_function(&mir_func.name, fn_type, None);

        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        // TODO: Generate instructions from MIR blocks

        self.builder
            .build_return(None)
            .map_err(|e| CodegenError::LlvmError(format!("{:?}", e)))?;

        Ok(function)
    }
}
