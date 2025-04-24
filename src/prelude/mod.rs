use std::borrow::Cow;

use system::System;

use crate::parser::nodes::FunctionArgument;

mod string;
mod system;

pub trait Class {
    fn get_field(&self, name: &str) -> Option<Box<dyn Class>>;

    fn code_from_method(
        &self,
        name: &str,
        args: Vec<FunctionArgument>,
    ) -> Option<Cow<'static, str>>;
}

pub fn get_prelude_class(input: &str) -> Option<Box<dyn Class>> {
    match input {
        "String" => Some(Box::new(string::String)),
        "System" => Some(Box::new(System::new())),
        _ => None,
    }
}
