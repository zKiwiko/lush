use mlua::prelude::*;

pub fn exec(command: String) -> LuaResult<()> {
    if command.trim().is_empty() {
        return Err(mlua::Error::RuntimeError("command cannot be empty".into()));
    }

    if command == "lush" {
        return Err(mlua::Error::RuntimeError("Recursive call to 'lush'".into()));
    }

    // Run shell command
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .arg("/C")
            .arg(command)
            .status()
            .expect("failed to execute process");
    }

    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()
            .expect("failed to execute process");
    }
    Ok(())
}

pub fn getenv(var: String) -> LuaResult<String> {
    match std::env::var(&var) {
        Ok(val) => Ok(val),
        Err(_) => Ok(String::new()),
    }
}

pub fn setenv(var: String, value: String) -> LuaResult<()> {
    unsafe {
        std::env::set_var(var, value);
    }

    Ok(())
}
