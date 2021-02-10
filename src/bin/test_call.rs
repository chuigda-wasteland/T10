use t10::data::{StaticWrapper, DynBase, Value};
use t10::func::{RustFunction, RustCallable};
use std::marker::PhantomData;

struct S(i32);

fn bar(x: &mut S, y: &S) -> i64 {
    (x.0 + y.0) as i64
}

fn main() {
    let s1 = Box::leak(Box::new(StaticWrapper::owned(S(0)))) as *mut dyn DynBase;
    let s2 = Box::leak(Box::new(StaticWrapper::owned(S(4)))) as *mut dyn DynBase;
    let v1 = Value::from(s1);
    let v2 = Value::from(s2);
    let f = RustFunction { f: bar, _phantom: PhantomData::default() };

    unsafe {
        for _ in 0..10000 {
            for _ in 0..10000 {
                let _x = f.call_prechecked(&[v1, v2]).unwrap().value_typed_data.inner.int;
            }
        }
    }
}
