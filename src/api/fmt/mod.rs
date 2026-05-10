use mlua::prelude::*;

mod helpers;

/// @desc Format time into a more readable format (ms)
/// @param time integer|number The time in milliseconds to format.
/// @return string The formatted time as a human-readable string.
pub fn time(ms: f64) -> mlua::Result<String> {
    let seconds = ms / 1000.0;

    if seconds >= 3600.0 {
        Ok(format!("{:.2}h", seconds / 3600.0))
    } else if seconds >= 60.0 {
        Ok(format!("{:.2}m", seconds / 60.0))
    } else {
        Ok(format!("{:.2}s", seconds))
    }
}

/// @desc Format bytes into a more readable format (e.g., 1024 becomes "1 KB").
/// @param bytes integer|number The number of bytes to format.
/// @return string The formatted byte size as a human-readable string.
pub fn bytes(bytes: f64) -> mlua::Result<String> {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut size = bytes;
    let mut unit_index = 0;

    if size >= 1024.0 {
        unit_index = ((size.ln() / 1024.0_f64.ln()).floor() as usize).min(units.len() - 1);
        size /= 1024.0_f64.powi(unit_index as i32);
    }

    Ok(format!("{:.2} {}", size, units[unit_index]))
}

/// @desc Pads the input value on the left with the specified fill string until it reaches the desired width. If the input value is already wider than or equal to the specified width, it is returned unchanged. The function accepts a value of any type (string, integer, or number), a target width as an integer, and an optional fill string (defaulting to a single space if not provided). The function returns a new string with the input value padded on the left to achieve the specified width.
/// @param value any
/// @param width integer
/// @param fill? string
/// @return string
pub fn pad_left(
    (value, width, fill): (mlua::Value, usize, Option<String>),
) -> mlua::Result<String> {
    let fill = fill.unwrap_or_else(|| " ".to_string());
    let value_str = match value {
        mlua::Value::String(s) => s.to_str()?.to_string(),
        mlua::Value::Integer(i) => i.to_string(),
        mlua::Value::Number(n) => n.to_string(),
        _ => {
            return Err(mlua::Error::RuntimeError(
                "fmt.pad_left only accepts string, integer, and number arguments".into(),
            ));
        }
    };

    if value_str.len() >= width {
        Ok(value_str)
    } else {
        let padding = fill.repeat((width - value_str.len()) / fill.len() + 1);
        Ok(format!("{}{}", padding, value_str))
    }
}

/// @desc Pads the input value on the right with the specified fill string until it reaches the desired width. If the input value is already wider than or equal to the specified width, it is returned unchanged. The function accepts a value of any type (string, integer, or number), a target width as an integer, and an optional fill string (defaulting to a single space if not provided). The function returns a new string with the input value padded on the right to achieve the specified width.
/// @param value any
/// @param width integer
/// @param fill? string
/// @return string
pub fn pad_right(
    (value, width, fill): (mlua::Value, usize, Option<String>),
) -> mlua::Result<String> {
    let fill = fill.unwrap_or_else(|| " ".to_string());
    let value_str = match value {
        mlua::Value::String(s) => s.to_str()?.to_string(),
        mlua::Value::Integer(i) => i.to_string(),
        mlua::Value::Number(n) => n.to_string(),
        _ => {
            return Err(mlua::Error::RuntimeError(
                "fmt.pad_right only accepts string, integer, and number arguments".into(),
            ));
        }
    };

    if value_str.len() >= width {
        Ok(value_str)
    } else {
        let padding = fill.repeat((width - value_str.len()) / fill.len() + 1);
        Ok(format!("{}{}", value_str, padding))
    }
}

/// @desc Converts a number to its hexadecimal representation, prefixed with "0x".
/// @param value integer|number The number to convert to hexadecimal.
/// @return string Result
pub fn to_hex(value: f64) -> mlua::Result<String> {
    let bytes = format!("{:X}", value as i64);

    Ok(format!("0x{}", bytes))
}

/// @desc Converts a number to its binary representation, prefixed with "0b".
/// @param value integer|number The number to convert to binary.
/// @return string Result
pub fn to_binary(value: f64) -> mlua::Result<String> {
    let bytes = format!("{:b}", value as i64);

    Ok(format!("0b{}", bytes))
}

/// @desc Converts a number to its octal representation, prefixed with "0o".
/// @param value integer|number The number to convert to octal.
/// @return string Result
pub fn to_octal(value: f64) -> mlua::Result<String> {
    let bytes = format!("{:o}", value as i64);

    Ok(format!("0o{}", bytes))
}
/// @desc Joins multiple path segments into a single path string, ensuring that there are no duplicate slashes between segments.
/// @param ... string|integer|number A variable number of path segments to join. Each segment can be a string, integer, or number. Non-string segments will be converted to strings before joining.
/// @return string Result The joined path string.
pub fn path_join(args: mlua::Variadic<mlua::Value>) -> mlua::Result<String> {
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
pub fn string(lua: &Lua, args: mlua::Variadic<mlua::Value>) -> mlua::Result<String> {
    let args: Vec<mlua::Value> = args.into_iter().collect();

    if args.is_empty() {
        return Ok(String::new());
    }

    let template: LuaBorrowedStr = match &args[0] {
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
pub fn print(lua: &Lua, args: mlua::Variadic<mlua::Value>) -> mlua::Result<()> {
    let args: Vec<mlua::Value> = args.into_iter().collect();

    if args.is_empty() {
        print!("");
        return Ok(());
    }

    let template: LuaBorrowedStr = match &args[0] {
        mlua::Value::String(s) => s.to_str()?,
        _ => {
            return Err(mlua::Error::RuntimeError(
                "first argument to fmt.print must be a string".into(),
            ));
        }
    };

    let formatted: String = helpers::format_with_args(lua, &template, &args[1..])?;
    print!("{}", formatted);

    Ok(())
}

/// @desc Prints a formatted string to the console. The first argument is a template string that can contain placeholders in the form of `{}`, and the subsequent arguments are the values to be formatted into the template. The function formats the string by replacing the placeholders with the provided arguments and then prints the resulting string to the console.
/// @param ... string|integer|number The first argument must be a string template, and the remaining arguments are the values to format into the template. Each argument can be a string, integer, or number. Non-string arguments will be converted to strings before formatting.
/// @return nil
pub fn println(lua: &Lua, args: mlua::Variadic<mlua::Value>) -> mlua::Result<()> {
    let args: Vec<mlua::Value> = args.into_iter().collect();

    if args.is_empty() {
        println!();
        return Ok(());
    }

    let template: LuaBorrowedStr = match &args[0] {
        mlua::Value::String(s) => s.to_str()?,
        _ => {
            return Err(mlua::Error::RuntimeError(
                "first argument to fmt.println must be a string".into(),
            ));
        }
    };

    let formatted: String = helpers::format_with_args(lua, &template, &args[1..])?;
    println!("{}", formatted);

    Ok(())
}
