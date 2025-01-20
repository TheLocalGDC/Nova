use super::{Literal, RValue};

pub trait NameGenerator {
    fn generate_name(&self, rvalue: &RValue, identifier: usize) -> Option<String>;
}

pub struct DefaultNameGenerator {}

impl NameGenerator for DefaultNameGenerator {
    fn generate_name(&self, rvalue: &RValue, identifier: usize) -> Option<String> {
        let hint = match rvalue {
            RValue::Global(_) | RValue::Index(_) | RValue::Literal(Literal::String(_)) => {
                Some(format!("v{}", identifier % 10)) 
            }
            _ => None,
        };

        hint
    }
}
