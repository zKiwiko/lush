use mlua::prelude::*;

use std::fs;
use std::path::PathBuf;

use crate::api::{build, json, string, sys};
use crate::{reg, regv};

pub struct Runtime {
    lua: Lua,
}

impl Runtime {
    /// @param `c` If true, enable LuaJIT's unsafe FFI and Debug libraries.
    pub fn new(c: bool) -> Self {
        let lua = if c {
            unsafe { Lua::unsafe_new() }
        } else {
            Lua::new()
        };

        let runtime = Runtime { lua };

        runtime.load_api();

        runtime
    }
    pub fn execute(
        &self,
        command: &str,
        arguments: &[String],
        file_path: Option<String>,
    ) -> LuaResult<bool> {
        // determine which file to load
        let script_path = match file_path {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("./lush.lua"),
        };

        // read lush.lua
        let src = match fs::read_to_string(&script_path) {
            Ok(s) => s,
            Err(err) => {
                eprintln!(
                    "could not read lush.lua ({}): {}",
                    script_path.display(),
                    err
                );
                return Ok(false);
            }
        };

        // execute the script (this should register commands)
        if let Err(err) = self.lua.load(&src).exec() {
            eprintln!("error running lush.lua: {}", err);
            return Ok(false);
        }

        // run the requested command (registered by lush.lua)
        self.run_command(command, arguments)
    }
    /// Load and execute the user's `lush.lua` but do not invoke any registered command.
    /// Returns Ok(true) if the script executed successfully.
    pub fn dry_execute(&self, file_path: Option<String>) -> LuaResult<bool> {
        // determine which file to load
        let script_path = match file_path {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("./lush.lua"),
        };

        // read lush.lua
        let src = match fs::read_to_string(&script_path) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("could not read file ({}): {}", script_path.display(), err);
                return Ok(false);
            }
        };

        // execute the script (this should register commands)
        if let Err(err) = self.lua.load(&src).exec() {
            eprintln!("error running lush.lua: {}", err);
            return Ok(false);
        }

        Ok(true)
    }
    pub fn run_command(&self, command: &str, arguments: &[String]) -> LuaResult<bool> {
        // First, try to execute as a task (with dependency resolution)
        if let Ok(success) = self.execute_task(command, arguments) {
            return Ok(success);
        }

        // Fall back to legacy command lookup
        let globals = self.lua.globals();
        match globals.get::<mlua::Table>("lush_commands") {
            Ok(cmds) => match cmds.get::<mlua::Function>(command) {
                Ok(func) => {
                    if arguments.is_empty() {
                        Ok(func.call::<()>(()).is_ok())
                    } else {
                        // Pass arguments as separate variadic arguments
                        let args: Vec<mlua::Value> = arguments
                            .iter()
                            .map(|s| mlua::Value::String(self.lua.create_string(s).unwrap()))
                            .collect();
                        Ok(func
                            .call::<mlua::MultiValue>(mlua::MultiValue::from_vec(args))
                            .is_ok())
                    }
                }
                Err(_) => Ok(false),
            },
            Err(_) => Ok(false),
        }
    }
    fn execute_task(&self, task_name: &str, arguments: &[String]) -> LuaResult<bool> {
        let task_registry = self.lua.named_registry_value::<mlua::Table>("lush_tasks")?;

        // Check if task exists
        match task_registry.get::<mlua::Table>(task_name) {
            Ok(_) => {
                // Execute with dependency resolution
                let mut visited = std::collections::HashSet::new();
                let mut rec_stack = std::collections::HashSet::new();
                self.visit_task(
                    task_name,
                    &task_registry,
                    &mut visited,
                    &mut rec_stack,
                    arguments,
                )?;
                Ok(true)
            }
            Err(_) => Err(LuaError::RuntimeError("task not found".into())),
        }
    }
    fn visit_task(
        &self,
        task_name: &str,
        task_registry: &mlua::Table,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
        arguments: &[String],
    ) -> LuaResult<()> {
        if visited.contains(task_name) {
            return Ok(());
        }

        if rec_stack.contains(task_name) {
            return Err(LuaError::RuntimeError(format!(
                "circular dependency detected involving task '{}'",
                task_name
            )));
        }

        rec_stack.insert(task_name.to_string());

        let task_table: mlua::Table = task_registry.get(task_name)?;
        let depends: mlua::Table = task_table.get("depends")?;

        // Execute dependencies first
        let mut i = 1;
        loop {
            match depends.get::<mlua::Value>(i) {
                Ok(mlua::Value::String(s)) => {
                    let dep_name = s.to_str()?.to_owned();
                    self.visit_task(&dep_name, task_registry, visited, rec_stack, arguments)?;
                    i += 1;
                }
                Ok(mlua::Value::Nil) | Err(_) => break,
                _ => {
                    i += 1;
                }
            }
        }

        // Execute this task
        let handler: mlua::Function = task_table.get("handler")?;
        if arguments.is_empty() {
            handler.call::<()>(())?;
        } else {
            handler.call::<mlua::MultiValue>(
                arguments
                    .iter()
                    .map(|s| mlua::Value::String(self.lua.create_string(s).unwrap()))
                    .collect::<mlua::MultiValue>(),
            )?;
        }

        rec_stack.remove(task_name);
        visited.insert(task_name.to_string());
        Ok(())
    }
    fn load_api(&self) {
        // sys module
        if let Ok(sys_module) = self.lua.create_table() {
            reg!(sys_module, self.lua,
                "exec"   => |_, cmd: String| sys::exec(cmd),
                "getenv" => |_, var: String| sys::getenv(var),
                "setenv" => |_, (var, value): (String, String)| sys::setenv(var, value),
                "find"   => |_, (what, name): (u8, String)| sys::find(what, name),
                "mkdir"  => |_, path: String| sys::mkdir(path),
                "rm"     => |_, path: String| sys::rm(path),
                "cp"     => |_, (src, dst): (String, String)| sys::cp(src, dst),
                "mv"     => |_, (src, dst): (String, String)| sys::mv(src, dst),
                "cwd"    => |_, ()| sys::pwd(),
                "envs"   => |lua, ()| sys::envs(lua),
                "os"     => |_, ()| sys::os(),
                "arch"   => |_, ()| sys::arch(),
                "which"  => |_, command: String| sys::which(command),
                "grep"   => |lua, (pattern, text): (String, String)| sys::grep(lua, pattern, text),
                "popen"  => |_, command: String| sys::popen(command),
            );

            regv!(sys_module, self.lua,
                "FILE" => sys::FILE,
                "DIRECTORY" => sys::DIRECTORY,
                "SYMLINK" => sys::SYMLINK,
            );

            let _ = self.lua.globals().set("sys", sys_module);
        }

        // fmt module
        if let Ok(fmt_module) = self.lua.create_table() {
            reg!(fmt_module, self.lua,
                "print" => |lua, args: mlua::Variadic<mlua::Value>| crate::api::fmt::Print(lua, args),
                "string" => |lua, args: mlua::Variadic<mlua::Value>| crate::api::fmt::String(lua, args),
                "path_join" => |lua, args: mlua::Variadic<mlua::Value>| crate::api::fmt::path_join(lua, args),

                "to_hex" => |lua, value: mlua::Value| crate::api::fmt::to_hex(lua, value),
                "to_bin" => |lua, value: mlua::Value| crate::api::fmt::to_binary(lua, value),
                "to_oct" => |lua, value: mlua::Value| crate::api::fmt::to_octal(lua, value)
            );
            let _ = self.lua.globals().set("fmt", fmt_module);
        }

        // JSON module
        if let Ok(json_module) = self.lua.create_table() {
            reg!(json_module, self.lua,
                "read_file" => |lua, path: String| json::read_file(lua, path),
                "read_string" => |lua, content: String| json::read_string(lua, content),
                "write_file" => |_, (path, value): (String, mlua::Value)| json::write_file(path, value),
                "write_string" => |_, value: mlua::Value| json::write_string(value),
            );
            let _ = self.lua.globals().set("json", json_module);
        }

        // string module additions
        if let Ok(str_module) = self.lua.globals().get::<mlua::Table>("string") {
            reg!(str_module, self.lua,
                "trim" => |lua, string: String| string::Trim(lua, string),
                "split" => |lua, (string, sep): (String, String)| string::Split(lua, (string, sep)),
            );
        }

        // ensure a table to hold registered commands (persists functions)
        if self
            .lua
            .globals()
            .get::<mlua::Table>("lush_commands")
            .is_err()
        {
            let _ = self
                .lua
                .globals()
                .set("lush_commands", self.lua.create_table().unwrap());
        }

        // lush module
        let _ = crate::api::lush::load(&self.lua);

        // build module
        let _ = build::register(&self.lua);
    }
}
