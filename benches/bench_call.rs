#![feature(test)]
#![feature(core_intrinsics)]

extern crate test;

use std::marker::PhantomData;
use std::intrinsics::volatile_load;
use std::mem::MaybeUninit;

use test::Bencher;

use t10::data::{StaticWrapper, DynBase, Value};
use t10::func::{RustCallable, RustFunction};

struct S(i32);

fn bar(x: &mut S, y: &S) -> i64 {
    (x.0 + y.0) as i64
}

fn baz(x: i64, y: i64) -> i64 {
    x + y
}

#[bench]
fn bench_simple_call(b: &mut Bencher) {
    let s1 = Box::leak(Box::new(StaticWrapper::owned(S(0)))) as *mut dyn DynBase;
    let s2 = Box::leak(Box::new(StaticWrapper::owned(S(4)))) as *mut dyn DynBase;
    let v1 = Value::from(s1);
    let v2 = Value::from(s2);
    let f = RustFunction { f: bar, _phantom: PhantomData::default() };

    let mut dest = MaybeUninit::uninit();
    let mut dest_value_ref = [&mut dest];

    b.iter(|| {
        unsafe {
            for _ in 0..1000 {
                for _ in 0..1000 {
                    f.call_prechecked(&[v1, v2], &mut dest_value_ref).unwrap();
                }
            }
        }
    })
}

#[bench]
fn bench_rust_call(b: &mut Bencher) {
    let mut s1 = S(13);
    let s2 = S(5);
    b.iter(|| {
        for _ in 0..1000 {
            for _ in 0..1000 {
                unsafe {
                    let _x = bar(
                        &mut volatile_load(&mut s1 as *mut S),
                        &volatile_load(&s2 as *const S as *mut S)
                    );
                }
            }
        }
    })
}

#[bench]
fn bench_simple_call2(b: &mut Bencher) {
    let f = RustFunction { f: baz, _phantom: PhantomData::default() };
    let mut dest = MaybeUninit::uninit();
    let mut dest_value_ref = [&mut dest];
    b.iter(|| {
        for i in 0..1000i64 {
            for j in 0..1000i64 {
                let v1 = Value::from(i);
                let v2 = Value::from(j);
                unsafe {
                    f.call_prechecked(&[v1, v2], &mut dest_value_ref).unwrap();
                }
            }
        }
    })
}
