//! `error` 模块中定义了错误处理工具类

use std::any::TypeId;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::data::GcInfo;
use crate::tyck::FFIAction;

/// T10 中所使用的错误处理类型
#[derive(Debug)]
pub enum TError {
    /// （运行时）生存期检查错误
    LifetimeError(LifetimeError),
    /// 类型错误
    TypeError(TypeError),
    /// 空指针/空值错误
    NullError(NullError),
    /// 非受检异常
    UncheckedException(String),
    /// 用户定义的受检异常
    UserException(Box<dyn 'static + Error>)
}

impl TError {
    pub fn unchecked_exception(info: impl ToString) -> Self {
        Self::UncheckedException(info.to_string())
    }

    pub fn user_exception(exception: impl 'static + Error) -> Self {
        Self::UserException(Box::new(exception))
    }
}

impl From<LifetimeError> for TError {
    fn from(e: LifetimeError) -> Self {
        Self::LifetimeError(e)
    }
}

impl From<TypeError> for TError {
    fn from(e: TypeError) -> Self {
        Self::TypeError(e)
    }
}

impl From<NullError> for TError {
    fn from(e: NullError) -> Self {
        Self::NullError(e)
    }
}

impl Display for TError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TError::LifetimeError(e) => write!(f, "{}", e),
            TError::TypeError(e) => write!(f, "{}", e),
            TError::NullError(e) => write!(f, "{}", e),
            TError::UncheckedException(e) => write!(f, "{}", e),
            TError::UserException(e) => write!(f, "{}", e)
        }
    }
}

impl Error for TError {}

#[derive(Debug)]
pub struct LifetimeError {
    pub required: &'static [GcInfo],
    pub action: FFIAction,
    pub actual: GcInfo,
    pub extra_info: Option<&'static str>
}

impl LifetimeError {
    pub fn new(required: &'static [GcInfo],
               action: FFIAction,
               actual: GcInfo) -> Self {
        Self {
            required,
            action,
            actual,
            extra_info: None
        }
    }

    pub fn add_extra_info(self, extra_info: &'static str) -> Self {
        let mut ret = self;
        ret.extra_info.replace(extra_info);
        ret
    }
}

impl Display for LifetimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LifetimeError: when performing {:?}: expected {:?}, got {:?}",
               self.action, self.required, self.actual)?;
        if let Some(extra_info) = self.extra_info {
            write!(f, ": \"{}\"", extra_info)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct TypeError {
    pub required: TypeId,
    pub actual: TypeId,
    pub required_name: Option<String>,
    pub actual_name: Option<String>,
    pub extra_info: Option<&'static str>
}

impl TypeError {
    pub fn new(required: TypeId, got: TypeId) -> Self {
        Self {
            required,
            actual: got,
            required_name: None,
            actual_name: None,
            extra_info: None
        }
    }

    pub fn add_required_name(self, required_name: String) -> Self {
        let mut ret = self;
        ret.required_name.replace(required_name);
        ret
    }

    pub fn add_actual_name(self, actual_name: String) -> Self {
        let mut ret = self;
        ret.actual_name.replace(actual_name);
        ret
    }

    pub fn add_extra_info(self, extra_info: &'static str) -> Self {
        let mut ret = self;
        ret.extra_info.replace(extra_info);
        ret
    }
}

impl Display for TypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeError: ")?;
        if let Some(required_name) = &self.required_name {
            write!(f, "expected type {}({:?}), ", required_name, self.required)?;
        } else {
            write!(f, "expected type <unknown>({:?}), ", self.required)?;
        }
        if let Some(actual_name) = &self.actual_name {
            write!(f, "got {}({:?})", actual_name, self.actual)?;
        } else {
            write!(f, "got <unknown>({:?})", self.actual)?;
        }
        if let Some(extra_info) = self.extra_info {
            write!(f, ": {}", extra_info)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct NullError ();

impl Display for NullError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NullError")
    }
}
