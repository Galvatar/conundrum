use std::fmt;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f64),
    String(String),
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Number(n) => write!(f, "{}", n),
            RuntimeValue::String(s) => write!(f, "{}", s),
        }
    }
}