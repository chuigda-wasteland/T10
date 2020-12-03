use std::marker::PhantomData;

#[repr(transparent)]
struct VMValueVec<T: Copy> {
    vec: Vec<T>
}
