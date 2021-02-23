//! `insc` 中约定了VM模拟使用的“指令集”
//! 这里仅仅实现 microbench 所需要的部分

pub enum Insc {
    MakeIntConst{ c: i64, dest_value: usize },
    IntAdd { lhs_value: usize, rhs_value: usize, dest_value: usize },
    IntSub { lhs_value: usize, rhs_value: usize, dest_value: usize },
    IntEq{ lhs_value: usize, rhs_value: usize, dest_value: usize },
    JumpIfTrue { cond_value: usize, jump_dest: usize },
    FuncCall { func_id: usize, arg_values: Vec<usize>, ret_value_dest: usize },
    Return { ret_value: usize },
    ReturnNothing,
    UnreachableInsc
}

#[derive(Copy, Clone)]
pub struct CompiledFuncInfo {
    pub start_addr: usize,
    pub arg_count: usize,
    pub stack_size: usize
}

impl CompiledFuncInfo {
    pub fn new(start_addr: usize, arg_count: usize, stack_size: usize) -> Self {
        Self {
            start_addr, arg_count, stack_size
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
