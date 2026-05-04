use mlua::prelude::*;

#[allow(non_snake_case)]
pub fn Print(_: &Lua, (fmt, args): (String, mlua::Variadic<String>)) -> mlua::Result<()> {
    let args: Vec<String> = args.into_iter().collect();
    let parts: Vec<&str> = fmt.split("{}").collect();
    let placeholders = parts.len() - 1;

    if args.len() != placeholders {
        return Err(mlua::Error::RuntimeError(
            format!("expected {} arguments, got {}", placeholders, args.len()).into(),
        ));
    }

    let mut out = String::new();
    for i in 0..placeholders {
        out.push_str(parts[i]);
        out.push_str(&args[i]);
    }
    out.push_str(parts[placeholders]);

    println!("{}", out);
    Ok(())
}
