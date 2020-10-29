#[derive(Debug)]
pub enum TypeCheckInfo {
    SimpleType(std::any::TypeId),
    Container(std::any::TypeId, Vec<TypeCheckInfo>),
}

pub trait StaticBase {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool;

    fn tyck_info() -> TypeCheckInfo;
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
}

impl<T: 'static> StaticBase for &T {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        <T as StaticBase>::type_check(tyck_info)
    }

    fn tyck_info() -> TypeCheckInfo {
        <T as StaticBase>::tyck_info()
    }
}

impl<T: 'static> StaticBase for &mut T {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        <T as StaticBase>::type_check(tyck_info)
    }

    fn tyck_info() -> TypeCheckInfo {
        <T as StaticBase>::tyck_info()
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
