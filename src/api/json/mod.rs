use mlua::prelude::*;

fn json_to_lua(lua: &Lua, json: serde_json::Value) -> LuaResult<mlua::Value> {
    match json {
        serde_json::Value::Null => Ok(mlua::Value::Nil),
        serde_json::Value::Bool(b) => Ok(mlua::Value::Boolean(b)),
        serde_json::Value::Number(n) => Ok(mlua::Value::Number(n.as_f64().unwrap_or(0.0))),
        serde_json::Value::String(s) => Ok(mlua::Value::String(lua.create_string(&s)?)),
        serde_json::Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, v) in arr.into_iter().enumerate() {
                table.set(i + 1, json_to_lua(lua, v)?)?;
            }
            Ok(mlua::Value::Table(table))
        }
        serde_json::Value::Object(map) => {
            let table = lua.create_table()?;
            for (k, v) in map {
                table.set(k, json_to_lua(lua, v)?)?;
            }
            Ok(mlua::Value::Table(table))
        }
    }
}

fn lua_to_json(value: mlua::Value) -> LuaResult<serde_json::Value> {
    match value {
        mlua::Value::Nil => Ok(serde_json::Value::Null),
        mlua::Value::Boolean(b) => Ok(serde_json::Value::Bool(b)),
        mlua::Value::Number(n) => Ok(serde_json::json!(n)),
        mlua::Value::Integer(i) => Ok(serde_json::json!(i)),
        mlua::Value::String(s) => Ok(serde_json::Value::String(s.to_str()?.to_string())),
        mlua::Value::Table(table) => {
            let mut index = 1;

            // Check if it's an array or object
            loop {
                match table.get::<mlua::Value>(index)? {
                    mlua::Value::Nil => break,
                    _ => index += 1,
                }
            }

            if index > 1 {
                // It's an array
                let mut arr = Vec::new();
                for i in 1..index {
                    arr.push(lua_to_json(table.get(i)?)?);
                }
                Ok(serde_json::Value::Array(arr))
            } else {
                // It's an object
                let mut map = serde_json::Map::new();
                for pair in table.pairs::<mlua::Value, mlua::Value>() {
                    let (k, v) = pair?;
                    if let mlua::Value::String(key) = k {
                        map.insert(key.to_str()?.to_string(), lua_to_json(v)?);
                    }
                }
                Ok(serde_json::Value::Object(map))
            }
        }
        _ => Err(mlua::Error::external(
            "unsupported value type for JSON conversion",
        )),
    }
}

/// @desc Reads a JSON file from disk and converts it into native Lua values.
/// @param path string Path to the JSON file.
/// @return any Parsed Lua value (typically a table).
pub fn read_file(lua: &Lua, path: String) -> LuaResult<mlua::Value> {
    let content = std::fs::read_to_string(path)?;
    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| mlua::Error::external(e.to_string()))?;
    json_to_lua(lua, json)
}

/// @desc Serializes a Lua value to pretty-printed JSON and writes it to a file.
/// @param path string Destination file path.
/// @param value any Lua value to serialize.
/// @return nil
pub fn write_file(path: String, value: mlua::Value) -> LuaResult<()> {
    let json = lua_to_json(value)?;
    let content =
        serde_json::to_string_pretty(&json).map_err(|e| mlua::Error::external(e.to_string()))?;
    std::fs::write(path, content)?;
    Ok(())
}

/// @desc Parses a JSON string and converts it into native Lua values.
/// @param json_str string JSON source string.
/// @return any Parsed Lua value (typically a table).
pub fn read_string(lua: &Lua, json_str: String) -> LuaResult<mlua::Value> {
    let json: serde_json::Value =
        serde_json::from_str(&json_str).map_err(|e| mlua::Error::external(e.to_string()))?;
    json_to_lua(lua, json)
}

/// @desc Serializes a Lua value into a pretty-printed JSON string.
/// @param value any Lua value to serialize.
/// @return string JSON output string.
pub fn write_string(value: mlua::Value) -> LuaResult<String> {
    let json = lua_to_json(value)?;
    serde_json::to_string_pretty(&json).map_err(|e| mlua::Error::external(e.to_string()))
}
