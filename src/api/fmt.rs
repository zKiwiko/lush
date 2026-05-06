use mlua::prelude::*;
use std::fmt::Write as _;

fn format_value(
    lua: &Lua,
    value: mlua::Value,
    indent: usize,
    in_table: bool,
) -> mlua::Result<String> {
    match value {
        mlua::Value::Nil => Ok("nil".into()),
        mlua::Value::Boolean(b) => Ok(b.to_string()),
        mlua::Value::Integer(i) => Ok(i.to_string()),
        mlua::Value::Number(n) => Ok(n.to_string()),
        mlua::Value::String(s) => {
            let s_str = s.to_str()?;
            Ok(if in_table {
                format!("\"{}\"", s_str)
            } else {
                s_str.to_string()
            })
        }
        mlua::Value::Table(table) => format_table(lua, table, indent),
        _ => Ok("<unsupported>".into()),
    }
}

fn format_table(lua: &Lua, table: mlua::Table, indent: usize) -> mlua::Result<String> {
    let mut result = String::with_capacity(128);
    result.push_str("{\n");
    let next_indent = indent + 2;
    let indent_str = " ".repeat(next_indent);
    let close_indent_str = " ".repeat(indent);

    let mut first = true;
    for pair in table.pairs::<mlua::Value, mlua::Value>() {
        let (k, v) = pair?;

        if !first {
            result.push_str(",\n");
        }
        first = false;

        result.push_str(&indent_str);

        // Format key
        match k {
            mlua::Value::String(s) => write!(result, "\"{}\"", s.to_str()?).ok(),
            mlua::Value::Integer(i) => write!(result, "{}", i).ok(),
            _ => result.push_str("<key>").into(),
        };

        result.push_str(": ");
        result.push_str(&format_value(lua, v, next_indent, true)?);
    }

    if !first {
        result.push('\n');
        result.push_str(&close_indent_str);
    }
    result.push('}');

    Ok(result)
}

fn format_with_args(lua: &Lua, template: &str, args: &[mlua::Value]) -> mlua::Result<String> {
    let mut result = String::with_capacity(template.len() * 2); // Pre-allocate
    let bytes = template.as_bytes();
    let mut i = 0;
    let mut arg_index = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                if i + 1 < bytes.len() {
                    match bytes[i + 1] {
                        b'}' => {
                            if arg_index >= args.len() {
                                return Err(mlua::Error::RuntimeError(
                                    "not enough arguments for format string".into(),
                                ));
                            }
                            result.push_str(&format_value(lua, args[arg_index].clone(), 0, false)?);
                            arg_index += 1;
                            i += 2;
                            continue;
                        }
                        b'{' => {
                            result.push('{');
                            i += 2;
                            continue;
                        }
                        _ => {}
                    }
                }
                result.push('{');
                i += 1;
            }
            b'}' if i + 1 < bytes.len() && bytes[i + 1] == b'}' => {
                result.push('}');
                i += 2;
            }
            b => {
                result.push(b as char);
                i += 1;
            }
        }
    }

    if arg_index < args.len() {
        return Err(mlua::Error::RuntimeError(
            "too many arguments for format string".into(),
        ));
    }

    Ok(result)
}

#[allow(non_snake_case)]
pub fn Print(lua: &Lua, args: mlua::Variadic<mlua::Value>) -> mlua::Result<()> {
    let args: Vec<mlua::Value> = args.into_iter().collect();

    if args.is_empty() {
        println!();
        return Ok(());
    }

    let template = match &args[0] {
        mlua::Value::String(s) => s.to_str()?,
        _ => {
            return Err(mlua::Error::RuntimeError(
                "first argument to fmt.print must be a string".into(),
            ));
        }
    };

    let formatted = format_with_args(lua, &template, &args[1..])?;
    println!("{}", formatted);

    Ok(())
}
