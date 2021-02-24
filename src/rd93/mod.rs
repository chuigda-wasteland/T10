//! `rd93` 中实现了一个最小化、可运行的 VM，主要用作正式开发之前的
//! Benchmarking

pub mod insc;
pub mod scope;

use std::any::TypeId;

use crate::data::Value;
use crate::rd93::insc::{CompiledProgram, CompiledFuncInfo, Insc};
use crate::rd93::scope::Scope;

pub struct RD93 ();

impl RD93 {
    pub unsafe fn run_func(program: &CompiledProgram, func_id: usize, args: &[Value]) -> Value {
        let func_info: CompiledFuncInfo = program.funcs[func_id];
        debug_assert_eq!(args.len(), func_info.arg_count);

        let mut insc_ptr = func_info.start_addr;
        let mut scopes = vec![];
        let mut top_scope = Scope::new(func_info.stack_size, 0);
        for (idx, arg) in args.iter().enumerate() {
            top_scope.set_value(idx, *arg);
        }
        scopes.push(top_scope);

        let mut scope: &mut Scope = scopes.last_mut().unwrap_unchecked();
        loop {
            let mut insc: &Insc = program.inscs.get_unchecked(insc_ptr);

            match insc {
                Insc::MakeIntConst { c, dest_value } => {
                    scope.set_value(*dest_value, Value::from(*c));
                },
                Insc::IntAdd { lhs_value, rhs_value, dest_value } => {
                    debug_assert_eq!(scope.get_value(*lhs_value).type_id(), TypeId::of::<i64>());
                    debug_assert_eq!(scope.get_value(*rhs_value).type_id(), TypeId::of::<i64>());
                    let lhs = scope.get_value(*lhs_value).value_typed_data.inner.int;
                    let rhs = scope.get_value(*rhs_value).value_typed_data.inner.int;
                    let sum = lhs.wrapping_add(rhs);
                    scope.set_value(*dest_value, Value::from(sum));
                },
                Insc::IntSub { lhs_value, rhs_value, dest_value } => {
                    debug_assert_eq!(scope.get_value(*lhs_value).type_id(), TypeId::of::<i64>());
                    debug_assert_eq!(scope.get_value(*rhs_value).type_id(), TypeId::of::<i64>());
                    let lhs = scope.get_value(*lhs_value).value_typed_data.inner.int;
                    let rhs = scope.get_value(*rhs_value).value_typed_data.inner.int;
                    let diff = lhs.wrapping_sub(rhs);
                    scope.set_value(*dest_value, Value::from(diff));
                },
                Insc::IntEq { lhs_value, rhs_value, dest_value } => {
                    debug_assert_eq!(scope.get_value(*lhs_value).type_id(), TypeId::of::<i64>());
                    debug_assert_eq!(scope.get_value(*rhs_value).type_id(), TypeId::of::<i64>());
                    let lhs = scope.get_value(*lhs_value).value_typed_data.inner.int;
                    let rhs = scope.get_value(*rhs_value).value_typed_data.inner.int;
                    scope.set_value(*dest_value, Value::from(lhs == rhs));
                },
                Insc::JumpIfTrue { cond_value, jump_dest } => {
                    debug_assert_eq!(scope.get_value(*cond_value).type_id(), TypeId::of::<bool>());
                    let cond = scope.get_value(*cond_value).value_typed_data.inner.boolean;
                    if cond {
                        insc_ptr = *jump_dest;
                        continue;
                    }
                },
                Insc::FuncCall { func_id, arg_values, ret_value_dest: _ } => {
                    let func_info: CompiledFuncInfo = *program.funcs.get_unchecked(*func_id);
                    debug_assert_eq!(func_info.arg_count, arg_values.len());

                    let mut new_scope = Scope::new(func_info.stack_size, insc_ptr);
                    for (idx, value) in arg_values.iter().enumerate() {
                        new_scope.set_value(idx, scope.get_value(*value));
                    }
                    scopes.push(new_scope);
                    scope = scopes.last_mut().unwrap_unchecked();
                    insc_ptr = func_info.start_addr;
                    continue;
                },
                Insc::Return { ret_value } => {
                    scope.ret_value_loc = *ret_value;
                    insc_ptr = scope.ret_addr;

                    let value = *scope.values.get_unchecked(scope.ret_value_loc);
                    if scopes.len() == 1 {
                        return value;
                    } else {
                        insc = &program.inscs[insc_ptr];
                        if let Insc::FuncCall {
                            func_id: _,
                            arg_values: _,
                            ret_value_dest
                        } = insc {
                            scopes.pop().unwrap_unchecked();
                            scope = scopes.last_mut().unwrap_unchecked();
                            scope.set_value(*ret_value_dest, value);
                        } else {
                            core::hint::unreachable_unchecked();
                        }
                    }
                },
                Insc::UnreachableInsc => panic!("this is an internal, unreachable insc"),
                _ => todo!("unimplemented insc")
            }

            insc_ptr += 1;
        }
    }
}
