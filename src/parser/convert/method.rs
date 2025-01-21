use std::borrow::Cow;

use crate::parser::nodes::method::NodeMethod;

use super::data_type::convert_data_type;

pub fn convert_method(method: NodeMethod) -> Cow<'static, str> {
    let visibility = match method.visibility {
        crate::token::Visibility::PUBLIC => "pub",
        crate::token::Visibility::NONE => "",
        crate::token::Visibility::PRIVATE => "",
    };
    let return_type = match method.return_type {
        crate::parser::nodes::MethodReturnType::VOID => "",
        crate::parser::nodes::MethodReturnType::DATATYPE(data_type) => &format!("-> {}",  convert_data_type(data_type)),
        crate::parser::nodes::MethodReturnType::CLASS(class) => todo!(),
    };
    let name = method.name;
    // TODO: args
    format!("{visibility} fn {name} {return_type}").into()
}
