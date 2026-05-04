use mlua::prelude::*;

pub const FILE: u8 = 0;
pub const DIRECTORY: u8 = 1;
pub const SYMLINK: u8 = 2;

pub fn find(what: u8, name: String) -> LuaResult<bool> {
    let path = std::path::Path::new(&name);
    if path.exists() {
        match what {
            FILE => {
                if path.is_file() {
                    return Ok(true);
                }
            }
            DIRECTORY => {
                if path.is_dir() {
                    return Ok(true);
                }
            }
            SYMLINK => {
                if path.is_symlink() {
                    return Ok(true);
                }
            }
            _ => {}
        }
    }

    Ok(false)
}

pub fn exec(command: String) -> LuaResult<()> {
    if command.trim().is_empty() {
        return Err(mlua::Error::RuntimeError("command cannot be empty".into()));
    }

    if command.trim().to_lowercase() == "lush" {
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

pub fn mkdir(path: String) -> LuaResult<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub fn rm(path: String) -> LuaResult<()> {
    let path = std::path::Path::new(&path);
    if path.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else if path.is_file() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

pub fn cp(src: String, dst: String) -> LuaResult<()> {
    std::fs::copy(src, dst)?;
    Ok(())
}

pub fn mv(src: String, dst: String) -> LuaResult<()> {
    std::fs::rename(src, dst)?;
    Ok(())
}

// Current working directory (not change)
pub fn cwd() -> LuaResult<String> {
    match std::env::current_dir() {
        Ok(path) => Ok(path.to_string_lossy().to_string()),
        Err(err) => Err(mlua::Error::RuntimeError(format!(
            "Failed to get current working directory: {}",
            err
        ))),
    }
}

pub fn read(path: String) -> LuaResult<String> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(err) => Err(mlua::Error::RuntimeError(format!(
            "Failed to read file: {}",
            err
        ))),
    }
}

pub fn popen(command: String) -> LuaResult<String> {
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .arg("/C")
            .arg(command)
            .output()
            .expect("failed to execute process")
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn os() -> LuaResult<String> {
    Ok(std::env::consts::OS.to_string())
}

pub fn arch() -> LuaResult<String> {
    Ok(std::env::consts::ARCH.to_string())
}

pub fn which(command: String) -> LuaResult<String> {
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("where")
            .arg(&command)
            .output()
            .expect("failed to execute process")
    } else {
        std::process::Command::new("which")
            .arg(&command)
            .output()
            .expect("failed to execute process")
    };

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(mlua::Error::RuntimeError(format!(
            "Command '{}' not found",
            command
        )))
    }
}

pub fn envs() -> LuaResult<mlua::Table> {
    let env_table: mlua::Table = mlua::Lua::new().create_table()?;
    for (key, value) in std::env::vars() {
        env_table.set(key, value)?;
    }
    Ok(env_table)
}

pub fn grep(pattern: String, text: String) -> LuaResult<mlua::Table> {
    let regex = match regex::Regex::new(&pattern) {
        Ok(r) => r,
        Err(err) => {
            return Err(mlua::Error::RuntimeError(format!(
                "Invalid regex pattern: {}",
                err
            )));
        }
    };

    let result_table: mlua::Table = mlua::Lua::new().create_table()?;
    for (i, line) in text.lines().enumerate() {
        if regex.is_match(line) {
            result_table.set(i + 1, line.to_string())?;
        }
    }
    Ok(result_table)
}
