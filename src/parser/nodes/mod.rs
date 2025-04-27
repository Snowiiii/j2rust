use variable::{NodeVariable, VariableType};

use crate::{prelude::Class, token::DataType};

pub mod class;
pub mod expr;
pub mod method;
pub mod variable;

/// System.out.println(-> ...)
pub enum FunctionArgument {
    /// Value
    STRING(String),
    /// Type, Value
    DATATYPE((DataType, String)),
    /// Type, Var Name
    VARIABLE((VariableType, String)),
    /// Value
    CLASS(Box<dyn Class>),
}

/// void myMethod(int x) {
pub enum MethodArgumentType {
    STRING,
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
