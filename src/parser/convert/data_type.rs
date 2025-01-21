use crate::token::DataType;

pub fn convert_data_type(data_type: DataType) -> &'static str {
    match data_type {
        DataType::BYTE => "i8",
        DataType::SHORT => "i16",
        DataType::INT => "i32",
        DataType::LONG => "i64",
        DataType::FLOAT => "f32",
        DataType::DOUBLE => "f64",
        DataType::CHAR => "char",
        DataType::BOOLEAN => "bool",
    }
}
