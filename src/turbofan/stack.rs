use std::mem::MaybeUninit;

use crate::data::Value;

#[derive(Copy, Clone)]
pub struct StackSlice(*mut [MaybeUninit<Value>]);

impl StackSlice {
    #[cfg(not(debug_assertions))]
    pub unsafe fn set_value(&mut self, idx: usize, value: Value) {
        let _ = *self.0.as_mut().unwrap_unchecked().get_unchecked_mut(idx).write(value);
    }

    #[cfg(not(debug_assertions))]
    pub unsafe fn get_value(&mut self, idx: usize) -> Value {
        self.0.as_ref().unwrap_unchecked().get_unchecked(idx).assume_init_read()
    }

    #[cfg(debug_assertions)]
    pub unsafe fn set_value(&mut self, idx: usize, value: Value) {
        self.0.as_mut().unwrap_unchecked()[idx].write(value);
    }

    #[cfg(debug_assertions)]
    pub unsafe fn get_value(&mut self, idx: usize) -> Value {
        self.0.as_ref().unwrap_unchecked()[idx].assume_init_read()
    }
}

#[derive(Debug)]
pub struct FrameInfo<'a> {
    pub frame_start: usize,
    pub frame_end: usize,
    pub ret_value_locs: &'a [usize],
    pub ret_addr: usize
}

impl<'a> FrameInfo<'a> {
    pub fn new(
        frame_start: usize, frame_end: usize, ret_value_locs: &'a [usize], ret_addr: usize
    ) -> Self {
        Self {
            frame_start,
            frame_end,
            ret_value_locs,
            ret_addr
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

    pub unsafe fn ext_func_call_grow_stack(
        &mut self,
        frame_size: usize,
        args: &[Value],
        dummy_ret_value_locs: &'a [usize]
    ) -> StackSlice {
        debug_assert_eq!(self.values.len(), 0);
        debug_assert_eq!(self.frames.len(), 0);

        self.values.resize(frame_size, MaybeUninit::uninit());
        for (i, arg) in args.iter().enumerate() {
            self.values.get_unchecked_mut(i).write(*arg);
        }
        self.frames.push(FrameInfo::new(0, frame_size, dummy_ret_value_locs, 0));
        StackSlice(&mut self.values[..] as *mut [MaybeUninit<Value>])
    }

    pub unsafe fn func_call_grow_stack(
        &mut self,
        frame_size: usize,
        arg_locs: &[usize],
        ret_value_locs: &'a [usize],
        ret_addr: usize
    ) -> StackSlice {
        let this_frame = self.frames.last().unwrap_unchecked();
        let (this_frame_start, this_frame_end) = (this_frame.frame_start, this_frame.frame_end);

        debug_assert_eq!(this_frame_end, self.values.len());
        let new_frame_end = this_frame_end + frame_size;
        self.values.resize(new_frame_end, MaybeUninit::uninit());
        self.frames.push(FrameInfo::new(this_frame_end, new_frame_end, ret_value_locs, ret_addr));
        let mut old_slice = StackSlice(
            &mut self.values[this_frame_start..this_frame_end] as *mut [MaybeUninit<Value>]
        );
        let mut new_slice = StackSlice(
            &mut self.values[this_frame_end..new_frame_end] as *mut [MaybeUninit<Value>]
        );
        for (i, arg_loc) in arg_locs.iter().enumerate() {
            new_slice.set_value(i, old_slice.get_value(*arg_loc));
        }
        new_slice
    }

    pub unsafe fn done_func_call_shrink_stack(
        &mut self,
        ret_values: &[usize]
    ) -> Option<(StackSlice, usize)> {
        let frame_count = self.frames.len();
        if frame_count == 1 {
            return None;
        }

        let this_frame: &FrameInfo = self.frames.get_unchecked(frame_count - 1);
        let prev_frame: &FrameInfo = self.frames.get_unchecked(frame_count - 2);
        debug_assert_eq!(prev_frame.frame_end, this_frame.frame_start);
        let mut this_slice =
            StackSlice(&mut self.values[this_frame.frame_start..this_frame.frame_end]);
        let mut prev_slice =
            StackSlice(&mut self.values[prev_frame.frame_start..prev_frame.frame_end]);

        debug_assert_eq!(ret_values.len(), this_frame.ret_value_locs.len());
        for (ret_value, ret_value_loc) in
            ret_values.iter().zip(this_frame.ret_value_locs)
        {
            prev_slice.set_value(*ret_value_loc, this_slice.get_value(*ret_value))
        }
        let ret_addr = this_frame.ret_addr;
        self.values.truncate(prev_frame.frame_end);
        self.frames.pop().unwrap_unchecked();
        Some((prev_slice, ret_addr))
    }

    pub unsafe fn done_func_call_shrink_stack1(
        &mut self,
        ret_value: usize
    ) -> Option<(StackSlice, usize)> {
        let frame_count = self.frames.len();
        if frame_count == 1 {
            return None;
        }

        let this_frame: &FrameInfo = self.frames.get_unchecked(frame_count - 1);
        let prev_frame: &FrameInfo = self.frames.get_unchecked(frame_count - 2);
        debug_assert_eq!(prev_frame.frame_end, this_frame.frame_start);
        let mut this_slice =
            StackSlice(&mut self.values[this_frame.frame_start..this_frame.frame_end]);
        let mut prev_slice =
            StackSlice(&mut self.values[prev_frame.frame_start..prev_frame.frame_end]);

        debug_assert_eq!(this_frame.ret_value_locs.len(), 1);
        prev_slice.set_value(*this_frame.ret_value_locs.get_unchecked(0),
                             this_slice.get_value(ret_value));
        let ret_addr = this_frame.ret_addr;
        self.values.truncate(prev_frame.frame_end);
        self.frames.pop().unwrap_unchecked();
        Some((prev_slice, ret_addr))
    }
}
