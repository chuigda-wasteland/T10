pub enum TypeCheckInfo {
    SimpleType(std::any::TypeId),
    Container(std::any::TypeId, Vec<TypeCheckInfo>),
}

pub trait StaticBase {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool;

    fn type_check_info() -> TypeCheckInfo;
}

impl StaticBase for i64 {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        if let TypeCheckInfo::SimpleType(type_id) = tyck_info {
            std::any::TypeId::of::<Self>() == *type_id
        } else {
            false
        }
    }

    fn type_check_info() -> TypeCheckInfo {
        TypeCheckInfo::SimpleType(std::any::TypeId::of::<Self>())
    }
}

impl<T: StaticBase> StaticBase for Vec<T> {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        if let TypeCheckInfo::Container(container_type_id, params) = tyck_info {
            std::any::TypeId::of::<Vec<()>>() == *container_type_id
                && params.len() == 1
                && T::type_check(&params[0])
        } else {
            false
        }
    }

    fn type_check_info() -> TypeCheckInfo {
        TypeCheckInfo::Container(
            std::any::TypeId::of::<Vec<()>>(),
            vec![T::type_check_info()]
        )
    }
}
