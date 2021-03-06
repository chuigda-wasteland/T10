use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::time::Instant;

use t10::data::Value;
use t10::func::RustFunction;
use t10::turbofan::rd93::{CompiledFuncInfo, CompiledProgram, Insc, RD93};

#[cfg(not(debug_assertions))]
const BENCH_RUNS: i32 = 10;
#[cfg(debug_assertions)]
const BENCH_RUNS: i32 = 1;

fn bench(program: &CompiledProgram, args: &[Value], outputs: &mut [MaybeUninit<Value>]) {
    for _ in 0..BENCH_RUNS {
        let start_time = Instant::now();
        unsafe {
            RD93::run_func(&program, 0, args, outputs);
        };
        let end_time = Instant::now();
        eprintln!("{} millis elapsed", (end_time - start_time).as_millis());
    }
}

fn bench_fib35() {
    let program = CompiledProgram::new(vec![
        // fibonacci(n int @0) -> int
        /*00*/ Insc::MakeIntConst { c: 0, dest_value: 1 },
        /*01*/ Insc::IntEq { lhs_value: 0, rhs_value: 1, dest_value: 2 },
        /*02*/ Insc::JumpIfTrue { cond_value: 2, jump_dest: 13 },
        /*03*/ Insc::MakeIntConst { c: 1, dest_value: 1 },
        /*04*/ Insc::IntEq { lhs_value: 0, rhs_value: 1, dest_value: 2 },
        /*05*/ Insc::JumpIfTrue { cond_value: 2, jump_dest: 13 },
        /*06*/ Insc::IntSub { lhs_value: 0, rhs_value: 1, dest_value: 2 },
        /*07*/ Insc::MakeIntConst { c: 2, dest_value: 1 },
        /*08*/ Insc::IntSub { lhs_value: 0, rhs_value: 1, dest_value: 3 },
        /*09*/ Insc::FuncCall { func_id: 0, arg_values: vec![2], ret_value_locs: vec![2] },
        /*10*/ Insc::FuncCall { func_id: 0, arg_values: vec![3], ret_value_locs: vec![3] },
        /*11*/ Insc::IntAdd { lhs_value: 2, rhs_value: 3, dest_value: 2 },
        /*12*/ Insc::ReturnOne { ret_value: 2 },
        /*13*/ Insc::ReturnOne { ret_value: 1 }
    ], vec![
        CompiledFuncInfo::new(0, 1, 1, 4),
    ], vec![]);
    bench(&program, &[Value::from(35i64)], &mut [MaybeUninit::uninit()]);
}

fn bench_loop100m() {
    let program = CompiledProgram::new(vec![
        // application_start() -> void
        /*00*/ Insc::MakeIntConst { c: 10000, dest_value: 0 },
        /*01*/ Insc::MakeIntConst { c: 1, dest_value: 1 },
        /*02*/ Insc::IntGt { lhs_value: 1, rhs_value: 0, dest_value: 3 },
        /*03*/ Insc::JumpIfTrue { cond_value: 3, jump_dest: 12 },
        /*04*/ Insc::MakeIntConst { c: 1, dest_value: 2 },
        /*05*/ Insc::IntGt { lhs_value: 2, rhs_value: 0, dest_value: 3 },
        /*06*/ Insc::JumpIfTrue { cond_value: 3, jump_dest: 10 },
        /*07*/ Insc::IntAdd { lhs_value: 1, rhs_value: 2, dest_value: 3 },
        /*08*/ Insc::Incr { value: 2 },
        /*09*/ Insc::Jump { jump_dest: 5 },
        /*10*/ Insc::Incr { value: 1 },
        /*11*/ Insc::Jump { jump_dest: 2 },
        /*12*/ Insc::ReturnNothing
    ], vec![
        CompiledFuncInfo::new(0, 0, 0, 4)
    ], vec![]);
    bench(&program, &[], &mut []);
}

fn baz(x: i64, y: i64) -> i64 {
    x + y
}

fn bench_ffi100m() {
    let program = CompiledProgram::new(vec![
        // application_start() -> void
        /*00*/ Insc::MakeIntConst { c: 10000, dest_value: 0 },
        /*01*/ Insc::MakeIntConst { c: 1, dest_value: 1 },
        /*02*/ Insc::IntGt { lhs_value: 1, rhs_value: 0, dest_value: 3 },
        /*03*/ Insc::JumpIfTrue { cond_value: 3, jump_dest: 12 },
        /*04*/ Insc::MakeIntConst { c: 1, dest_value: 2 },
        /*05*/ Insc::IntGt { lhs_value: 2, rhs_value: 0, dest_value: 3 },
        /*06*/ Insc::JumpIfTrue { cond_value: 3, jump_dest: 10 },
        /*07*/ Insc::FFICall { func_id: 0, arg_values: vec![1, 2], ret_value_locs: vec![3] },
        /*08*/ Insc::Incr { value: 2 },
        /*09*/ Insc::Jump { jump_dest: 5 },
        /*10*/ Insc::Incr { value: 1 },
        /*11*/ Insc::Jump { jump_dest: 2 },
        /*12*/ Insc::ReturnNothing
    ], vec![
        CompiledFuncInfo::new(0, 0, 0, 4)
    ], vec![
        Box::new(RustFunction { f: baz, _phantom: PhantomData::default() })
    ]);
    bench(&program, &[], &mut []);
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args[1].as_str() {
        "fib35" => bench_fib35(),
        "loop100m" => bench_loop100m(),
        "ffi100m" => bench_ffi100m(),
        _ => panic!("unknown benchmark")
    }
}
