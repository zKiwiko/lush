use mlua::prelude::*;

use std::string;

/// ## Usage
/// `str.trim(string) -> string`
///
/// ## Description
/// Trims leading and trailing whitespace from the input string.
///
/// ## Example
/// ```lua
/// local result = str.trim("   Hello, World!   ")
/// print(result) -- "Hello, World!"
/// ```
#[allow(non_snake_case)]
#[inline(always)]
pub fn Trim(_lua: &Lua, string: String) -> mlua::Result<String> {
    Ok(string.trim().to_string())
}

/// ## Usage
/// `str.split(string, sep) -> table`
///
/// ## Description
/// Splits a string into a table of substrings based on the specified separator. If the separator is an empty string, it defaults to splitting on whitespace.
///
/// ## Example:
/// ```lua
/// local result = str.split("Hello, World!", ", ")
/// result[1] -- "Hello"
/// result[2] -- "World!"
/// ```
#[allow(non_snake_case)]
#[inline(always)]
pub fn Split(_lua: &Lua, (string, sep): (String, String)) -> mlua::Result<Vec<String>> {
    let sep = if sep.is_empty() { " " } else { &sep };
    Ok(string.split(sep).map(|s| s.to_string()).collect())
}
