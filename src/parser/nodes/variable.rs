use std::{borrow::Cow, iter::Peekable, slice::Iter};

use crate::{
    parser::ClassContext,
    token::{DataType, Token, TokenType},
};

#[derive(Clone)]
pub struct NodeVariable {
    pub name: String,
    pub r#type: VariableType,
    /// If this is not ininitalzed yet this will be None
    ///
    /// # Example
    ///
    /// int i; // Value will be None
    ///
    /// int i = 10; // Value will be Direct
    pub value: Option<VariableValue>,
}

impl NodeVariable {
    pub fn to_code(&self) -> Cow<'static, str> {
        match &self.value {
            Some(val) => {
                let val = val.clone().get_value();
                format!("let {} = {};", self.name, val).into()
            }
            None => format!("let {};", self.name).into(),
        }
    }
}

#[derive(Clone)]
pub enum VariableType {
    DataType(DataType),
    Class,
}

#[derive(Clone)]
pub enum VariableValue {
    /// The value is directly hard-coded
    /// Saves the value
    ///
    /// # Example
    ///
    /// int i = 10;
    Direct(String),
    /// The value is set from an existing variable
    /// Saves the var name
    ///
    /// # Example
    ///
    /// int o = 10;
    /// int i = o;
    ByVar(String),
    /// The value is calculated by an other function
    ///
    /// # Example
    ///
    /// int i = get_int_number();
    /// TODO
    Indirect,
}

impl VariableValue {
    pub fn get_value(self) -> String {
        match self {
            VariableValue::Direct(string) => string,
            // Gives the var name from which the var comes from
            VariableValue::ByVar(string) => string,
            VariableValue::Indirect => todo!(),
        }
    }
}

impl NodeVariable {
    /// By current_context_vars mean that we only have variables which we currently can see
    ///
    /// # Example
    ///
    /// public class Main {
    ///   // These both would be in a context
    ///     private int i;
    ///     private int i;
    ///     public static void main(String[] args) {
    ///   // These both would be in a context, but not the above
    ///         int o = 10;
    ///         int o = 10;
    ///     }
    pub fn parse(
        tokens: &mut Peekable<Iter<Token>>,
        class_context: &ClassContext,
        method_vars: &[NodeVariable],
    ) -> Result<Self, String> {
        let token = match tokens.next() {
            Some(token) => token,
            None => {
                return Err("Unexpected end of input while parsing expression".to_string());
            }
        };

        match &token.token_type {
            // Variable is a data type
            TokenType::DATATYPE(data_type) => Self::parse_variable_declaration(
                tokens,
                method_vars,
                VariableType::DataType(*data_type),
                token,
            ),
            // Variable is a class
            TokenType::UNKNOWN => {
                Self::parse_variable_declaration(tokens, method_vars, VariableType::Class, token)
            }
            _ => Err(format!("{}, Invalid expression: expected Variable", token)),
        }
    }

    fn parse_variable_declaration(
        tokens: &mut Peekable<Iter<Token>>,
        current_context_vars: &[NodeVariable],
        r#type: VariableType,
        name_token: &Token,
    ) -> Result<Self, String> {
        let name = match &name_token.token_type {
            TokenType::UNKNOWN => name_token.value.clone().unwrap(),
            _ => {
                return Err(format!(
                    "{}, Invalid expression: expected variable name",
                    name_token
                ));
            }
        };

        if current_context_vars.iter().any(|var| var.name == name) {
            return Err(format!(
                "{}, Variable with the same name is already defined",
                name_token
            ));
        }

        let next_token = match tokens.next() {
            Some(token) => token,
            None => {
                return Err(format!(
                    "{}, Invalid expression: Variable got no semicolon or value",
                    name_token
                ));
            }
        };

        match next_token.token_type {
            TokenType::SEMICOLON => Ok(Self {
                name,
                r#type,
                value: None,
            }),
            TokenType::EQUAL => {
                return Self::parse_variable_value(
                    tokens,
                    current_context_vars,
                    r#type,
                    name_token,
                    name,
                );
            }
            _ => Err(format!(
                "{}, Invalid expression: Variable got no semicolon or value",
                next_token
            )),
        }
    }

    fn parse_variable_value(
        tokens: &mut Peekable<Iter<Token>>,
        current_context_vars: &[NodeVariable],
        r#type: VariableType,
        name_token: &Token,
        name: String,
    ) -> Result<Self, String> {
        let next_token = match tokens.next() {
            Some(token) => token,
            None => {
                return Err(format!(
                    "{}, Invalid expression: Variable got no value but equal",
                    name_token
                ));
            }
        };

        let mut value = None;
        match next_token.token_type {
            TokenType::INTLIT => {
                value = Some(VariableValue::Direct(
                    next_token.value.as_ref().unwrap().clone(),
                ))
            }
            TokenType::TRUE => value = Some(VariableValue::Direct("true".to_string())),
            TokenType::FALSE => value = Some(VariableValue::Direct("false".to_string())),
            TokenType::UNKNOWN => {
                // First lets check if we set the value to an existing vars value
                // TODO: allow non context vars here
                for var in current_context_vars {
                    if &var.name == next_token.value.as_ref().unwrap() {
                        value = Some(VariableValue::ByVar(name.clone()));
                        break;
                    }
                }
                // TODO: implement indirect
            }

            _ => {
                return Err(format!(
                    "{}, Invalid expression: Variable got no semicolon or value",
                    next_token
                ))
            }
        };

        let next_token = match tokens.next() {
            Some(token) => token,
            None => {
                return Err(format!(
                    "{}, Invalid expression: Variable got no simicolon",
                    name_token
                ));
            }
        };

        match next_token.token_type {
            TokenType::SEMICOLON => Ok(Self {
                name,
                r#type,
                value,
            }),
            _ => Err(format!(
                "{}, Invalid expression: Variable got no semicolon",
                next_token
            )),
        }
    }
}
