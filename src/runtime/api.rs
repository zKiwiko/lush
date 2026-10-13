use std::process::Command;

use mlua::prelude::*;

/* Holds Rust definitions of stuff exposed to the lua api */

pub fn task(
    lua: &Lua,
    (name, depends, handler): (String, Option<mlua::Table>, mlua::Function),
) -> LuaResult<()> {
    let tasks: mlua::Table = lua.named_registry_value("lush.tasks")?;

    if tasks.get::<mlua::Value>(name.as_str())? != mlua::Value::Nil {
        return Err(LuaError::RuntimeError(format!(
            "task '{name}' is already defined."
        )));
    }

    let dependencies = match depends {
        Some(table) => table
            .sequence_values::<String>()
            .collect::<LuaResult<Vec<_>>>()?,
        None => Vec::new(),
    };

    let depends_table = lua.create_table()?;

    for (index, dependency) in dependencies.into_iter().enumerate() {
        depends_table.set(index + 1, dependency)?;
    }

    let task = lua.create_table()?;

    task.set("depends", depends_table)?;
    task.set("handler", handler)?;

    tasks.set(name, task)?;

    Ok(())
}

pub fn exec(_: &Lua, command: String) -> LuaResult<()> {
    if command.trim().is_empty() {
        return Err(LuaError::RuntimeError("command cannot be empty".into()));
    }

    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", &command]).status()
    } else {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_owned());

        Command::new(shell).args(["-c", &command]).status()
    }
    .map_err(LuaError::external)?;

    if status.success() {
        Ok(())
    } else {
        Err(LuaError::RuntimeError(format!(
            "command failed with status {}",
            status.code().unwrap_or(-1)
        )))
    }
}
/// "Verbose" version of exec that returns the exit code, stdout, and stderr as a tuple.
pub fn vexec(_: &Lua, command: String) -> LuaResult<(i32, String, String)> {
    if command.trim().is_empty() {
        return Err(LuaError::RuntimeError("command cannot be empty".into()));
    }

    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", &command])
            .output()
            .map_err(LuaError::external)
    } else {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_owned());

        std::process::Command::new(shell)
            .args(["-c", &command])
            .output()
            .map_err(LuaError::external)
    }?;

    Ok((
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    ))
}
