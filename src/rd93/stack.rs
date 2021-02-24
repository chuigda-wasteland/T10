use std::mem::MaybeUninit;

use crate::data::Value;

#[derive(Copy, Clone)]
pub struct StackSlice(*mut [MaybeUninit<Value>]);

impl StackSlice {
    pub unsafe fn set_value(&mut self, idx: usize, value: Value) {
        unimplemented!()
    }

    pub unsafe fn get_value(&mut self, idx: usize) -> Value {
        unimplemented!()
    }
}

pub struct FrameInfo<'a> {
    pub frame_start: usize,
    pub frame_end: usize,
    pub ret_value_locs: &'a [usize]
}

impl<'a> FrameInfo<'a> {
    pub fn new(frame_start: usize, frame_end: usize, ret_value_locs: &'a [usize]) -> Self {
        Self {
            frame_start,
            frame_end,
            ret_value_locs
        }
    }
}

pub struct Stack<'a> {
    pub values: Vec<MaybeUninit<Value>>,
    pub frames: Vec<FrameInfo<'a>>
}

impl<'a> Stack<'a> {
    pub fn new() -> Self {
        Self {
            values: Vec::with_capacity(64),
            frames: Vec::with_capacity(4)
        }
    }

    pub unsafe fn func_call_grow_stack(
        &mut self,
        frame_size: usize,
        ret_value_locs: &'a [usize]
    ) -> StackSlice {
        let stack_size = self.values.len();
        let new_stack_size = stack_size + frame_size;
        self.values.resize(new_stack_size, MaybeUninit::uninit());
        self.frames.push(FrameInfo::new(stack_size, new_stack_size, ret_value_locs));
        StackSlice(&mut self.values[stack_size..new_stack_size] as *mut [MaybeUninit<Value>])
    }

    pub unsafe fn done_func_call_shrink_stack(
        &mut self
    ) -> Option<StackSlice> {
        let frame_count = self.frames.len();
        if frame_count == 1 {
            return None;
        }

        let this_frame: &FrameInfo = self.frames.get_unchecked(frame_count - 1);
        let prev_frame: &FrameInfo = self.frames.get_unchecked(frame_count - 2);
        let _this_slice = StackSlice(&mut self.values[this_frame.frame_start..this_frame.frame_end]);
        let _prev_slice = StackSlice(&mut self.values[prev_frame.frame_start..prev_frame.frame_end]);

        unimplemented!();
    }
}
