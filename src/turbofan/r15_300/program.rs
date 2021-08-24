use crate::func::RustCallable;

#[repr(u8)]
pub enum OpCode {
    // [OP:1] [PAD:3] [DEST:4] [VALUE:8]
    // IP = IP +16
    MakeIntConst = 1,

    // [OP:1] [PAD:3] [DEST:4] [SRC:4] [PAD:4]
    // IP = IP +16
    IntAdd = 2,
    IntSub = 3,
    IntEq = 4,
    IntGt = 5,

    // [OP:1] [PAD:3] [POS:4]
    // IP = IP + 8
    Incr = 6,

    // [OP:1] [PAD:3] [COND:4] [DEST:4] [PAD:4]
    // IP = IP +16
    JumpIfTrue = 7,

    // [OP:1] [PAD:3] [DEST:4]
    // IP = IP + 8
    Jump = 8,

    // [OP:1] [ARG_CNT:1] [RET_CNT:1] [W:1] [FUNC:4] [ARGS:4*ARG_CNT] [RETS:4*RET_CNT] [PAD:W]
    // IP = IP + 8 + (ARG_CNT * 4) + (RET_CNT * 4) * W
    FuncCall = 9,
    FFICall = 10,

    // [OP:1] [PAD:3] [RET:1]
    // IP = IP + 8
    ReturnOne = 11,

    // [OP:1] [RET_CNT:1] [W:1] [PAD:1] [RETS:4*RET_CNT] [PAD:W]
    // IP = IP + 4 + (RET_CNT*4) + W
    ReturnMultiple = 12,

    // [OP:1] [PAD:7]
    // IP = IP +8
    ReturnNothing = 13,
    UnreachableInsc = 14
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

}


