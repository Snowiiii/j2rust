// use std::{iter::Peekable, slice::Iter};

// use crate::{
//     parser::nodes::variable::NodeVariable,
//     token::{Token, TokenType},
// };

// pub struct BoolExpression {
//     final_code: String,
// }

// impl BoolExpression {
//     pub fn parse(
//         tokens: &mut Peekable<Iter<Token>>,
//         current_context_vars: &[NodeVariable],
//     ) -> Result<Self, String> {
//         if let Some(token) = tokens.next() {
//             let string = String::new();
//             match token.token_type {
//                 TokenType::TRUE => {
//                     let name = token.value.clone().unwrap();
//                     Ok(Self { name })
//                 }
//                 TokenType::FALSE => Ok(Self { final_code }),
//                 _ => Err(format!(
//                     "{}, Invalid expression: expected string literal",
//                     token
//                 )),
//             }
//         } else {
//             Err(String::from(
//                 "Unexpected end of input while parsing expression",
//             ))
//         }
//     }
// }
