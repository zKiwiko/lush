use crate::{reg, regv};
use mlua::prelude::*;

fn parse_depends(_lua: &Lua, opts: &mlua::Table) -> LuaResult<Vec<String>> {
    // Check if it's just an array of dependencies: {"dep1", "dep2"}
    // Or a table with depends key: {depends = "dep1"} or {depends = {"dep1", "dep2"}}

    // First check if it has numeric keys (is it an array?)
    if opts.raw_len() > 0 {
        return opts.sequence_values::<String>().collect();
    }

    // Otherwise check for depends key
    match opts.get::<mlua::Value>("depends")? {
        mlua::Value::String(s) => Ok(vec![s.to_str()?.to_owned()]),
        mlua::Value::Table(dep_table) => dep_table.sequence_values::<String>().collect(),
        mlua::Value::Nil => Ok(vec![]),
        _ => Err(LuaError::RuntimeError(
            "'depends' must be a string or array of strings".into(),
        )),
    }
}

/// @desc Register a new task for Lush to execute.
/// @param name string Name of the task. This will be used to execute it.
/// @param depends? table Execute other tasks before this one.
/// @param handler function The function to execute for this task.
fn task(lua: &Lua, args: mlua::Variadic<mlua::Value>) -> LuaResult<()> {
    if args.is_empty() {
        return Err(LuaError::RuntimeError(
            "task() requires at least a name and handler".into(),
        ));
    }

    let name = match &args[0] {
        mlua::Value::String(s) => s.to_str()?.to_owned(),
        _ => return Err(LuaError::RuntimeError("task name must be a string".into())),
    };

    let (depends, handler) = if args.len() == 2 {
        (vec![], &args[1])
    } else if args.len() == 3 {
        match &args[1] {
            mlua::Value::Table(opts) => {
                let deps = parse_depends(lua, opts)?;
                (deps, &args[2])
            }
            mlua::Value::Nil => (vec![], &args[2]),
            _ => {
                return Err(LuaError::RuntimeError(
                    "options must be a table or nil".into(),
                ));
            }
        }
    } else {
        return Err(LuaError::RuntimeError(
            "task() takes 2 or 3 arguments: name, [opts], handler".into(),
        ));
    };

    let func = match handler {
        mlua::Value::Function(f) => f.clone(),
        _ => return Err(LuaError::RuntimeError("handler must be a function".into())),
    };

    let task_registry = lua.named_registry_value::<mlua::Table>("lush_tasks")?;
    let task_table = lua.create_table()?;
    task_table.set("name", name.clone())?;
    task_table.set("handler", func)?;

    let depends_table = lua.create_table()?;
    for (i, dep) in depends.iter().enumerate() {
        depends_table.set(i + 1, dep.clone())?;
    }
    task_table.set("depends", depends_table)?;

    task_registry.set(name, task_table)?;

    Ok(())
}

fn rule(_lua: &Lua, (_output, _input, _handler): (String, String, LuaFunction)) -> LuaResult<()> {
    // TODO: Implement rules if needed
    Ok(())
}

fn target(_lua: &Lua, (_files, _opts): (mlua::Value, Option<mlua::Table>)) -> LuaResult<()> {
    // TODO: Implement targets if needed
    Ok(())
}

pub fn load(lua: &Lua) -> LuaResult<()> {
    // Initialize task registry in Lua registry
    let task_registry = lua.create_table()?;
    lua.set_named_registry_value("lush_tasks", task_registry)?;

    // let lush_module = lua.create_table()?;

    // lush_module.set("task", lua.create_function(task)?)?;
    // lush_module.set("rule", lua.create_function(rule)?)?;
    // lush_module.set("target", lua.create_function(target)?)?;

    if let Ok(lush_module) = lua.create_table() {
        reg!(lush_module, lua,
            "task" => task,
            "rule" => rule,
            "target" => target,
        );
        regv!(lush_module, lua,
            "VERSION" => env!("CARGO_PKG_VERSION"),
        );
        let _ = lua.globals().set("lush", lush_module);
    }

    Ok(())
}
