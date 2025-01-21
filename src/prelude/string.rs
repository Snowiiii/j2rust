use crate::parser::nodes::MethodArgumentType;

use super::Class;

pub struct String;

impl Class for String {
    fn get_field(&self, name: &str) -> Option<Box<dyn Class>> {
        todo!()
    }

    fn code_from_method(
        &self,
        name: &str,
        args: Vec<MethodArgumentType>,
    ) -> Option<std::borrow::Cow<'static, str>> {
        todo!()
    }
}
