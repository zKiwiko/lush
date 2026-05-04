use mlua::prelude::*;

pub fn load(lua: &Lua) -> LuaResult<()> {
    if let Ok(lush_module) = lua.create_table() {
        // register_command(name, handler) or register_command(name, alias, handler)
        let register_cmd = lua.create_function(|lua, args: mlua::Variadic<mlua::Value>| {
            if args.len() < 2 {
                return Err(mlua::Error::RuntimeError(
                    "expected (name, handler) or (name, alias, handler)".into(),
                ));
            }

            // first arg: name
            let name = match &args[0] {
                mlua::Value::String(s) => s.to_str()?.to_owned(),
                _ => {
                    return Err(mlua::Error::RuntimeError(
                        "first argument must be a string".into(),
                    ));
                }
            };

            // determine alias + handler
            let (alias_opt, handler_val) = if args.len() == 2 {
                (None, &args[1])
            } else {
                let alias = match &args[1] {
                    mlua::Value::String(s) => Some(s.to_str()?.to_owned()),
                    _ => {
                        return Err(mlua::Error::RuntimeError("alias must be a string".into()));
                    }
                };
                (alias, &args[2])
            };

            // ensure handler is function
            let func = match handler_val {
                mlua::Value::Function(f) => f.clone(),
                _ => {
                    return Err(mlua::Error::RuntimeError(
                        "handler must be a function".into(),
                    ));
                }
            };

            // get or create the commands table
            let globals = lua.globals();
            let cmds = match globals.get::<mlua::Table>("lush_commands") {
                Ok(t) => t,
                Err(_) => {
                    let t = lua.create_table()?;
                    globals.set("lush_commands", t.clone())?;
                    t
                }
            };

            cmds.set(name, func.clone())?;
            if let Some(alias_name) = alias_opt {
                cmds.set(alias_name, func)?;
            }

            Ok(())
        })?;

        lush_module.set("register", register_cmd)?;
        lua.globals().set("lush", lush_module)?;
    }

    Ok(())
}
