use mlua::prelude::*;

use std::path::PathBuf;
use std::{env, fs};

use crate::api::{build, json, system};

const LUSH_VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! reg {
    ($table:expr, $lua:expr, $( $name:expr => $fn:expr ),* $(,)?) => {{
        $(
            $table.set($name, $lua.create_function($fn).unwrap()).unwrap();
        )*
    }};
}

macro_rules! regv {
    ($table:expr, $lua:expr, $( $key:expr => $value:expr ),* $(,)?) => {{
        $(
            $table.set($key, $value).unwrap();
        )*
    }};
}

pub struct Runtime {
    lua: Lua,
}

impl Runtime {
    pub fn new() -> Self {
        let runtime = Runtime { lua: Lua::new() };

        runtime.load_api();

        runtime
    }

    pub fn execute(&self, command: &str, file_path: Option<String>) -> LuaResult<bool> {
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
        self.run_command(command)
    }

    /// Load and execute the user's `lush.lua` but do not invoke any registered command.
    /// Returns Ok(true) if the script executed successfully.
    pub fn load_init_only(&self, file_path: Option<String>) -> LuaResult<bool> {
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

        Ok(true)
    }

    pub fn run_command(&self, command: &str) -> LuaResult<bool> {
        // First, try to execute as a task (with dependency resolution)
        if let Ok(success) = self.execute_task(command) {
            return Ok(success);
        }

        // Fall back to legacy command lookup
        let globals = self.lua.globals();
        match globals.get::<mlua::Table>("lush_commands") {
            Ok(cmds) => match cmds.get::<mlua::Function>(command) {
                Ok(func) => Ok(func.call::<()>(()).is_ok()),
                Err(_) => Ok(false),
            },
            Err(_) => Ok(false),
        }
    }

    fn execute_task(&self, task_name: &str) -> LuaResult<bool> {
        let task_registry = self.lua.named_registry_value::<mlua::Table>("lush_tasks")?;

        // Check if task exists
        match task_registry.get::<mlua::Table>(task_name) {
            Ok(_) => {
                // Execute with dependency resolution
                let mut visited = std::collections::HashSet::new();
                let mut rec_stack = std::collections::HashSet::new();
                self.visit_task(task_name, &task_registry, &mut visited, &mut rec_stack)?;
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
                    self.visit_task(&dep_name, task_registry, visited, rec_stack)?;
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
        handler.call::<()>(())?;

        rec_stack.remove(task_name);
        visited.insert(task_name.to_string());
        Ok(())
    }

    fn load_api(&self) {
        // sys module
        if let Ok(sys_module) = self.lua.create_table() {
            reg!(sys_module, self.lua,
                "exec"   => |_, cmd: String| system::exec(cmd),
                "getenv" => |_, var: String| system::getenv(var),
                "setenv" => |_, (var, value): (String, String)| system::setenv(var, value),
                "find"   => |_, (what, name): (u8, String)| system::find(what, name),
                "mkdir"  => |_, path: String| system::mkdir(path),
                "rm"     => |_, path: String| system::rm(path),
                "cp"     => |_, (src, dst): (String, String)| system::cp(src, dst),
                "mv"     => |_, (src, dst): (String, String)| system::mv(src, dst),
                "cwd"    => |_, ()| system::cwd(),
                "read"   => |_, path: String| system::read(path),
                "envs"   => |lua, ()| system::envs(lua),
                "os"    => |_, ()| system::os(),
                "arch"  => |_, ()| system::arch(),
                "which" => |_, command: String| system::which(command),
                "grep"  => |lua, (pattern, text): (String, String)| system::grep(lua, pattern, text),
                "popen" => |_, command: String| system::popen(command),
            );

            regv!(sys_module, self.lua,
                "FILE" => system::FILE,
                "DIRECTORY" => system::DIRECTORY,
                "SYMLINK" => system::SYMLINK,
                "VERSION" => LUSH_VERSION
            );

            let _ = self.lua.globals().set("sys", sys_module);
        }

        // fmt module
        if let Ok(fmt_module) = self.lua.create_table() {
            reg!(fmt_module, self.lua,
                "print" => |lua, args: mlua::Variadic<mlua::Value>| crate::api::fmt::Print(lua, args),
            );
            let _ = self.lua.globals().set("fmt", fmt_module);
        }

        if let Ok(json_module) = self.lua.create_table() {
            reg!(json_module, self.lua,
                "read_file" => |lua, path: String| json::read_file(lua, path),
                "read_string" => |lua, content: String| json::read_string(lua, content),
                "write_file" => |_, (path, value): (String, mlua::Value)| json::write_file(path, value),
                "write_string" => |_, value: mlua::Value| json::write_string(value),
            );
            let _ = self.lua.globals().set("json", json_module);
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
