//! `vm` 实现了虚拟机的总体布局

use crate::data::Value;
use crate::rd93::insc::{Insc, CompiledFuncInfo, CompiledProgram};
use crate::rd93::scope::Scope;

pub struct RD93<'a> {
    pub program: &'a CompiledProgram,
    pub scopes: Vec<Scope>,
    pub insc_ptr: usize
}

impl<'a> RD93<'a> {
    pub fn new(program: &'a CompiledProgram) -> Self {
        Self {
            program,
            scopes: vec![],
            insc_ptr: 0
        }
    }

    pub unsafe fn run_func(&mut self, func_id: usize, args: &[Value]) -> Value {
        let func_info: CompiledFuncInfo = self.program.funcs[func_id];

        debug_assert_eq!(args.len(), func_info.arg_count);
        self.insc_ptr = func_info.start_addr;
        let top_scope = Scope::new(func_info.stack_size, 0);

        self.scopes.push(top_scope);

        loop {
            let insc: &Insc = &self.program.inscs[self.insc_ptr];
            let _scope: &mut Scope = self.scopes.last_mut().unwrap_unchecked();

            match insc {
                Insc::UnreachableInsc => panic!("this is an internal, unreachable insc"),
                Insc::Halt => break,
                _ => todo!("unimplemented insc")
            }
        }

        unimplemented!()
    }
}
