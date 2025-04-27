use std::{iter::Peekable, slice::Iter};

use crate::token::{Token, TokenType};

pub struct IntExpression {
    final_code: String,
}

impl IntExpression {
    pub fn parse(
        tokens: &mut Peekable<Iter<Token>>,
        current_context_vars: &[NodeVariable],
    ) -> Result<Self, String> {
        if let Some(token) = tokens.next() {
            match token.token_type {
                TokenType::ADD => {
                    let name = token.value.clone().unwrap();
                    Ok(Self { name })
                }
                TokenType::REMOVE => {
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
