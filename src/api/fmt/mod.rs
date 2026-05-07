use mlua::prelude::*;

mod helpers;

/// Convert a number into its Hexidecimal representation.
/// 255 -> 0xff
/// returns a string
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

/// Convert a number into its binary representation.
/// 5 -> 0b101
/// returns a string
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

/// Convert a number into its octal representation.
/// 8 -> 0o10
/// returns a string
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

#[allow(non_snake_case)]
pub fn Path(_lua: &Lua, args: mlua::Variadic<mlua::Value>) -> mlua::Result<String> {
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
                    "fmt.path only accepts string, integer, and number arguments".into(),
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
