//! `vm` 实现了虚拟机的总体布局

use crate::rd93::insc::{Insc, CompiledProgram};
use crate::rd93::scope::Scope;

pub struct RD93<'a> {
    pub program: &'a CompiledProgram,
    pub scopes: Vec<Scope>

}

impl<'a> RD93<'a> {
    pub fn new(program: &'a CompiledProgram) -> Self {
        Self {
            program,
            scopes: vec![]
        }
    }

    pub fn run_insc(_insc: Insc) -> ! {
        unimplemented!()
    }
}
