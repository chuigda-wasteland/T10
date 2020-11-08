use crate::cast::RustLifetime;

#[derive(Debug)]
pub enum TypeCheckInfo {
    SimpleType(std::any::TypeId),
    Container(std::any::TypeId, Vec<TypeCheckInfo>),
}

pub trait StaticBase {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool;

    fn tyck_info() -> TypeCheckInfo;

    fn lifetime_info() -> RustLifetime;
}

trait StaticBaseImpl<T> {
    fn lifetime_info_impl() -> RustLifetime;
}

impl<T> StaticBaseImpl<T> for () {
    default fn lifetime_info_impl() -> RustLifetime {
        RustLifetime::Move
    }
}

impl<T: Copy> StaticBaseImpl<T> for () {
    fn lifetime_info_impl() -> RustLifetime {
        RustLifetime::Copy
    }
}

impl<T: 'static> StaticBase for T {
    default fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        if let TypeCheckInfo::SimpleType(type_id) = tyck_info {
            std::any::TypeId::of::<T>() == *type_id
        } else {
            false
        }
    }

    default fn tyck_info() -> TypeCheckInfo {
        TypeCheckInfo::SimpleType(std::any::TypeId::of::<T>())
    }

    default fn lifetime_info() -> RustLifetime {
        <() as StaticBaseImpl<T>>::lifetime_info_impl()
    }
}

impl<T: 'static> StaticBase for &T {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        <T as StaticBase>::type_check(tyck_info)
    }

    fn tyck_info() -> TypeCheckInfo {
        <T as StaticBase>::tyck_info()
    }

    fn lifetime_info() -> RustLifetime {
        RustLifetime::Share
    }
}

impl<T: 'static> StaticBase for &mut T {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        <T as StaticBase>::type_check(tyck_info)
    }

    fn tyck_info() -> TypeCheckInfo {
        <T as StaticBase>::tyck_info()
    }

    fn lifetime_info() -> RustLifetime {
        RustLifetime::MutShare
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use crate::tyck::StaticBase;

    #[test]
    fn test1<'b>() {
        struct S<'a> { _phantom: PhantomData<&'a ()> }
        let tyck_info = <&'b S<'static> as StaticBase>::tyck_info();
        eprintln!("{:?}", tyck_info)
    }
}
