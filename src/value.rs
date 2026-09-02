use std::fmt;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Literal(f64)
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Literal(n) => write!(f, "{}", n),
        }
    }
}