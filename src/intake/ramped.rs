use std::collections::HashSet;
use std::mem::MaybeUninit;
use crate::data::{StaticWrapper, Value};
use crate::intake::OxProvider;

pub struct IntakeOptions {
    max_debt: u32,
    max_debt_bytes: usize
}

impl IntakeOptions {
    pub fn new(max_debt: u32, max_debt_bytes: usize) -> Self {
        Self {
            max_debt,
            max_debt_bytes
        }
    }
}

impl Default for IntakeOptions {
    fn default() -> Self {
        Self::new(1024, 32768)
    }
}

pub struct RampedIntake<'a> {
    options: IntakeOptions,

    stacks: Vec<&'a Vec<MaybeUninit<Value>>>,
    heap: Vec<*mut Value>,

    collect_enabled: bool,
    debt: u32,
    debt_bytes: usize,
}

impl<'a> RampedIntake<'a> {
    pub fn new(options: IntakeOptions) -> Self {
        Self {
            options,

            stacks: Vec::new(),
            heap: Vec::new(),

            collect_enabled: false,
            debt: 0,
            debt_bytes: usize
        }
    }
}

impl<'a> Default for RampedIntake<'a> {
    fn default() -> Self {
        Self::new(IntakeOptions::default())
    }
}

impl<'a> OxProvider<'a> for RampedIntake<'a> {
    fn allocate<T>(&mut self, _t: T) -> StaticWrapper<T> {
        todo!()
    }

    fn allocate_ref<T>(&mut self, _t: &T) -> StaticWrapper<T> {
        todo!()
    }

    fn allocate_ref_mut<T>(&mut self, _t: &mut T) -> StaticWrapper<T> {
        todo!()
    }

    fn force_collect(&mut self) {
        todo!()
    }

    fn add_stack(&mut self, stack: &'a Vec<MaybeUninit<Value>>) {
        self.stacks.push(stack);
    }

    fn enable_collect(&mut self) {
        self.collect_enabled = true;
    }

    fn disable_collect(&mut self) {
        self.collect_enabled = false;
    }
}
