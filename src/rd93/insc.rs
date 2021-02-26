//! `insc` 中约定了VM模拟使用的“指令集”
//! 这里仅仅实现 micro bench 所需要的部分

pub enum Insc {
    MakeIntConst{ c: i64, dest_value: usize },
    IntAdd { lhs_value: usize, rhs_value: usize, dest_value: usize },
    IntSub { lhs_value: usize, rhs_value: usize, dest_value: usize },
    IntEq { lhs_value: usize, rhs_value: usize, dest_value: usize },
    IntGt { lhs_value: usize, rhs_value: usize, dest_value: usize },
    Incr { value: usize },
    JumpIfTrue { cond_value: usize, jump_dest: usize },
    Jump { jump_dest: usize },
    FuncCall { func_id: usize, arg_values: Vec<usize>, ret_value_locs: Vec<usize> },
    ReturnOne { ret_value: usize },
    ReturnMultiple { ret_values: Vec<usize> },
    ReturnNothing,
    UnreachableInsc
}

#[derive(Copy, Clone)]
pub struct CompiledFuncInfo {
    pub start_addr: usize,
    pub arg_count: usize,
    pub ret_count: usize,
    pub stack_size: usize,
}

impl CompiledFuncInfo {
    pub fn new(start_addr: usize, arg_count: usize, ret_count: usize, stack_size: usize) -> Self {
        Self {
            start_addr, arg_count, ret_count, stack_size
        }
    }
}

pub struct CompiledProgram {
    pub inscs: Vec<Insc>,
    pub funcs: Vec<CompiledFuncInfo>
}

impl CompiledProgram {
    pub fn new(inscs: Vec<Insc>, funcs: Vec<CompiledFuncInfo>) -> Self {
        Self {
            inscs,
            funcs
        }
    }
}
