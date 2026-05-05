use mlua::prelude::*;

fn format_value(lua: &Lua, value: mlua::Value, indent: usize) -> mlua::Result<String> {
    match value {
        mlua::Value::Nil => Ok("nil".to_string()),
        mlua::Value::Boolean(b) => Ok(b.to_string()),
        mlua::Value::Integer(i) => Ok(i.to_string()),
        mlua::Value::Number(n) => Ok(n.to_string()),
        mlua::Value::String(s) => Ok(format!("{}", s.to_str()?)),
        mlua::Value::Table(table) => format_table(lua, table, indent),
        _ => Ok("<unsupported>".to_string()),
    }
}

fn format_table(lua: &Lua, table: mlua::Table, indent: usize) -> mlua::Result<String> {
    let mut result = String::from("{\n");
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
            mlua::Value::String(s) => result.push_str(&s.to_str()?),
            mlua::Value::Integer(i) => result.push_str(&i.to_string()),
            _ => result.push_str("<key>"),
        }

        result.push_str(": ");

        // Format value
        result.push_str(&format_value(lua, v, next_indent)?);
    }

    if !first {
        result.push('\n');
        result.push_str(&close_indent_str);
    }
    result.push('}');

    Ok(result)
}

#[allow(non_snake_case)]
pub fn Print(lua: &Lua, args: mlua::Variadic<mlua::Value>) -> mlua::Result<()> {
    let args: Vec<mlua::Value> = args.into_iter().collect();

    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", format_value(lua, arg.clone(), 0)?);
    }
    println!();

    Ok(())
}
