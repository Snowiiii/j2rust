// java.lang::System

use std::borrow::Cow;

use crate::{java_std::io::printstream::PrintStream, parser::nodes::MethodArgumentType};

use super::Class;

pub struct System {
    out: PrintStream,
}

impl System {
    pub fn new() -> Self {
        Self { out: PrintStream }
    }
}

impl Class for System {
    fn get_field(&self, name: &str) -> Option<Box<dyn Class>> {
        match name {
            "out" => Some(Box::new(PrintStream)),
            _ => None,
        }
    }

    fn code_from_method(&self, name: &str, args: Vec<MethodArgumentType>) -> Option<Cow<'static, str>> {
        todo!()
    }
}
