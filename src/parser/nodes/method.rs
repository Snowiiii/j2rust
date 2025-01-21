use std::{borrow::Cow, iter::Peekable, slice::Iter};

use crate::{
    prelude::get_prelude_class,
    token::{Token, TokenType, Visibility},
};

use super::{MethodArgumentType, MethodReturnType};

#[derive(Default)]
pub struct NodeMethod {
    pub visibility: Visibility,
    pub return_type: MethodReturnType,
    pub r#static: bool,
    pub name: String,
    pub args: Vec<MethodArgument>,
}

pub struct MethodArgument {
    pub arg: MethodArgumentType,
    pub array: bool,
    pub name: String,
}

impl NodeMethod {
    pub fn parse(
        tokens: &mut Peekable<Iter<Token>>,
        visibility: Visibility,
        is_static: bool,
        return_type: MethodReturnType,
    ) -> Result<Self, String> {
        let name = Self::parse_name(tokens)?;
        let args = Self::parse_arugments(tokens)?;
        return Ok(Self {
            visibility,
            return_type,
            r#static: is_static,
            name,
            args,
        });
    }

    pub fn parse_name(tokens: &mut Peekable<Iter<Token>>) -> Result<String, String> {
        if let Some(token) = tokens.next() {
            if token.token_type == TokenType::UNKNOWN {
                return Ok(token.value.clone().unwrap());
            }
        }
        Err("Failed to get method name".to_string())
    }

    pub fn parse_arugments(
        tokens: &mut Peekable<Iter<Token>>,
    ) -> Result<Vec<MethodArgument>, String> {
        if let Some(token) = tokens.next() {
            if token.token_type == TokenType::OPEN_BRACE {
                let mut args = vec![];
                // parse arguments
                for _ in 0..i32::MAX {
                    if let Some(token) = tokens.next() {
                        match &token.token_type {
                            TokenType::CLOSE_BRACE => {
                                // if we have a close brace we are done here :D
                                break;
                            }
                            TokenType::DATATYPE(r#type) => {
                                match Self::parse_argument(
                                    tokens,
                                    MethodArgumentType::DATATYPE(r#type.clone()),
                                ) {
                                    Ok(arg) => args.push(arg),
                                    Err(error) => return Err(error),
                                }
                            }
                            TokenType::UNKNOWN => {
                                let class_name = token.value.as_ref().unwrap();
                                // TODO: Don't only support prelude classes but also which are included using import
                                let class = get_prelude_class(class_name).expect("Invalid class");
                                match Self::parse_argument(tokens, MethodArgumentType::CLASS(class))
                                {
                                    Ok(arg) => args.push(arg),
                                    Err(error) => return Err(error),
                                }
                            }
                            _ => {}
                        }
                    } else {
                        return Err(String::from(
                            "Invalid method: Unexpected end of input while parsing expression",
                        ));
                    }
                }
                return Ok(args);
            }
        }
        Err(String::from("Invalid method: Expected bracket"))
    }

    pub fn parse_argument(
        tokens: &mut Peekable<Iter<Token>>,
        typee: MethodArgumentType,
    ) -> Result<MethodArgument, String> {
        // lets check if this is an array or just use the name
        if let Some(token) = tokens.next() {
            match token.token_type {
                TokenType::UNKNOWN => Ok(MethodArgument {
                    name: token.value.clone().unwrap(),
                    arg: typee,
                    array: false,
                }),
                TokenType::ARRAY => {
                    // great we know its an array, now lets get the name
                    if let Some(token) = tokens.next() {
                        if token.token_type == TokenType::UNKNOWN {
                            // yay thats the name
                            Ok(MethodArgument {
                                name: token.value.clone().unwrap(),
                                arg: typee,
                                array: true,
                            })
                        } else {
                            Err(String::from(
                                "Invalid method: Got a array for an argument but now no name",
                            ))
                        }
                    } else {
                        Err(String::from(
                            "Invalid method: Got a array for an argument but now no name",
                        ))
                    }
                }
                _ => Err(String::from(
                    "Invalid method: Expected argument name or array but got something different",
                )),
            }
        } else {
            Err(String::from(
                "Invalid method: Expected argument name or array but got nothing",
            ))
        }
    }

    pub fn parse_body(
        &self,
        tokens: &mut Peekable<Iter<Token>>,
    ) -> Result<Vec<Cow<'static, str>>, String> {
        let mut final_code = vec![];
        if let Some(token) = tokens.next() {
            if token.token_type == TokenType::OPEN_BRACKET {
                // here the magic happens
                // this is for saftly the max method length
                for _ in 0..i32::MAX {
                    if let Some(token) = tokens.next() {
                        if token.token_type == TokenType::UNKNOWN {
                            // So this is something unknow, this either can be a Class or a Var, TODO: Check for vars
                            let class_name = token.value.as_ref().unwrap();
                            // TODO: Don't only support prelude classes but also which are included using import
                            let mut class = get_prelude_class(class_name).expect("Invalid class");
                            let mut method_name = String::new();
                            for _ in 0..i32::MAX {
                                if let Some(token) = tokens.next() {
                                    if token.token_type == TokenType::UNKNOWN {
                                        // we are using field from method (e.g. System.out...)
                                        let name = token.value.clone().unwrap();
                                        
                                        if let Some(token) = tokens.peek() {
                                            // this is the final function and not a field
                                            if token.token_type == TokenType::OPEN_BRACE {
                                                method_name = name;
                                            } else {
                                                let field = match class.get_field(&name) {
                                                    Some(field) => field,
                                                    None => {
                                                        return Err("Failed to get Class field".to_string())
                                                    }
                                                };
                                                class = field;
                                            }
                                        }
                                        
                                        // lets contuine searching, we also may use multiple fields
                                    } else if token.token_type == TokenType::OPEN_BRACE {
                                        // we want to call a method (e.g. System.println())
                                        // lets parse the arguments
                                        let args = Self::parse_body_arguments(tokens)?;
                                        let code = match class.code_from_method(&method_name, args) {
                                            Some(code) => code,
                                            None => return Err(format!("Failed to get method {}", method_name)),
                                        };
                                        final_code.push(code);
                                    } else if token.token_type == TokenType::SEMICOLON {
                                        break;
                                    }
                                }
                            }
                        } else if token.token_type == TokenType::CLOSE_BRACKET {
                            // end of the method
                            break;
                        }
                    }
                }
            }
        }
        Ok(final_code)
    }

    fn parse_body_arguments(
        tokens: &mut Peekable<Iter<Token>>,
    ) -> Result<Vec<MethodArgumentType>, String> {
        let mut args = vec![];
        for _ in 0..i32::MAX {
            if let Some(token) = tokens.next() {
                if token.token_type == TokenType::UNKNOWN {
                    // todo
                } else if token.token_type == TokenType::QUOTE {
                    // we have a direct string
                    let mut words = String::new();
                    for _ in 0..i16::MAX {
                        if let Some(token) = tokens.next() {
                            if token.token_type == TokenType::UNKNOWN {
                                words.push_str(&token.value.clone().unwrap());
                            } else if token.token_type == TokenType::QUOTE {
                                args.push(MethodArgumentType::STRING(words));
                                break;
                            }
                        }
                    }
                } else if token.token_type == TokenType::COMMA {
                    // TODO: next arg
                } else if token.token_type == TokenType::CLOSE_BRACE {
                    return Ok(args);
                }
            }
        }
        Ok(args)
    }
}
