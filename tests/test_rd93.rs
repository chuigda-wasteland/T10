use t10::data::Value;
use t10::rd93::vm::RD93;
use t10::rd93::insc::{CompiledFuncInfo, CompiledProgram, Insc};

#[test]
fn test_add_func() {
    let program = CompiledProgram::new(vec![
        Insc::IntAdd { lhs_value: 0, rhs_value: 1, dest_value: 0 },
        Insc::Return { ret_value: 0 }
    ], vec![
        CompiledFuncInfo::new(0, 2, 2)
    ]);

    let mut rd93 = RD93::new(&program);
    let ret_value = unsafe {
        rd93.run_func(0, &[Value::from(13i64), Value::from(42i64)])
    };
    unsafe {
        assert_eq!(ret_value.value_typed_data.inner.int, 55);
    }
}
