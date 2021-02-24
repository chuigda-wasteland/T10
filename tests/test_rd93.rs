use std::any::TypeId;

use t10::data::Value;
use t10::rd93::RD93;
use t10::rd93::insc::{CompiledFuncInfo, CompiledProgram, Insc};

#[test]
fn test_add_func() {
    let program = CompiledProgram::new(vec![
        // add(a int @%0, b int @%1) -> int
        Insc::IntAdd { lhs_value: 0, rhs_value: 1, dest_value: 0 },
        Insc::Return { ret_value: 0 }
    ], vec![
        CompiledFuncInfo::new(0, 2, 2)
    ]);

    let ret_value = unsafe {
        RD93::run_func(&program, 0, &[Value::from(13i64), Value::from(42i64)])
    };
    unsafe {
        assert_eq!(ret_value.type_id(), TypeId::of::<i64>());
        assert_eq!(ret_value.value_typed_data.inner.int, 55);
    }
}

#[test]
fn test_func_call() {
    let program = CompiledProgram::new(vec![
        // entry(a int @%0, b int @%1) -> int
        /*00*/ Insc::MakeIntConst { c: 1, dest_value: 2 },
        /*01*/ Insc::MakeIntConst { c: 2, dest_value: 3 },
        /*02*/ Insc::IntAdd { lhs_value: 0, rhs_value: 2, dest_value: 0 },
        /*03*/ Insc::IntAdd { lhs_value: 1, rhs_value: 3, dest_value: 1 },
        /*04*/ Insc::FuncCall { func_id: 1, arg_values: vec![0, 1], ret_value_dest: 0 },
        /*05*/ Insc::Return { ret_value: 0 },

        // add(a int @%0, b int @%1) -> int
        /*06*/ Insc::IntAdd { lhs_value: 0, rhs_value: 1, dest_value: 0 },
        /*07*/ Insc::Return { ret_value: 0 }
    ], vec![
        CompiledFuncInfo::new(0, 2, 4), // entry
        CompiledFuncInfo::new(6, 2, 2), // add
    ]);

    let ret_value = unsafe {
        RD93::run_func(&program, 0, &[Value::from(13i64), Value::from(42i64)])
    };
    unsafe {
        assert_eq!(ret_value.type_id(), TypeId::of::<i64>());
        assert_eq!(ret_value.value_typed_data.inner.int, 58);
    }
}

#[test]
fn test_fibonacci() {
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
        /*09*/ Insc::FuncCall { func_id: 0, arg_values: vec![2], ret_value_dest: 2 },
        /*10*/ Insc::FuncCall { func_id: 0, arg_values: vec![3], ret_value_dest: 3 },
        /*11*/ Insc::IntAdd { lhs_value: 2, rhs_value: 3, dest_value: 2 },
        /*12*/ Insc::Return { ret_value: 2 },
        /*13*/ Insc::Return { ret_value: 1 }
    ], vec![
        CompiledFuncInfo::new(0, 1, 4),
    ]);

    let ret_value = unsafe {
        RD93::run_func(&program, 0, &[Value::from(10i64)])
    };
    unsafe {
        assert_eq!(ret_value.type_id(), TypeId::of::<i64>());
        assert_eq!(ret_value.value_typed_data.inner.int, 55);
    }
}
