use crate::turbofan::r15_300::program::{CompiledProgram, CompiledFuncInfo};
use std::collections::HashMap;
use crate::turbofan::r15_300::program::OpCode;
use crate::func::RustCallable;
use crate::turbofan::r15_300::aligned_bytes::AlignedBytes;

pub struct CompiledProgramBuilder {
    pub program: CompiledProgram,
    pub func_maps: HashMap<String, u32>,
    pub label_maps: HashMap<u32, u32>,
    pub incomplete_conditional_jumps: Vec<(u32, u32)>,
    pub incomplete_jumps: Vec<(u32, u32)>
}

impl CompiledProgramBuilder {
    pub fn new(ffi_funcs: Vec<Box<dyn RustCallable>>) -> Self {
        Self {
            program: CompiledProgram::new(AlignedBytes::new(), vec![], ffi_funcs),
            func_maps: HashMap::new(),
            label_maps: HashMap::new(),
            incomplete_conditional_jumps: vec![],
            incomplete_jumps: vec![]
        }
    }

    pub fn create_fn(
        &mut self,
        func_name: String,
        arg_count: u32,
        ret_count: u32,
        stack_size: u32
    ) -> u32 {
        self.program.inscs.assert_aligned(8);

        let start_addr = self.program.inscs.len() as u32;
        let func_info = CompiledFuncInfo::new(start_addr, arg_count, ret_count, stack_size);
        let func_idx = self.program.funcs.len();
        self.program.funcs.push(func_info);
        self.label_maps.clear();
        self.func_maps.insert(func_name, func_idx as u32);
        func_idx as u32
    }

    pub fn create_label(
        &mut self,
        label_id: u32,
    ) -> u32 {
        self.program.inscs.assert_aligned(8);

        let label_addr = self.program.inscs.len() as u32;
        self.label_maps.insert(label_id, label_addr);
        label_addr
    }

    pub fn make_int_const(&mut self, c: i64, dest: u32) {
        self.program.inscs.assert_aligned(8);

        self.program.inscs.push_byte(OpCode::MakeIntConst as u8);
        unsafe { self.program.inscs.push_zero_bytes(3); }
        self.program.inscs.push_u32(dest);
        self.program.inscs.push_u64(unsafe { std::mem::transmute::<i64, u64>(c) });
    }

    fn gen_3ac(&mut self, op_code: u8, dest: u32, src1: u32, src2: u32) {
        self.program.inscs.assert_aligned(8);

        self.program.inscs.push_byte(op_code);
        unsafe { self.program.inscs.push_zero_bytes(3); }
        self.program.inscs.push_u32(dest);
        self.program.inscs.push_u32(src1);
        self.program.inscs.push_u32(src2);
    }

    pub fn int_add(&mut self, dest: u32, src1: u32, src2: u32) {
        self.gen_3ac(OpCode::IntAdd as u8, dest, src1, src2);
    }

    pub fn int_sub(&mut self, dest: u32, src1: u32, src2: u32) {
        self.gen_3ac(OpCode::IntSub as u8, dest, src1, src2);
    }

    pub fn incr(&mut self, pos: u32) {
        self.program.inscs.assert_aligned(8);

        self.program.inscs.push_byte(OpCode::Incr as u8);
        unsafe { self.program.inscs.push_zero_bytes(3); }
        self.program.inscs.push_u32(pos);
    }

    pub fn jump_if_true(&mut self, cond: u32, dest: u32) {
        self.program.inscs.assert_aligned(8);

        self.program.inscs.push_byte(OpCode::JumpIfTrue as u8);
        unsafe { self.program.inscs.push_zero_bytes(3); }
        self.program.inscs.push_u32(cond);
        self.program.inscs.push_u32(dest);
        unsafe { self.program.inscs.push_zero_bytes(4); }
    }

    pub fn jump_if_true_dangle(&mut self, cond: u32) -> u32 {
        self.program.inscs.assert_aligned(8);

        let insc_pos = self.program.inscs.len();
        self.program.inscs.push_byte(OpCode::JumpIfTrue as u8);
        unsafe { self.program.inscs.push_zero_bytes(3); }
        self.program.inscs.push_u32(cond);
        unsafe { self.program.inscs.push_zero_bytes(8); }
        insc_pos as u32
    }

    pub fn jump(&mut self, dest: u32) {
        self.program.inscs.assert_aligned(8);

        self.program.inscs.push_byte(OpCode::JumpIfTrue as u8);
        unsafe { self.program.inscs.push_zero_bytes(3); }
        self.program.inscs.push_u32(dest);
    }
}
