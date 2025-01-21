use nodes::{class::NodeClass, method::NodeMethod};

use crate::token::{Token, TokenType, Visibility};
pub mod convert;
pub mod nodes;

pub enum Node {
    Class(NodeClass),
    Method(NodeMethod),
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Vec<Node>, String> {
    let mut tokens = tokens.iter().peekable();
    let mut nodes = Vec::new();

    let current_class = String::new();

    let mut is_static = false;
    let mut current_visibility = Visibility::NONE;

    while let Some(token) = tokens.next() {
        let token_type = &token.token_type;
        match token_type {
            TokenType::RETURN => todo!(), // Handle return statement (optional)
            TokenType::INTLIT => todo!(), // Handle integer literals outside of exit (optional)
            TokenType::SEMICOLON => {}    // Currently ignored, consider handling semicolons
            TokenType::DATATYPE(d) => todo!(),
            TokenType::CLASS => {
                let class = NodeClass::parse(&mut tokens)?;
                nodes.push(Node::Class(class));
            }
            TokenType::VOID => {
                let mut method = NodeMethod::parse(
                    &mut tokens,
                    current_visibility.clone(),
                    is_static,
                    nodes::MethodReturnType::VOID,
                )
                .unwrap();
                // TODO
                let code = method.parse_body(&mut tokens).unwrap();
                method.code = code;
                nodes.push(Node::Method(method));
            }
            TokenType::VISIBILITY(visibility) => current_visibility = visibility.to_owned(),
            TokenType::STATIC => {
                is_static = true;
            }
            _ => {}
        }
    }

    Ok(nodes) // Parsing successful
}
