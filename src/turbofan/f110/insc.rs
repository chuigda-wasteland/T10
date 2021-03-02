pub trait VMValueTyped : Copy {}

impl VMValueTyped for i64 {}
impl VMValueTyped for f64 {}
impl VMValueTyped for char {}
impl VMValueTyped for bool {}

#[derive(Copy, Clone)]
pub enum OpData<T: VMValueTyped> {
    Const(T),
    Stack(usize),
    StackValue(usize)
}

#[derive(Copy, Clone)]
pub struct Operand<T: VMValueTyped> {
    tyck: bool,
    rtlc: bool,
    data: OpData<T>
}

/*
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
*/

pub enum Insc {
    IntAdd { lhs: Operand<i64>, rhs: Operand<i64>, dest_value: usize },
    IntSub { lhs: Operand<i64>, rhs: Operand<i64>, dest_value: usize }
}

#[cfg(test)]
mod test {
    use crate::turbofan::f110::insc::Insc;

    #[test]
    fn print_insc_size() {
        eprintln!("std::mem::size_of::<t10::turbofan::f110::insc::Insc>() = {}",
                  std::mem::size_of::<Insc>())
    }
}
