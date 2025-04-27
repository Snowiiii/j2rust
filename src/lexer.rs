use std::path::Path;

use crate::token::{CharLocationInfo, DataType, Token};

pub fn read_file(file: &Path) -> Vec<Token> {
    let path = file.display().to_string();
    let content = std::fs::read_to_string(file).expect("Failed to read file");
    let mut final_vec = Vec::new();
    for (line_number, line) in content.lines().enumerate() {
        final_vec.append(&mut tokennize_line(line.to_owned(), line_number, &path))
    }
    final_vec
}

// We tokennize line by line so we can do better error handling/better compiler errors
pub fn tokennize_line(string: String, line_number: usize, file: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = string.chars().peekable();

    let mut line_col = 0;
    let mut is_quote_message = false;
    while let Some(c) = chars.next() {
        line_col += 1;
        let char_info = CharLocationInfo {
            file_path: file.clone(),
            line_number: line_number + 1, // We want to start from line 1 and not 0
            line_col,
        };
        if c == '"' {
            is_quote_message = !is_quote_message;

            tokens.push(Token {
                token_type: crate::token::TokenType::QUOTE,
                value: None,
                char_info,
            });
        } else if is_quote_message {
            let mut ident = String::new();
            ident.push(c);
            while let Some(next) = chars.peek() {
                if next.is_alphanumeric() {
                    ident.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            tokens.push(Token {
                token_type: crate::token::TokenType::QUOTE_STRING,
                value: Some(ident),
                char_info,
            });
        } else if c.is_alphabetic() {
            let mut ident = String::new();
            ident.push(c);
            while let Some(next) = chars.peek() {
                if next.is_alphanumeric() {
                    ident.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            tokens.push(parse_token(&ident, char_info));
        } else if c.is_numeric() {
            let mut num = String::new();
            num.push(c);
            while let Some(next) = chars.peek() {
                if next.is_numeric() {
                    num.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            //   let int_value = num.parse::<i32>().unwrap();
            tokens.push(Token {
                token_type: crate::token::TokenType::INTLIT,
                value: Some(num),
                char_info,
            });
        } else if c == '(' {
            tokens.push(Token {
                token_type: crate::token::TokenType::OPEN_BRACE,
                value: None,
                char_info,
            });
        } else if c == ')' {
            tokens.push(Token {
                token_type: crate::token::TokenType::CLOSE_BRACE,
                value: None,
                char_info,
            });
        } else if c == '{' {
            tokens.push(Token {
                token_type: crate::token::TokenType::OPEN_BRACKET,
                value: None,
                char_info,
            });
        } else if c == '}' {
            tokens.push(Token {
                token_type: crate::token::TokenType::CLOSE_BRACKET,
                value: None,
                char_info,
            });
        } else if c == ',' {
            tokens.push(Token {
                token_type: crate::token::TokenType::COMMA,
                value: None,
                char_info,
            });
        } else if string == "[]" {
            // TODO fix array
            tokens.push(Token {
                token_type: crate::token::TokenType::ARRAY,
                value: None,
                char_info,
            });
        } else if c == ';' {
            tokens.push(Token {
                token_type: crate::token::TokenType::SEMICOLON,
                value: None,
                char_info,
            })
        } else if c.is_whitespace() {
            continue;
        } else if let Some(token) = parse_expr(c, char_info) {
            tokens.push(token)
        } else {
            println!("Unexpected char")
        }
    }
    tokens
}

pub fn parse_token(string: &str, char_info: CharLocationInfo) -> Token {
    if let Some(data_type) = parse_data_types(string, char_info.clone()) {
        return data_type;
    } else if let Some(visibility) = parse_visibility(string, char_info.clone()) {
        return visibility;
    } else if string == "return" {
        return Token {
            token_type: crate::token::TokenType::RETURN,
            value: None,
            char_info,
        };
    } else if string == "if" {
        return Token {
            token_type: crate::token::TokenType::IF,
            value: None,
            char_info,
        };
    } else if string == "else" {
        return Token {
            token_type: crate::token::TokenType::ELSE,
            value: None,
            char_info,
        };
    } else if string == "||" {
        return Token {
            token_type: crate::token::TokenType::OR,
            value: None,
            char_info,
        };
    } else if string == "&&" {
        return Token {
            token_type: crate::token::TokenType::AND,
            value: None,
            char_info,
        };
    } else if string == "class" {
        return Token {
            token_type: crate::token::TokenType::CLASS,
            value: None,
            char_info,
        };
    } else if string == "void" {
        return Token {
            token_type: crate::token::TokenType::VOID,
            value: None,
            char_info,
        };
    } else if string == "true" {
        return Token {
            token_type: crate::token::TokenType::TRUE,
            value: None,
            char_info,
        };
    } else if string == "false" {
        return Token {
            token_type: crate::token::TokenType::FALSE,
            value: None,
            char_info,
        };
    }
    Token {
        token_type: crate::token::TokenType::UNKNOWN,
        value: Some(string.to_string()),
        char_info,
    }
}

pub fn parse_visibility(string: &str, char_info: CharLocationInfo) -> Option<Token> {
    match string {
        "public" => Some(Token {
            token_type: crate::token::TokenType::VISIBILITY(crate::token::Visibility::PUBLIC),
            value: None,
            char_info,
        }),
        "static" => Some(Token {
            token_type: crate::token::TokenType::STATIC,
            value: None,
            char_info,
        }),
        "private" => Some(Token {
            token_type: crate::token::TokenType::VISIBILITY(crate::token::Visibility::PRIVATE),
            value: None,
            char_info,
        }),
        _ => None,
    }
}

pub fn parse_data_types(string: &str, char_info: CharLocationInfo) -> Option<Token> {
    match string {
        "boolean" => Some(Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::BOOLEAN),
            value: None,
            char_info,
        }),
        "byte" => Some(Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::BYTE),
            value: None,
            char_info,
        }),
        "short" => Some(Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::SHORT),
            value: None,
            char_info,
        }),
        "int" => Some(Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::INT),
            value: None,
            char_info,
        }),
        "long" => Some(Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::LONG),
            value: None,
            char_info,
        }),
        "float" => Some(Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::FLOAT),
            value: None,
            char_info,
        }),
        "double" => Some(Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::DOUBLE),
            value: None,
            char_info,
        }),
        "char" => Some(Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::CHAR),
            value: None,
            char_info,
        }),
        _ => None,
    }
}

pub fn parse_expr(string: char, char_info: CharLocationInfo) -> Option<Token> {
    match string {
        '=' => Some(Token {
            token_type: crate::token::TokenType::EQUAL,
            value: None,
            char_info,
        }),
        '-' => Some(Token {
            token_type: crate::token::TokenType::REMOVE,
            value: None,
            char_info,
        }),
        '+' => Some(Token {
            token_type: crate::token::TokenType::ADD,
            value: None,
            char_info,
        }),
        _ => None,
    }
}
