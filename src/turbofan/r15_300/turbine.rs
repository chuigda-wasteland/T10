use std::collections::HashMap;

use crate::turbofan::r15_300::program::{CompiledProgram, CompiledFuncInfo};
use crate::turbofan::r15_300::program::OpCode;
use crate::func::RustCallable;
use crate::turbofan::r15_300::aligned_bytes::AlignedBytes;

pub struct CompiledProgramBuilder {
    pub program: CompiledProgram,
    pub func_maps: HashMap<String, u32>,
    pub label_maps: HashMap<u32, u32>,
    pub incomplete_conditional_jumps: Vec<(u32, u32)>,
    pub incomplete_jumps: Vec<(u32, u32)>,
    pub incomplete_calls: Vec<(u32, String)>,
}

impl CompiledProgramBuilder {
    pub fn new(ffi_funcs: Vec<Box<dyn RustCallable>>) -> Self {
        Self {
            program: CompiledProgram::new(AlignedBytes::new(), vec![], ffi_funcs),
            func_maps: HashMap::new(),
            label_maps: HashMap::new(),
            incomplete_conditional_jumps: vec![],
            incomplete_jumps: vec![],
            incomplete_calls: vec![]
        }
    }

    pub fn create_fn(
        &mut self,
        func_name: impl ToString,
        arg_count: u32,
        ret_count: u32,
        stack_size: u32
    ) -> u32 {
        self.program.inscs.assert_aligned(8);
        assert_eq!(self.incomplete_conditional_jumps.len(), 0);
        assert_eq!(self.incomplete_jumps.len(), 0);
        assert_eq!(self.label_maps.len(), 0);

        let start_addr = self.program.inscs.len() as u32;
        let func_info = CompiledFuncInfo::new(start_addr, arg_count, ret_count, stack_size);
        let func_idx = self.program.funcs.len();
        self.program.funcs.push(func_info);
        self.func_maps.insert(func_name.to_string(), func_idx as u32);
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

    pub fn int_eq(&mut self, dest: u32, src1: u32, src2: u32) {
        self.gen_3ac(OpCode::IntEq as u8, dest, src1, src2);
    }

    pub fn incr(&mut self, pos: u32) {
        self.program.inscs.assert_aligned(8);

        self.program.inscs.push_byte(OpCode::Incr as u8);
        unsafe { self.program.inscs.push_zero_bytes(3); }
        self.program.inscs.push_u32(pos);
    }

    pub fn jump_if_true_dangle(&mut self, cond: u32, dest_label: u32) {
        self.program.inscs.assert_aligned(8);

        let insc_pos = self.program.inscs.len();
        self.program.inscs.push_byte(OpCode::JumpIfTrue as u8);
        unsafe { self.program.inscs.push_zero_bytes(3); }
        self.program.inscs.push_u32(cond);
        unsafe { self.program.inscs.push_zero_bytes(8); }
        self.incomplete_conditional_jumps.push((insc_pos as u32, dest_label));
    }

    pub fn jump_dangle(&mut self, dest_label: u32) {
        self.program.inscs.assert_aligned(8);

        let insc_pos = self.program.inscs.len();
        self.program.inscs.push_byte(OpCode::Jump as u8);
        unsafe { self.program.inscs.push_zero_bytes(7); }
        self.incomplete_jumps.push((insc_pos as u32, dest_label));
    }

    pub fn func_call_dangle(&mut self, func_name: impl ToString, args: &[u32], rets: &[u32]) {
        self.program.inscs.assert_aligned(8);

        assert!(args.len() <= u8::MAX as usize);
        assert!(rets.len() <= u8::MAX as usize);

        let insc_pos = self.program.inscs.len();
        self.program.inscs.push_byte(OpCode::FuncCall as u8);
        self.program.inscs.push_byte(args.len() as u8);
        self.program.inscs.push_byte(rets.len() as u8);

        let padding = (args.len() * 4 + rets.len() * 4) % 8;
        self.program.inscs.push_byte(padding as u8);

        unsafe { self.program.inscs.push_zero_bytes(4); }

        for arg in args {
            self.program.inscs.push_u32(*arg);
        }
        for ret in rets {
            self.program.inscs.push_u32(*ret);
        }
        unsafe { self.program.inscs.push_zero_bytes(padding); }

        self.incomplete_calls.push((insc_pos as u32, func_name.to_string()));
    }

    pub fn ffi_call(&mut self, ffi_func_id: u32, args: &[u32], rets: &[u32]) {
        self.program.inscs.assert_aligned(8);

        assert!(args.len() <= u8::MAX as usize);
        assert!(rets.len() <= u8::MAX as usize);

        self.program.inscs.push_byte(OpCode::FFICall as u8);
        self.program.inscs.push_byte(args.len() as u8);
        self.program.inscs.push_byte(rets.len() as u8);

        let padding = (args.len() * 4 + rets.len() * 4) % 8;
        self.program.inscs.push_byte(padding as u8);

        self.program.inscs.push_u32(ffi_func_id);

        unsafe { self.program.inscs.push_zero_bytes(4); }

        for arg in args {
            self.program.inscs.push_u32(*arg);
        }
        for ret in rets {
            self.program.inscs.push_u32(*ret);
        }
        unsafe { self.program.inscs.push_zero_bytes(padding); }
    }

    pub fn return_one(&mut self, ret: u32) {
        self.program.inscs.assert_aligned(8);

        self.program.inscs.push_byte(OpCode::ReturnOne as u8);
        unsafe { self.program.inscs.push_zero_bytes(3); }
        self.program.inscs.push_u32(ret);
    }

    pub fn return_multiple(&mut self, rets: &[u32]) {
        self.program.inscs.assert_aligned(8);

        assert!(rets.len() <= u8::MAX as usize);

        self.program.inscs.push_byte(OpCode::ReturnMultiple as u8);
        self.program.inscs.push_byte(rets.len() as u8);

        let padding = (rets.len() * 4) % 8;
        self.program.inscs.push_byte(padding as u8);
        self.program.inscs.push_byte(0);

        for ret in rets {
            self.program.inscs.push_u32(*ret);
        }
        unsafe { self.program.inscs.push_zero_bytes(padding) };
    }

    pub fn return_nothing(&mut self) {
        self.program.inscs.assert_aligned(8);

        self.program.inscs.push_byte(OpCode::ReturnNothing as u8);
        unsafe { self.program.inscs.push_zero_bytes(7); }
    }

    pub fn unreachable(&mut self) {
        self.program.inscs.assert_aligned(8);

        self.program.inscs.push_byte(OpCode::UnreachableInsc as u8);
        unsafe { self.program.inscs.push_zero_bytes(7); }
    }

    pub fn finish_function(&mut self) {
        for (jump_insc_pos, jump_label) in self.incomplete_jumps.iter() {
            let label_pos = self.label_maps.get(jump_label).unwrap();
            unsafe { self.program.inscs.write_u32((jump_insc_pos + 4) as usize, *label_pos); }
        }

        for (jump_insc_pos, jump_label) in self.incomplete_conditional_jumps.iter() {
            let label_pos = self.label_maps.get(jump_label).unwrap();
            unsafe { self.program.inscs.write_u32((jump_insc_pos + 8) as usize, *label_pos); }
        }

        self.incomplete_jumps.clear();
        self.incomplete_conditional_jumps.clear();
        self.label_maps.clear();
    }

    pub fn finish(mut self) -> CompiledProgram {
        for (call_insc_pos, call_func_name) in self.incomplete_calls {
            let func_id = self.func_maps.get(&call_func_name).unwrap();
            unsafe { self.program.inscs.write_u32((call_insc_pos + 4) as usize, *func_id); }
        }
        self.program
    }
}
