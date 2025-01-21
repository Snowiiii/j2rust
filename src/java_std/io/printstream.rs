use std::borrow::Cow;

use crate::{
    parser::nodes::{MethodArgumentType, MethodReturnType},
    prelude::Class,
};

pub struct PrintStream;

impl Class for PrintStream {
    fn get_field(&self, name: &str) -> Option<Box<dyn Class>> {
        todo!()
    }

    fn code_from_method(
        &self,
        name: &str,
        args: Vec<MethodArgumentType>,
    ) -> Option<Cow<'static, str>> {
        match name {
            "println" => {
                let arg = match &args[0] {
                    MethodArgumentType::STRING(s) => s,
                    MethodArgumentType::DATATYPE(data_type) => todo!(),
                    MethodArgumentType::CLASS(class) => todo!(),
                };
                Some(format!("println!(\"{}\")", arg).into())
            }
            _ => None,
        }
    }
}
