#[derive(Debug, PartialEq)]
pub enum TokenType {
    RETURN,
    VOID,
    CLASS,
    IF,
    OR,
    AND,
    ELSE,
    INTLIT,
    ARRAY,
    STATIC,
    QUOTE,
    QUOTE_STRING,
    /// (
    OPEN_BRACE,
    /// )
    CLOSE_BRACE,
    /// {
    OPEN_BRACKET,
    /// }
    CLOSE_BRACKET,
    COMMA,
    TRUE,
    FALSE,
    VISIBILITY(Visibility),
    DATATYPE(DataType),
    EQUAL,
    ADD,
    MUL,
    SUB,
    REMOVE,
    SEMICOLON,
    UNKNOWN,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub enum Visibility {
    PUBLIC,
    #[default]
    NONE,
    PRIVATE,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataType {
    BYTE,
    SHORT,
    INT,
    LONG,
    FLOAT,
    DOUBLE,
    CHAR,
    BOOLEAN,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub char_info: CharLocationInfo,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}:{} in {}:{}:{}",
            self.token_type,
            self.value.as_ref().unwrap_or(&"None".to_string()),
            self.char_info.file_path,
            self.char_info.line_number,
            self.char_info.line_col
        )
    }
}

#[derive(Debug, Clone)]
pub struct CharLocationInfo {
    pub file_path: String,
    pub line_number: usize,
    pub line_col: u32,
}
