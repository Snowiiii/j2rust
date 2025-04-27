use std::str;

use nodes::{class::NodeClass, method::NodeMethod, variable::NodeVariable};

use crate::token::{Token, TokenType, Visibility};
pub mod convert;
pub mod nodes;

pub enum Node {
    Class(NodeClass),
    Method(NodeMethod),
}

#[derive(Default)]
pub struct ClassContext {
    variables: Vec<ClassVariable>,
    methods: Vec<NodeMethod>,
}

pub struct ClassVariable {
    visibility: Visibility,
    is_static: bool,
    variable: NodeVariable,
}


/// Usally parses all tokens of one file
pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Vec<Node>, String> {
    let mut tokens = tokens.iter().peekable();
    let mut nodes = Vec::new();

    let mut class_context = ClassContext::default();

    let mut is_static = false;
    let mut current_visibility = Visibility::NONE;

    while let Some(token) = tokens.next() {
        if let Ok(variable) = NodeVariable::parse(&mut tokens, &class_context, &[]) {
            class_context
                .variables
                .push(ClassVariable { visibility: current_visibility.clone(), is_static, variable });
        }

        let token_type = &token.token_type;
        match token_type {
            TokenType::RETURN => todo!(), // Handle return statement (optional)
            TokenType::INTLIT => todo!(), // Handle integer literals outside of exit (optional)
            TokenType::SEMICOLON => {}    // Currently ignored, consider handling semicolons
            TokenType::DATATYPE(datatyp) => {
                let method = NodeMethod::parse(
                    &mut tokens,
                    &class_context,
                    current_visibility.clone(),
                    is_static,
                    nodes::MethodReturnType::DATATYPE(*datatyp),
                )
                .unwrap();
                nodes.push(Node::Method(method));
            }
            TokenType::CLASS => {
                let class = NodeClass::parse(&mut tokens)?;
                nodes.push(Node::Class(class));
            }
            TokenType::VOID => {
                let method = NodeMethod::parse(
                    &mut tokens,
                    &class_context,
                    current_visibility.clone(),
                    is_static,
                    nodes::MethodReturnType::VOID,
                )
                .unwrap();
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
