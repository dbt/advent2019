use std::error;
use std::fmt;


#[derive(Debug,Clone)]
pub struct InvalidOpcode {
    opcode: i32
}

impl InvalidOpcode {
    pub fn new(opcode: i32) -> InvalidOpcode {
        InvalidOpcode{opcode}
    }
}

impl fmt::Display for InvalidOpcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid opcode: {}", self.opcode)
    }
}

impl error::Error for InvalidOpcode {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
