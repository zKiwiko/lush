use mlua::prelude::*;

use std::path::PathBuf;
use std::{env, fs};

use crate::api::system;
use crate::runtime;

const LUSH_VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! reg {
    ($table:expr, $lua:expr, $( $name:expr => $fn:expr ),* $(,)?) => {{
        $(
            $table.set($name, $lua.create_function($fn).unwrap()).unwrap();
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
        // Look up registered commands in the global `lush_commands` table
        let globals = self.lua.globals();
        match globals.get::<mlua::Table>("lush_commands") {
            Ok(cmds) => match cmds.get::<mlua::Function>(command) {
                Ok(func) => Ok(func.call::<()>(()).is_ok()),
                Err(_) => Ok(false),
            },
            Err(_) => Ok(false),
        }
    }

    fn load_api(&self) {
        // sys module
        if let Ok(sys_module) = self.lua.create_table() {
            reg!(sys_module, self.lua,
                "exec"   => |_, cmd: String| system::exec(cmd),
                "getenv" => |_, var: String| system::getenv(var),
                "setenv" => |_, (var, value): (String, String)| system::setenv(var, value),
            );

            sys_module.set("VERSION", LUSH_VERSION).unwrap();
            let _ = self.lua.globals().set("sys", sys_module);
        }

        // fmt module
        if let Ok(fmt_module) = self.lua.create_table() {
            reg!(fmt_module, self.lua,
                "print" => |lua, (fmt, args): (String, mlua::Variadic<String>)| crate::api::fmt::Print(lua, (fmt, args)),
            );
            let _ = self.lua.globals().set("fmt", fmt_module);
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
    }
}
