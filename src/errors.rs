use mlua::Error as LuaError;
use std::fmt::Display;

const ASCII_RED: &str = "\x1b[31m";
const ASCII_CLS: &str = "\x1b[0m";

pub struct LushErr {
    pub message: String,
}

impl LushErr {
    pub fn new(msg: String) -> Self {
        LushErr { message: msg }
    }
}

impl Display for LushErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{ASCII_RED}[Lush Err]{ASCII_CLS}: {}", self.message)
    }
}

impl From<LuaError> for LushErr {
    fn from(value: LuaError) -> Self {
        match value {
            _ => LushErr {
                message: format!("{ASCII_RED}(Lua Err){ASCII_CLS}: {value}"),
            },
        }
    }
}
