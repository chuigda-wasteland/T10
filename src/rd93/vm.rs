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
        let mut top_scope = Scope::new(func_info.stack_size, 0);
        for (idx, arg) in args.iter().enumerate() {
          *top_scope.values.get_unchecked_mut(idx) = *arg;
        }
        self.scopes.push(top_scope);

        loop {
            let mut insc: &Insc = &self.program.inscs[self.insc_ptr];
            let mut scope: &mut Scope = self.scopes.last_mut().unwrap_unchecked();

            match insc {
                Insc::IntAdd { lhs_value, rhs_value, dest_value } => {
                    let lhs = *scope.values.get_unchecked(*lhs_value);
                    let rhs = *scope.values.get_unchecked(*rhs_value);
                    let sum = lhs.value_typed_data.inner.int + rhs.value_typed_data.inner.int;
                    *scope.values.get_unchecked_mut(*dest_value) = Value::from(sum);
                },
                Insc::Return { ret_value } => {
                    scope.ret_value_loc = *ret_value;
                    self.insc_ptr = scope.ret_addr;

                    let value = *scope.values.get_unchecked(scope.ret_value_loc);
                    if self.scopes.len() == 1 {
                        return value;
                    } else {
                        insc = &self.program.inscs[self.insc_ptr];
                        if let Insc::FuncCall { func_id: _, arg_values: _, ret_value_dest } = insc {
                            self.scopes.pop().unwrap_unchecked();
                            scope = self.scopes.last_mut().unwrap_unchecked();
                            *scope.values.get_unchecked_mut(*ret_value_dest) = value;
                        } else {
                            core::hint::unreachable_unchecked();
                        }
                    }
                },
                Insc::UnreachableInsc => panic!("this is an internal, unreachable insc"),
                _ => todo!("unimplemented insc")
            }
            self.insc_ptr += 1;
        }
    }
}
