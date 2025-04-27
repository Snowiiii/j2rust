use variable::{NodeVariable, VariableType};

use crate::{prelude::Class, token::DataType};

use super::ClassContext;

pub mod class;
pub mod expr;
pub mod method;
pub mod variable;

pub fn get_variable_in_method(
    var_name: String,
    class_context: &ClassContext,
    method_vars: &[NodeVariable],
) -> Option<(NodeVariable, bool)> {
    // First search class variables
    for var in &class_context.variables {
        if var.variable.name == var_name {
            return Some((var.variable.clone(), true));
        }
    }
    for var in method_vars {
        if var.name == var_name {
            return Some((var.clone(), false));
        }
    }
    None
}

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
