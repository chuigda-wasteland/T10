pub mod base;
pub mod fusion;

#[derive(Debug)]
pub enum TypeCheckInfo {
    SimpleType(std::any::TypeId),
    Container(std::any::TypeId, Vec<TypeCheckInfo>),
}

#[derive(Debug)]
pub enum FFIAction {
    Move,
    Copy,
    Share,
    MutShare
}
