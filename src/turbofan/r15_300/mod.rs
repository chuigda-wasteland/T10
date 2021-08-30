pub mod aligned_bytes;
pub mod program;
pub mod turbine;

use std::mem::{MaybeUninit, transmute};

use crate::turbofan::r15_300::program::{CompiledProgram, OpCode};
use crate::data::Value;
use crate::turbofan::stack::Stack;

pub struct R15_300();

impl R15_300 {
    pub unsafe fn run_func(
        program: &CompiledProgram,
        func_id: u32,
        args: &[Value],
        outputs: &mut [MaybeUninit<Value>]
    ) {
        let func_info = *program.funcs.get_unchecked(func_id as usize);
        let dummy_ret_locs = [];

        debug_assert_eq!(args.len() as u32, func_info.arg_count);
        debug_assert_eq!(outputs.len() as u32, func_info.ret_count);

        let mut insc_ptr = func_info.start_addr as usize;
        let mut stack = Stack::new();
        let mut slice = stack.ext_func_call_grow_stack(
            func_info.stack_size,
            args,
            &dummy_ret_locs
        );
        for (idx, arg) in args.iter().enumerate() {
            slice.set_value(idx as u32, *arg);
        }

        let mut ffi_args: Vec<Value> = Vec::with_capacity(8);
        let mut ffi_rets: Vec<&mut MaybeUninit<Value>> = Vec::with_capacity(3);

        loop {
            let insc = program.inscs.read_byte(insc_ptr);

            #[cfg(debug_assertions)]
            eprintln!("INSC[{:02x}] = {:02x}", insc_ptr, insc);

            let insc = transmute::<u8, OpCode>(insc);
            match insc {
                OpCode::MakeIntConst => {
                    let dest = program.inscs.read_u32(insc_ptr + 4);
                    let data = program.inscs.read_u64(insc_ptr + 8);
                    slice.set_value(dest, Value::from(transmute::<u64, i64>(data)));
                    insc_ptr += 16;
                },
                OpCode::IntAdd => {
                    let dest = program.inscs.read_u32(insc_ptr + 4);
                    let src1 = program.inscs.read_u32(insc_ptr + 8);
                    let src2 = program.inscs.read_u32(insc_ptr + 12);

                    let lhs = slice.get_value(src1);
                    let rhs = slice.get_value(src2);

                    let lhs = lhs.value_typed_data.inner.int;
                    let rhs = rhs.value_typed_data.inner.int;
                    let sum = lhs.wrapping_add(rhs);

                    slice.set_value(dest, Value::from(sum));
                    insc_ptr += 16;
                },
                OpCode::IntSub => {
                    let dest = program.inscs.read_u32(insc_ptr + 4);
                    let src1 = program.inscs.read_u32(insc_ptr + 8);
                    let src2 = program.inscs.read_u32(insc_ptr + 12);

                    let lhs = slice.get_value(src1);
                    let rhs = slice.get_value(src2);

                    let lhs = lhs.value_typed_data.inner.int;
                    let rhs = rhs.value_typed_data.inner.int;
                    let sum = lhs.wrapping_sub(rhs);

                    slice.set_value(dest, Value::from(sum));
                    insc_ptr += 16;
                },
                OpCode::IntEq => {
                    let dest = program.inscs.read_u32(insc_ptr + 4);
                    let src1 = program.inscs.read_u32(insc_ptr + 8);
                    let src2 = program.inscs.read_u32(insc_ptr + 12);

                    let lhs = slice.get_value(src1);
                    let rhs = slice.get_value(src2);

                    let lhs = lhs.value_typed_data.inner.int;
                    let rhs = rhs.value_typed_data.inner.int;

                    slice.set_value(dest, Value::from(lhs == rhs));
                    insc_ptr += 16;
                },
                OpCode::IntGt => {
                    let dest = program.inscs.read_u32(insc_ptr + 4);
                    let src1 = program.inscs.read_u32(insc_ptr + 8);
                    let src2 = program.inscs.read_u32(insc_ptr + 12);

                    let lhs = slice.get_value(src1);
                    let rhs = slice.get_value(src2);

                    let lhs = lhs.value_typed_data.inner.int;
                    let rhs = rhs.value_typed_data.inner.int;

                    slice.set_value(dest, Value::from(lhs > rhs));
                    insc_ptr += 16;
                },
                OpCode::Incr => {
                    let pos = program.inscs.read_u32(insc_ptr + 4);
                    let mut value = slice.get_value(pos);
                    value.value_typed_data.inner.int += 1;
                    slice.set_value(pos, value);
                    insc_ptr += 8;
                },
                OpCode::JumpIfTrue => {
                    let cond = program.inscs.read_u32(insc_ptr + 4);
                    let value = slice.get_value(cond);
                    if value.value_typed_data.inner.boolean {
                        let dest = program.inscs.read_u32(insc_ptr + 8);
                        insc_ptr = dest as usize;
                    } else {
                        insc_ptr += 16;
                    }
                },
                OpCode::Jump => {
                    let dest = program.inscs.read_u32(insc_ptr + 4);
                    insc_ptr = dest as usize;
                },
                OpCode::FuncCall => {
                    let arg_count = program.inscs.read_byte(insc_ptr + 1) as u32;
                    let ret_count = program.inscs.read_byte(insc_ptr + 2) as u32;
                    let padding = program.inscs.read_byte(insc_ptr + 3) as u32;
                    let func_id = program.inscs.read_u32(insc_ptr + 4);

                    let func_info = *program.funcs.get_unchecked(func_id as usize);

                    let arg_values = std::slice::from_raw_parts::<u32>(
                        program.inscs.offset_ptr(insc_ptr + 8),
                        arg_count as usize
                    );
                    let ret_values_locs = std::slice::from_raw_parts::<u32>(
                        program.inscs.offset_ptr(insc_ptr + 8 + (arg_count * 4) as usize),
                        ret_count as usize
                    );

                    slice = stack.func_call_grow_stack(
                        func_info.stack_size,
                        arg_values,
                        ret_values_locs,
                        (insc_ptr as u32) + 8 + arg_count * 4 + ret_count * 4 + padding
                    );
                    insc_ptr = func_info.start_addr as usize;
                },
                OpCode::FFICall => {
                    let arg_count = program.inscs.read_byte(insc_ptr + 1) as usize;
                    let ret_count = program.inscs.read_byte(insc_ptr + 2) as usize;
                    let padding = program.inscs.read_byte(insc_ptr + 3) as u32;
                    let func_id = program.inscs.read_u32(insc_ptr + 4);

                    let ffi_func = program.ffi_funcs.get_unchecked(func_id as usize);

                    let arg_values = std::slice::from_raw_parts::<u32>(
                        program.inscs.offset_ptr(insc_ptr + 8),
                        arg_count
                    );
                    let ret_values_locs = std::slice::from_raw_parts::<u32>(
                        program.inscs.offset_ptr(insc_ptr + 8 + arg_count * 4),
                        ret_count
                    );

                    for arg_value in arg_values {
                        ffi_args.push(slice.get_value(*arg_value));
                    }
                    for ret_value_loc in ret_values_locs {
                        ffi_rets.push(slice.get_value_mut(*ret_value_loc));
                    }

                    match ffi_func.call_prechecked(&ffi_args, &mut ffi_rets[..]) {
                        Ok(()) => {},
                        // TODO support exception handling
                        Err(e) => panic!("{}", "exception: ".to_string() + &e.to_string())
                    }

                    ffi_args.clear();
                    ffi_rets.clear();
                    insc_ptr += 8 + arg_count * 4 + ret_count * 4 + padding as usize;
                },
                OpCode::ReturnMultiple => {
                    let ret_count = program.inscs.read_byte(insc_ptr + 1) as usize;

                    let ret_values = program.inscs.offset_ptr(insc_ptr + 4);
                    let ret_values = std::slice::from_raw_parts(ret_values, ret_count);

                    if let Some((prev_stack_slice, ret_addr)) = stack.done_func_call_shrink_stack(&ret_values) {
                        insc_ptr = ret_addr as usize;
                        slice = prev_stack_slice;
                    } else {
                        for (i, ret_value_loc) in ret_values.iter().enumerate() {
                            outputs.get_unchecked_mut(i).write(slice.get_value(*ret_value_loc));
                        }
                        return;
                    }
                },
                OpCode::ReturnOne => {
                    let ret_value = program.inscs.read_u32(insc_ptr + 4);
                    if let Some((prev_stack_slice, ret_addr)) = stack.done_func_call_shrink_stack1(ret_value) {
                        insc_ptr = ret_addr as usize;
                        slice = prev_stack_slice;
                    } else {
                        debug_assert_eq!(outputs.len(), 1);
                        outputs.get_unchecked_mut(0).write(slice.get_value(ret_value));
                        return;
                    }
                },
                OpCode::ReturnNothing => {
                    if let Some((prev_stack_slice, ret_addr)) = stack.done_func_call_shrink_stack(&[]) {
                        insc_ptr = ret_addr as usize;
                        slice = prev_stack_slice;
                    } else {
                        debug_assert_eq!(outputs.len(), 0);
                        return;
                    }
                },
                OpCode::UnreachableInsc => panic!("this is an internal, unreachable insc"),
            }
        }
    }
}
