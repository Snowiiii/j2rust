use std::borrow::Cow;

use crate::{
    parser::nodes::{FunctionArgument, MethodArgumentType, MethodReturnType},
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
        args: Vec<FunctionArgument>,
    ) -> Option<Cow<'static, str>> {
        match name {
            "println" => match &args[0] {
                FunctionArgument::STRING(s) => Some(format!("println!(\"{}\");", s).into()),
                FunctionArgument::VARIABLE((r#type, var)) => {
                    // because of regex
                    let mut final_var = "{".to_string();
                    final_var.push_str(var);
                    final_var.push('}');
                    Some(format!("println!({});", final_var).into())
                }
                _ => todo!(),
            },
            _ => None,
        }
    }
}
