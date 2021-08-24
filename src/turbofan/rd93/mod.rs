//! `rd93` 中实现了一个最小化、可运行的 VM，主要用作正式开发之前的 Benchmarking

pub mod insc;

use std::any::TypeId;
use std::mem::MaybeUninit;

use crate::data::Value;
use crate::turbofan::stack::Stack;

pub use insc::{CompiledFuncInfo, CompiledProgram, Insc};

pub struct RD93 ();

impl RD93 {
    pub unsafe fn run_func(
        program: &CompiledProgram,
        func_id: usize,
        args: &[Value],
        outputs: &mut [MaybeUninit<Value>]
    ) {
        #[cfg(not(debug_assertions))]
        let func_info: CompiledFuncInfo = *program.funcs.get_unchecked(func_id);
        #[cfg(debug_assertions)]
        let func_info: CompiledFuncInfo = program.funcs[func_id];

        let dummy_ret_locs = [];
        debug_assert_eq!(args.len() as u32, func_info.arg_count);
        debug_assert_eq!(outputs.len() as u32, func_info.ret_count);

        let mut insc_ptr = func_info.start_addr as usize;
        let mut stack = Stack::new();
        let mut cur_stack_slice = stack.ext_func_call_grow_stack(
            func_info.stack_size,
            args,
            &dummy_ret_locs
        );
        for (idx, arg) in args.iter().enumerate() {
            cur_stack_slice.set_value(idx as u32, *arg);
        }
        let mut ffi_args = Vec::with_capacity(8);
        let mut ffi_rets = Vec::with_capacity(3);

        loop {
            let insc: &Insc = program.inscs.get_unchecked(insc_ptr);
            match insc {
                Insc::MakeIntConst { c, dest_value } => {
                    cur_stack_slice.set_value(*dest_value, Value::from(*c));
                },
                Insc::IntAdd { lhs_value, rhs_value, dest_value } => {
                    let lhs = cur_stack_slice.get_value(*lhs_value);
                    let rhs = cur_stack_slice.get_value(*rhs_value);
                    debug_assert_eq!(lhs.type_id(), TypeId::of::<i64>());
                    debug_assert_eq!(rhs.type_id(), TypeId::of::<i64>());
                    let lhs = lhs.value_typed_data.inner.int;
                    let rhs = rhs.value_typed_data.inner.int;
                    let sum = lhs.wrapping_add(rhs);
                    cur_stack_slice.set_value(*dest_value, Value::from(sum));
                },
                Insc::IntSub { lhs_value, rhs_value, dest_value } => {
                    let lhs = cur_stack_slice.get_value(*lhs_value);
                    let rhs = cur_stack_slice.get_value(*rhs_value);
                    debug_assert_eq!(lhs.type_id(), TypeId::of::<i64>());
                    debug_assert_eq!(rhs.type_id(), TypeId::of::<i64>());
                    let lhs = lhs.value_typed_data.inner.int;
                    let rhs = rhs.value_typed_data.inner.int;
                    let sub = lhs.wrapping_sub(rhs);
                    cur_stack_slice.set_value(*dest_value, Value::from(sub));
                },
                Insc::IntEq { lhs_value, rhs_value, dest_value } => {
                    let lhs = cur_stack_slice.get_value(*lhs_value);
                    let rhs = cur_stack_slice.get_value(*rhs_value);
                    debug_assert_eq!(lhs.type_id(), TypeId::of::<i64>());
                    debug_assert_eq!(rhs.type_id(), TypeId::of::<i64>());
                    let lhs = lhs.value_typed_data.inner.int;
                    let rhs = rhs.value_typed_data.inner.int;
                    cur_stack_slice.set_value(*dest_value, Value::from(lhs == rhs));
                },
                Insc::IntGt { lhs_value, rhs_value, dest_value } => {
                    let lhs = cur_stack_slice.get_value(*lhs_value);
                    let rhs = cur_stack_slice.get_value(*rhs_value);
                    debug_assert_eq!(lhs.type_id(), TypeId::of::<i64>());
                    debug_assert_eq!(rhs.type_id(), TypeId::of::<i64>());
                    let lhs = lhs.value_typed_data.inner.int;
                    let rhs = rhs.value_typed_data.inner.int;
                    cur_stack_slice.set_value(*dest_value, Value::from(lhs > rhs));
                },
                Insc::Incr { value } => {
                    let v = cur_stack_slice.get_value(*value);
                    debug_assert_eq!(v.type_id(), TypeId::of::<i64>());
                    let i = v.value_typed_data.inner.int;
                    cur_stack_slice.set_value(*value, Value::from(i.wrapping_add(1)))
                },
                Insc::JumpIfTrue { cond_value, jump_dest } => {
                    let cv = cur_stack_slice.get_value(*cond_value);
                    debug_assert_eq!(cv.type_id(), TypeId::of::<bool>());
                    let cond = cv.value_typed_data.inner.boolean;
                    if cond {
                        insc_ptr = *jump_dest as usize;
                        continue;
                    }
                },
                Insc::Jump { jump_dest } => {
                    insc_ptr = *jump_dest as usize;
                    continue;
                },
                Insc::FuncCall { func_id, arg_values, ret_value_locs } => {
                    #[cfg(not(debug_assertions))]
                    let func_info = *program.funcs.get_unchecked(*func_id as usize);
                    #[cfg(debug_assertions)]
                    let func_info = program.funcs[*func_id as usize];
                    debug_assert_eq!(func_info.arg_count, arg_values.len() as u32);

                    cur_stack_slice = stack.func_call_grow_stack(
                        func_info.stack_size,
                        arg_values,
                        ret_value_locs,
                        (insc_ptr as u32) + 1
                    );
                    insc_ptr = func_info.start_addr as usize;
                    continue;
                },
                Insc::FFICall { func_id, arg_values, ret_value_locs } => {
                    #[cfg(not(debug_assertions))]
                    let ffi_func = program.ffi_funcs.get_unchecked(*func_id as usize);
                    #[cfg(debug_assertions)]
                    let ffi_func = &program.ffi_funcs[*func_id as usize];

                    for arg_value in arg_values {
                        ffi_args.push(cur_stack_slice.get_value(*arg_value));
                    }

                    for ret_value_loc in ret_value_locs {
                        ffi_rets.push(cur_stack_slice.get_value_mut(*ret_value_loc));
                    }

                    match ffi_func.call_prechecked(&ffi_args, &mut ffi_rets[..]) {
                        Ok(()) => {},
                        // TODO support exception handling
                        Err(e) => panic!("{}", "exception: ".to_string() + &e.to_string())
                    }

                    ffi_args.clear();
                    ffi_rets.clear();
                },
                Insc::ReturnMultiple { ret_values } => {
                    if let Some((prev_stack_slice, ret_addr)) = stack.done_func_call_shrink_stack(&ret_values) {
                        insc_ptr = ret_addr as usize;
                        cur_stack_slice = prev_stack_slice;
                        continue;
                    } else {
                        for (i, ret_value_loc) in ret_values.iter().enumerate() {
                            outputs.get_unchecked_mut(i).write(cur_stack_slice.get_value(*ret_value_loc));
                        }
                        return;
                    }
                },
                Insc::ReturnOne { ret_value } => {
                    if let Some((prev_stack_slice, ret_addr)) = stack.done_func_call_shrink_stack1(*ret_value) {
                        insc_ptr = ret_addr as usize;
                        cur_stack_slice = prev_stack_slice;
                        continue;
                    } else {
                        debug_assert_eq!(outputs.len(), 1);
                        outputs.get_unchecked_mut(0).write(cur_stack_slice.get_value(*ret_value));
                        return;
                    }
                },
                Insc::ReturnNothing => {
                    if let Some((prev_stack_slice, ret_addr)) = stack.done_func_call_shrink_stack(&[]) {
                        insc_ptr = ret_addr as usize;
                        cur_stack_slice = prev_stack_slice;
                        continue;
                    } else {
                        debug_assert_eq!(outputs.len(), 0);
                        return;
                    }
                },
                Insc::UnreachableInsc => panic!("this is an internal, unreachable insc"),
                // _ => todo!("unimplemented insc")
            }

            insc_ptr += 1;
        }
    }
}
