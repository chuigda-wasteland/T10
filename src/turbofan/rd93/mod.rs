//! `rd93` 中实现了一个最小化、可运行的 VM，主要用作正式开发之前的 Benchmarking

use std::any::TypeId;
use std::mem::MaybeUninit;

use insc::{CompiledFuncInfo, CompiledProgram, Insc};

use crate::data::Value;
use crate::turbofan::stack::Stack;

pub mod insc;

pub struct RD93 ();

impl RD93 {
    pub unsafe fn run_func(
        program: &CompiledProgram,
        func_id: usize,
        args: &[Value],
        outputs: &mut [MaybeUninit<Value>]
    ) {
        let func_info: CompiledFuncInfo = program.funcs[func_id];
        let dummy_ret_locs = [];
        debug_assert_eq!(args.len(), func_info.arg_count);
        debug_assert_eq!(outputs.len(), func_info.ret_count);

        let mut insc_ptr = func_info.start_addr;
        let mut stack = Stack::new();
        let mut cur_stack_slice = stack.ext_func_call_grow_stack(
            func_info.stack_size,
            args,
            &dummy_ret_locs
        );
        for (idx, arg) in args.iter().enumerate() {
            cur_stack_slice.set_value(idx, *arg);
        }

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
                        insc_ptr = *jump_dest;
                        continue;
                    }
                },
                Insc::Jump { jump_dest } => {
                    insc_ptr = *jump_dest;
                    continue;
                },
                Insc::FuncCall { func_id, arg_values, ret_value_locs } => {
                    let func_info: CompiledFuncInfo = *program.funcs.get_unchecked(*func_id);
                    debug_assert_eq!(func_info.arg_count, arg_values.len());

                    cur_stack_slice = stack.func_call_grow_stack(
                        func_info.stack_size,
                        arg_values,
                        ret_value_locs,
                        insc_ptr + 1
                    );
                    insc_ptr = func_info.start_addr;
                    continue;
                },
                Insc::ReturnMultiple { ret_values } => {
                    if let Some((prev_stack_slice, ret_addr)) = stack.done_func_call_shrink_stack(&ret_values) {
                        insc_ptr = ret_addr;
                        cur_stack_slice = prev_stack_slice;
                        continue;
                    } else {
                        for (i, ret_value_loc) in ret_values.iter().enumerate() {
                            outputs.get_unchecked_mut(i).write(cur_stack_slice.get_value(*ret_value_loc));
                        }
                        return;
                    }
                },
                Insc::ReturnOne { ret_value: _ } => {
                    todo!("RETURN-ONE unimplemented yet")
                },
                Insc::ReturnNothing => {
                    if let Some((prev_stack_slice, ret_addr)) = stack.done_func_call_shrink_stack(&[]) {
                        insc_ptr = ret_addr;
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
