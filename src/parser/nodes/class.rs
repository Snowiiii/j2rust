use std::{iter::Peekable, slice::Iter};

use crate::token::{Token, TokenType};

pub struct NodeClass {
    pub name: String,
}

impl NodeClass {
    pub fn parse(tokens: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        if let Some(token) = tokens.next() {
            match token.token_type {
                TokenType::UNKNOWN => {
                    let name = token.value.clone().unwrap();
                    Ok(Self { name })
                }
                _ => Err(format!(
                    "{}, Invalid expression: expected string literal",
                    token
                )),
            }
        } else {
            Err(String::from(
                "Unexpected end of input while parsing expression",
            ))
        }
    }
}
