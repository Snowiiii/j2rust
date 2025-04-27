use std::{borrow::Cow, iter::Peekable, slice::Iter};

use crate::{
    parser::{convert::method::convert_method, ClassContext},
    prelude::get_prelude_class,
    token::{Token, TokenType, Visibility},
};

use super::{variable::NodeVariable, FunctionArgument, MethodArgumentType, MethodReturnType};

#[derive(Default)]
pub struct NodeMethod {
    pub visibility: Visibility,
    pub return_type: MethodReturnType,
    pub r#static: bool,
    pub name: String,
    pub args: Vec<MethodArgument>,
    pub code: Vec<Cow<'static, str>>,
}

pub struct MethodArgument {
    pub arg: MethodArgumentType,
    pub array: bool,
    pub name: String,
}

impl NodeMethod {
    pub fn parse(
        tokens: &mut Peekable<Iter<Token>>,
        class_context: &ClassContext,
        visibility: Visibility,
        is_static: bool,
        return_type: MethodReturnType,
    ) -> Result<Self, String> {
        let name = Self::parse_name(tokens)?;
        let args = Self::parse_arguments(tokens)?;
        let code = Self::parse_body(&args, class_context, tokens)?;
        return Ok(Self {
            visibility,
            return_type,
            r#static: is_static,
            name,
            args,
            code,
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

    pub fn parse_arguments(
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
        args: &Vec<MethodArgument>,
        class_context: &ClassContext,
        tokens: &mut Peekable<Iter<Token>>,
    ) -> Result<Vec<Cow<'static, str>>, String> {
        let mut final_code = vec![];
        if let Some(token) = tokens.next() {
            if token.token_type == TokenType::OPEN_BRACKET {
                // here the magic happens
                // this is for saftly the max method length
                //
                let mut current_vars = vec![];
                let mut current_if_level = 0;
                for _ in 0..i32::MAX {
                    let mut cloned_tokens = tokens.clone();
                    if let Ok(var) =
                        NodeVariable::parse(&mut cloned_tokens, class_context, &current_vars)
                    {
                        final_code.push(var.to_code());
                        current_vars.push(var);
                        *tokens = cloned_tokens;
                        continue;
                    }

                    if let Some(token) = tokens.next() {
                        if token.token_type == TokenType::IF {
                            current_if_level += 1;
                        } else if token.token_type == TokenType::ELSE {
                        } else if token.token_type == TokenType::UNKNOWN {
                            // So this is something unknown, this either can be a Class or a Var, TODO: Check for vars
                            let class_name = token.value.as_ref().unwrap();

                            // TODO: Don't only support prelude classes but also which are included using import
                            dbg!(class_name);
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
                                                        return Err(
                                                            "Failed to get Class field".to_string()
                                                        )
                                                    }
                                                };
                                                class = field;
                                            }
                                        }

                                        // lets continue searching, we also may use multiple fields
                                    } else if token.token_type == TokenType::OPEN_BRACE {
                                        // we want to call a method (e.g. System.println())
                                        // lets parse the arguments
                                        let args =
                                            Self::parse_function_arguments(tokens, &current_vars)?;
                                        let code = match class.code_from_method(&method_name, args)
                                        {
                                            Some(code) => code,
                                            None => {
                                                return Err(format!(
                                                    "Failed to get method {}",
                                                    method_name
                                                ))
                                            }
                                        };
                                        final_code.push(code);
                                    } else if token.token_type == TokenType::SEMICOLON {
                                        break;
                                    }
                                }
                            }
                        } else if token.token_type == TokenType::CLOSE_BRACKET {
                            if current_if_level > 0 {
                                current_if_level + -1;
                            } else {
                                // end of the method
                                break;
                            }
                        }
                    }
                }
            }
        }
        Ok(final_code)
    }

    fn parse_function_arguments(
        tokens: &mut Peekable<Iter<Token>>,
        variables: &Vec<NodeVariable>,
    ) -> Result<Vec<FunctionArgument>, String> {
        let mut args = vec![];
        for _ in 0..i32::MAX {
            if let Some(token) = tokens.next() {
                if token.token_type == TokenType::UNKNOWN {
                    for var in variables {
                        if &var.name == token.value.as_ref().unwrap() {
                            if let Some(_val) = &var.value {
                                args.push(FunctionArgument::VARIABLE((
                                    var.r#type.clone(),
                                    var.name.clone(),
                                )));
                            } else {
                                return Err(format!("Variable {} is uninitialized", var.name));
                            }
                            break;
                        }
                    }
                } else if token.token_type == TokenType::QUOTE {
                    // we have a direct string
                    let mut words = String::new();
                    for _ in 0..i16::MAX {
                        if let Some(token) = tokens.next() {
                            if token.token_type == TokenType::QUOTE_STRING {
                                words.push_str(&token.value.clone().unwrap());
                            } else if token.token_type == TokenType::QUOTE {
                                args.push(FunctionArgument::STRING(words));
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

    pub fn get_full_code(&self) -> Cow<'static, str> {
        let header = convert_method(self);
        let code_lines = &self.code;
        let mut final_code = header.to_string();
        final_code.push_str("{\n");
        for line in code_lines {
            final_code.push_str(&format!("{}\n", line));
        }
        final_code.push_str("}");

        final_code.into()
    }
}
