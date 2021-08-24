//! `insc` 中约定了VM模拟使用的“指令集”
//! 这里仅仅实现 micro bench 所需要的部分

use crate::func::RustCallable;

pub enum Insc {
    MakeIntConst { c: i64, dest_value: u32 },
    IntAdd { lhs_value: u32, rhs_value: u32, dest_value: u32 },
    IntSub { lhs_value: u32, rhs_value: u32, dest_value: u32 },
    IntEq { lhs_value: u32, rhs_value: u32, dest_value: u32 },
    IntGt { lhs_value: u32, rhs_value: u32, dest_value: u32 },
    Incr { value: u32 },
    JumpIfTrue { cond_value: u32, jump_dest: u32 },
    Jump { jump_dest: u32 },
    FuncCall { func_id: u32, arg_values: Vec<u32>, ret_value_locs: Vec<u32> },
    FFICall { func_id: u32, arg_values: Vec<u32>, ret_value_locs: Vec<u32> },
    ReturnOne { ret_value: u32 },
    ReturnMultiple { ret_values: Vec<u32> },
    ReturnNothing,
    UnreachableInsc
}

#[derive(Copy, Clone)]
pub struct CompiledFuncInfo {
    pub start_addr: u32,
    pub arg_count: u32,
    pub ret_count: u32,
    pub stack_size: u32,
}

impl CompiledFuncInfo {
    pub fn new(start_addr: u32, arg_count: u32, ret_count: u32, stack_size: u32) -> Self {
        Self {
            start_addr, arg_count, ret_count, stack_size
        }
    }
}

pub struct CompiledProgram {
    pub inscs: Vec<Insc>,
    pub funcs: Vec<CompiledFuncInfo>,
    pub ffi_funcs: Vec<Box<dyn RustCallable>>
}

impl CompiledProgram {
    pub fn new(
        inscs: Vec<Insc>,
        funcs: Vec<CompiledFuncInfo>,
        ffi_funcs: Vec<Box<dyn RustCallable>>
    ) -> Self {
        Self {
            inscs,
            funcs,
            ffi_funcs
        }
    }
}

#[cfg(test)]
mod test {
    use crate::turbofan::rd93::insc::Insc;

    #[test]
    fn print_insc_size() {
        eprintln!("std::mem::size_of::<t10::turbofan::rd93::insc::Insc>() = {}",
                  std::mem::size_of::<Insc>())
    }
}
