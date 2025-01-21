use crate::{prelude::Class, token::DataType};

pub mod class;
pub mod int;
pub mod method;

pub enum MethodArgumentType {
    STRING(String),
    DATATYPE(DataType),
    CLASS(Box<dyn Class>),
}

#[derive(Default)]
pub enum MethodReturnType {
    #[default]
    VOID,
    DATATYPE(DataType),
    CLASS(Box<dyn Class>),
}
