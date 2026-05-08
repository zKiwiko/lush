pub mod c;
pub mod common;
pub mod generators;

use mlua::prelude::*;

/// Register build API with Lua
pub fn register(lua: &Lua) -> LuaResult<()> {
    let build_table = lua.create_table()?;

    // Language constructors - we'll register constants separately
    // and make the language tables callable
    c::register_constants(lua, &build_table)?;

    // Now set up the callable functions for each language
    let c_fn = lua.create_function(|lua, _: ()| c::create_task(lua))?;

    // Wrap functions with metatables so they're callable
    wrap_callable_constants(lua, &build_table, "c", c_fn)?;

    lua.globals().set("build", build_table)?;
    Ok(())
}

/// Wrap a constants table with a __call metamethod to make it callable
fn wrap_callable_constants(
    lua: &Lua,
    build_table: &LuaTable,
    lang: &str,
    func: LuaFunction,
) -> LuaResult<()> {
    let lang_table: LuaTable = build_table.get(lang)?;
    let metatable = lua.create_table()?;
    metatable.set("__call", func)?;
    lang_table.set_metatable(Some(metatable))?;
    Ok(())
}
