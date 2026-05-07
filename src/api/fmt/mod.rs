use mlua::prelude::*;

mod helpers;

/// @desc Converts a number to its hexadecimal representation, prefixed with "0x".
/// @param value integer|number The number to convert to hexadecimal.
/// @return string Result
pub fn to_hex(_lua: &Lua, value: mlua::Value) -> mlua::Result<String> {
    let bytes = match value {
        mlua::Value::Integer(i) => format!("{:X}", i),
        mlua::Value::Number(n) => format!("{:X}", n as i64),
        _ => {
            return Err(mlua::Error::RuntimeError(
                "fmt.to_hex only accepts integer and number arguments".into(),
            ));
        }
    };

    Ok(format!("0x{}", bytes))
}

/// @desc Converts a number to its binary representation, prefixed with "0b".
/// @param value integer|number The number to convert to binary.
/// @return string Result
pub fn to_binary(_lua: &Lua, value: mlua::Value) -> mlua::Result<String> {
    let bytes = match value {
        mlua::Value::Integer(i) => format!("{:b}", i),
        mlua::Value::Number(n) => format!("{:b}", n as i64),
        _ => {
            return Err(mlua::Error::RuntimeError(
                "fmt.to_binary only accepts integer and number arguments".into(),
            ));
        }
    };

    Ok(format!("0b{}", bytes))
}

/// @desc Converts a number to its octal representation, prefixed with "0o".
/// @param value integer|number The number to convert to octal.
/// @return string Result
pub fn to_octal(_lua: &Lua, value: mlua::Value) -> mlua::Result<String> {
    let bytes = match value {
        mlua::Value::Integer(i) => format!("{:o}", i),
        mlua::Value::Number(n) => format!("{:o}", n as i64),
        _ => {
            return Err(mlua::Error::RuntimeError(
                "fmt.to_octal only accepts integer and number arguments".into(),
            ));
        }
    };

    Ok(format!("0o{}", bytes))
}
/// @desc Joins multiple path segments into a single path string, ensuring that there are no duplicate slashes between segments.
/// @param ... string|integer|number A variable number of path segments to join. Each segment can be a string, integer, or number. Non-string segments will be converted to strings before joining.
/// @return string Result The joined path string.
pub fn path_join(_lua: &Lua, args: mlua::Variadic<mlua::Value>) -> mlua::Result<String> {
    let args: Vec<mlua::Value> = args.into_iter().collect();

    if args.is_empty() {
        return Ok(String::new());
    }

    let mut parts = Vec::with_capacity(args.len());

    for value in args {
        let part = match value {
            mlua::Value::String(s) => s.to_str()?.to_owned(),
            mlua::Value::Integer(i) => i.to_string(),
            mlua::Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "fmt.path_join only accepts string, integer, and number arguments".into(),
                ));
            }
        };

        let part = part.trim_matches('/').to_owned();
        if !part.is_empty() {
            parts.push(part);
        }
    }

    Ok(parts.join("/"))
}

/// @desc Formats a string using a template and a variable number of arguments. The first argument is the template string, and the subsequent arguments are the values to be formatted into the template. The template can contain placeholders in the form of `{}` which will be replaced by the corresponding arguments in order.
/// @param ... string|integer|number The first argument must be a string template, and the remaining arguments are the values to format into the template. Each argument can be a string, integer, or number. Non-string arguments will be converted to strings before formatting.
/// @return string Result The formatted string resulting from replacing the placeholders in the template with the provided arguments.
#[allow(non_snake_case)]
pub fn String(lua: &Lua, args: mlua::Variadic<mlua::Value>) -> mlua::Result<String> {
    let args: Vec<mlua::Value> = args.into_iter().collect();

    if args.is_empty() {
        return Ok(String::new());
    }

    let template: LuaBorrowedStr<'_> = match &args[0] {
        mlua::Value::String(s) => s.to_str()?,
        _ => {
            return Err(mlua::Error::RuntimeError(
                "first argument to fmt.string must be a string".into(),
            ));
        }
    };

    helpers::format_with_args(lua, &template, &args[1..])
}

/// @desc Prints a formatted string to the console. The first argument is a template string that can contain placeholders in the form of `{}`, and the subsequent arguments are the values to be formatted into the template. The function formats the string by replacing the placeholders with the provided arguments and then prints the resulting string to the console.
/// @param ... string|integer|number The first argument must be a string template, and the remaining arguments are the values to format into the template. Each argument can be a string, integer, or number. Non-string arguments will be converted to strings before formatting.
/// @return nil
#[allow(non_snake_case)]
pub fn Print(lua: &Lua, args: mlua::Variadic<mlua::Value>) -> mlua::Result<()> {
    let args: Vec<mlua::Value> = args.into_iter().collect();

    if args.is_empty() {
        println!();
        return Ok(());
    }

    let template: LuaBorrowedStr<'_> = match &args[0] {
        mlua::Value::String(s) => s.to_str()?,
        _ => {
            return Err(mlua::Error::RuntimeError(
                "first argument to fmt.print must be a string".into(),
            ));
        }
    };

    let formatted: String = helpers::format_with_args(lua, &template, &args[1..])?;
    println!("{}", formatted);

    Ok(())
}
