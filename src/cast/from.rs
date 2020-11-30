use crate::data::Value;

pub trait IntoValue<'a> {
    unsafe fn into_value(self) -> Result<Value<'a>, String>;
}

pub trait IntoValueNoexcept<'a> {
    unsafe fn into_value_noexcept(self) -> Result<Value<'a>, String>;
}

pub trait ValueFromRustL1<'a> {
    unsafe fn into_value1(self) -> Result<Value<'a>, String>;
}

pub trait ValueFromRustL2<'a> {
    unsafe fn into_value2(self) -> Result<Value<'a>, String>;
}

pub trait ValueFromRustL3<'a> {
    unsafe fn into_value3(self) -> Result<Value<'a>, String>;
}

impl<'a, T: IntoValueNoexcept<'a>> IntoValue<'a> for T {
    unsafe fn into_value(self) -> Result<Value<'a>, String> {
        self.into_value_noexcept()
    }
}

impl<'a, T: IntoValueNoexcept<'a>, E: 'static + std::error::Error> IntoValue<'a> for Result<T, E> {
    unsafe fn into_value(self) -> Result<Value<'a>, String> {
        match self {
            Ok(value) => value.into_value_noexcept(),
            Err(e) => Err(format!("{}", e))
        }
    }
}
