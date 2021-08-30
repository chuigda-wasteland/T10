use std::mem::MaybeUninit;
use std::time::Instant;

use t10::data::Value;
use t10::turbofan::r15_300::program::CompiledProgram;
use t10::turbofan::r15_300::R15_300;
use t10::turbofan::r15_300::turbine::CompiledProgramBuilder;

const BENCH_RUNS: i32 = 10;

fn fib35_program() -> CompiledProgram {
    let mut builder = CompiledProgramBuilder::new(vec![]);
    builder.create_fn("fibonacci", 1, 1, 4);

    builder.make_int_const(0, 1);
    builder.int_eq(2, 0, 1);
    builder.jump_if_true_dangle(2, 0);

    builder.make_int_const(1, 1);
    builder.int_eq(2, 0, 1);
    builder.jump_if_true_dangle(2, 0);

    builder.int_sub(2, 0, 1);
    builder.make_int_const(2, 1);
    builder.int_sub(3, 0, 1);
    builder.func_call_dangle("fibonacci", &[2], &[2]);
    builder.func_call_dangle("fibonacci", &[3], &[3]);
    builder.int_add(2, 2, 3);
    builder.return_one(2);

    builder.create_label(0);
    builder.return_one(1);

    builder.finish_function();
    builder.finish()
}

fn bench(program: &CompiledProgram, args: &[Value], outputs: &mut [MaybeUninit<Value>]) {
    for _ in 0..BENCH_RUNS {
        let start_time = Instant::now();
        unsafe { R15_300::run_func(&program, 0, args, outputs); };
        let end_time = Instant::now();
        eprintln!("{} millis elapsed", (end_time - start_time).as_millis());
    }
}

fn main() {
    let program = fib35_program();
    bench(&program, &[Value::from(35i64)], &mut [MaybeUninit::uninit()])
}
