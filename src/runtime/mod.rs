use crate::errors::LushErr;
use mlua::prelude::*;
use std::{collections::HashSet, path::Path};

mod api;

pub struct Runtime {
    lua: Lua,
}

impl Runtime {
    pub fn new() -> Result<Self, LushErr> {
        let lua = Lua::new();
        let runtime = Self { lua };

        runtime.register_api()?;

        Ok(runtime)
    }

    fn register_api(&self) -> Result<(), LushErr> {
        let lush = self.lua.create_table()?;
        let tasks = self.lua.create_table()?;

        self.lua.set_named_registry_value("lush.tasks", tasks)?;

        lush.set("task", self.lua.create_function(api::task)?)?;
        lush.set("exec", self.lua.create_function(api::exec)?)?;
        lush.set("vexec", self.lua.create_function(api::vexec)?)?;

        self.lua.globals().set("lush", lush)?;

        Ok(())
    }

    pub fn load(&self, path: &Path) -> Result<(), LushErr> {
        let source = std::fs::read_to_string(path)
            .map_err(|err| LushErr::new(format!("Couldnt read {}: {err}", path.display())))?;

        self.lua
            .load(&source)
            .set_name(path.to_string_lossy())
            .exec()
            .map_err(LushErr::from)
    }

    pub fn run(&mut self, name: &str, args: &[String]) -> Result<(), LushErr> {
        let mut finished = HashSet::new();
        let mut visiting = HashSet::new();

        self.run_task(name, args, &mut finished, &mut visiting)
    }

    pub fn run_task(
        &mut self,
        name: &str,
        args: &[String],
        finished: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
    ) -> Result<(), LushErr> {
        if finished.contains(name) {
            return Ok(());
        }

        if !visiting.insert(name.to_string()) {
            return Err(LushErr::new(format!(
                "Circular task dependencies involving: '{name}'"
            )));
        }

        let tasks: mlua::Table = self.lua.named_registry_value("lush.tasks")?;
        let task: mlua::Table = tasks
            .get(name)
            .map_err(|_| LushErr::new(format!("Task '{name}' not found")))?;
        let depends: mlua::Table = task.get("depends").map_err(LushErr::from)?;
        let handler: mlua::Function = task.get("handler").map_err(LushErr::from)?;

        let dependencies = depends
            .sequence_values::<String>()
            .collect::<LuaResult<Vec<_>>>()
            .map_err(LushErr::from)?;

        for dependency in dependencies {
            self.run(&dependency, &[])?;
        }

        let lua_args = args
            .iter()
            .map(|arg| self.lua.create_string(arg).map(mlua::Value::String))
            .collect::<LuaResult<Vec<_>>>()
            .map_err(LushErr::from)?;

        handler
            .call::<()>(mlua::MultiValue::from_vec(lua_args))
            .map_err(LushErr::from)?;

        visiting.remove(name);
        finished.insert(name.to_owned());

        Ok(())
    }
}
