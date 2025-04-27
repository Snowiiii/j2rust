use std::{iter::Peekable, slice::Iter};

use crate::{
    parser::{
        nodes::{get_variable_in_method, variable::NodeVariable},
        ClassContext,
    },
    token::{Token, TokenType},
};

pub struct IntExpression {
    pub final_code: String,
    pub end_with_semi: bool,
}

impl IntExpression {
    pub fn parse(
        tokens: &mut Peekable<Iter<Token>>,
        class_context: &ClassContext,
        method_vars: &[NodeVariable],
    ) -> Result<Self, String> {
        let mut final_code = String::new();
        let mut last_token: Option<&Token> = None;
        while let Some(token) = tokens.next() {
            match token.token_type {
                TokenType::ADD => {
                    final_code.push_str(" + ");
                }
                TokenType::REMOVE => {
                    final_code.push_str(" - ");
                }
                TokenType::SUB => {
                    final_code.push_str(" / ");
                }
                TokenType::MUL => {
                    final_code.push_str(" * ");
                }
                TokenType::OPEN_BRACE => {
                    final_code.push_str("(");
                }
                TokenType::CLOSE_BRACE => {
                    final_code.push_str(")");
                }
                TokenType::INTLIT => {
                    let int_val = token.value.clone().unwrap();
                    final_code.push_str(&int_val);
                }
                TokenType::UNKNOWN => {
                    if let Some(last) = last_token {
                        if last.token_type == TokenType::UNKNOWN {
                            return Err(format!(
                                "{}, Invalid Int expression: string after string",
                                token
                            ));
                        }
                    }

                    let name = token.value.clone().unwrap();

                    if let Some((var, in_class)) =
                        get_variable_in_method(name, class_context, method_vars)
                    {
                        if in_class {
                            final_code.push_str(&format!("Self::{}", var.name));
                        } else {
                            final_code.push_str(&var.name);
                        }
                    }
                }
                _ => {
                    if final_code.is_empty() {
                        return Err(format!(
                            "{}, Invalid Int expression: unexpected token",
                            token
                        ));
                    } else {
                        return Ok(Self {
                            final_code,
                            end_with_semi: token.token_type == TokenType::SEMICOLON,
                        });
                    }
                }
            }
            last_token = Some(token);
        }
        return Err("Empty".to_string());
    }
}
