//! `insc` 中约定了VM模拟使用的“指令集”
//! 这里仅仅实现 microbench 所需要的部分

pub enum Insc {
    MakeIntConst{ c: i64, dest_value: usize },
    IntAdd { lhs_value: usize, rhs_value: usize, dest_value: usize },
    IntSub { lhs_value: usize, rhs_value: usize, dest_value: usize },
    IntEq{ lhs_value: usize, rhs_value: usize, dest_value: usize },
    JumpIfTrue { cond_value: usize, jump_dest: usize },
    FuncCall { func_id: usize, arg_values: Vec<usize> },
    Return { ret_value: usize },
    ReturnNothing,
    UnreachableInsc
}

pub struct CompiledProgram {
    pub inscs: Vec<Insc>,
    pub funcs: Vec<usize>
}

impl CompiledProgram {
    pub fn new(inscs: Vec<Insc>, funcs: Vec<usize>) -> Self {
        Self {
            inscs,
            funcs
        }
    }
}
