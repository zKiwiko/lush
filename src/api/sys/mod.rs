use mlua::prelude::*;

/// @desc Constant representing a file type
pub const FILE: u8 = 0;
/// @desc Constant representing a directory type
pub const DIRECTORY: u8 = 1;
/// @desc Constant representing a symbolic link type
pub const SYMLINK: u8 = 2;

/// @desc Checks whether a path exists and matches a requested type.
/// @param what integer One of `sys.FILE`, `sys.DIRECTORY`, or `sys.SYMLINK`.
/// @param name string Path to test.
/// @return boolean True if the path exists and matches the requested type, otherwise false.
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

/// @desc Executes a shell command and streams output to the terminal.
/// @param command string Command to run. Must not be empty.
/// @return nil
pub fn exec(command: String) -> LuaResult<()> {
    if command.trim().is_empty() {
        return Err(mlua::Error::RuntimeError("command cannot be empty".into()));
    }

    if command.trim().to_lowercase() == "lush" {
        return Err(mlua::Error::RuntimeError("recursive call to 'lush'".into()));
    }

    // Run shell command
    #[cfg(target_os = "windows")]
    let status = std::process::Command::new("cmd")
        .arg("/C")
        .arg(command)
        .status()
        .map_err(|err| mlua::Error::RuntimeError(format!("failed to execute command: {err}")))?;

    #[cfg(not(target_os = "windows"))]
    let status = {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        std::process::Command::new(&shell)
            .arg("-c")
            .arg(command)
            .status()
            .map_err(|err| mlua::Error::RuntimeError(format!("failed to execute command: {err}")))?
    };

    if !status.success() {
        return Err(mlua::Error::RuntimeError(format!(
            "command exited with status {}",
            status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "terminated by signal".to_string())
        )));
    }

    Ok(())
}

/// @desc Gets the value of an environment variable.
/// @param var string Environment variable name.
/// @return string The environment variable value.
pub fn getenv(var: String) -> LuaResult<String> {
    match std::env::var(&var) {
        Ok(val) => Ok(val),
        Err(_) => Err(mlua::Error::RuntimeError(format!(
            "Environment variable '{}' not found",
            var
        ))),
    }
}

/// @desc Sets an environment variable for the current process.
/// @param var string Environment variable name.
/// @param value string Environment variable value.
/// @return nil
pub fn setenv(var: String, value: String) -> LuaResult<()> {
    unsafe {
        std::env::set_var(var, value);
    }

    Ok(())
}

/// @desc Creates a directory and any missing parent directories.
/// @param path string Directory path to create.
/// @return nil
pub fn mkdir(path: String) -> LuaResult<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

/// @desc Removes a file or directory recursively.
/// @param path string Path to remove.
/// @return nil
pub fn rm(path: String) -> LuaResult<()> {
    let path = std::path::Path::new(&path);
    if path.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else if path.is_file() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

/// @desc Copies a file from one path to another.
/// @param src string Source file path.
/// @param dst string Destination file path.
/// @return nil
pub fn cp(src: String, dst: String) -> LuaResult<()> {
    std::fs::copy(src, dst)?;
    Ok(())
}

/// @desc Renames or moves a file or directory.
/// @param src string Source path.
/// @param dst string Destination path.
/// @return nil
pub fn mv(src: String, dst: String) -> LuaResult<()> {
    std::fs::rename(src, dst)?;
    Ok(())
}

// Current working directory (not change)
/// @desc Returns the current working directory.
/// @return string Absolute path of the current working directory.
pub fn pwd() -> LuaResult<String> {
    match std::env::current_dir() {
        Ok(path) => Ok(path.to_string_lossy().to_string()),
        Err(err) => Err(mlua::Error::RuntimeError(format!(
            "Failed to get current working directory: {}",
            err
        ))),
    }
}

/// @desc Executes a shell command and returns captured standard output.
/// @param command string Command to run.
/// @return string Captured standard output.
pub fn popen(command: String) -> LuaResult<String> {
    if command.trim().is_empty() {
        return Err(mlua::Error::RuntimeError("command cannot be empty".into()));
    }

    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .arg("/C")
            .arg(command)
            .output()
            .map_err(|err| mlua::Error::RuntimeError(format!("failed to execute command: {err}")))?
    } else {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        std::process::Command::new(shell)
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|err| mlua::Error::RuntimeError(format!("failed to execute command: {err}")))?
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let detail = if stderr.is_empty() {
            format!(
                "command exited with status {}",
                output
                    .status
                    .code()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "terminated by signal".to_string())
            )
        } else {
            stderr
        };
        return Err(mlua::Error::RuntimeError(detail));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// @desc Returns the operating system name.
/// @return string OS name (for example `linux`, `macos`, or `windows`).
pub fn os() -> LuaResult<String> {
    Ok(std::env::consts::OS.to_string())
}

/// @desc Returns the CPU architecture name.
/// @return string Architecture name (for example `x86_64` or `aarch64`).
pub fn arch() -> LuaResult<String> {
    Ok(std::env::consts::ARCH.to_string())
}

/// @desc Finds an executable in the system PATH.
/// @param command string Executable name to search for.
/// @return string Absolute path to the executable.
pub fn which(command: String) -> LuaResult<String> {
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("where")
            .arg(&command)
            .output()
            .map_err(|err| mlua::Error::RuntimeError(format!("failed to execute command: {err}")))?
    } else {
        std::process::Command::new("which")
            .arg(&command)
            .output()
            .map_err(|err| mlua::Error::RuntimeError(format!("failed to execute command: {err}")))?
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

/// @desc Returns a table containing all current environment variables.
/// @return table A key-value table of environment variables.
pub fn envs(lua: &mlua::Lua) -> LuaResult<mlua::Table> {
    let env_table: mlua::Table = lua.create_table()?;
    for (key, value) in std::env::vars() {
        env_table.set(key, value)?;
    }
    Ok(env_table)
}

/// @desc Performs regex matching over multiline text and returns matching lines.
/// @param pattern string Regular expression pattern.
/// @param text string Input text to search.
/// @return table Array-like table containing matching lines.
pub fn grep(lua: &mlua::Lua, pattern: String, text: String) -> LuaResult<mlua::Table> {
    let regex = match regex::Regex::new(&pattern) {
        Ok(r) => r,
        Err(err) => {
            return Err(mlua::Error::RuntimeError(format!(
                "Invalid regex pattern: {}",
                err
            )));
        }
    };

    let result_table: mlua::Table = lua.create_table()?;
    for (i, line) in text.lines().enumerate() {
        if regex.is_match(line) {
            result_table.set(i + 1, line.to_string())?;
        }
    }
    Ok(result_table)
}

/// @desc Returns the size in bytes of a Lua string or numeric value.
/// @param value any A Lua string, integer, or number.
/// @return integer Size in bytes for the provided value.
pub fn sizeof(value: mlua::Value) -> LuaResult<usize> {
    let size = match value {
        mlua::Value::String(s) => s.as_bytes().len(),
        mlua::Value::Integer(i) => std::mem::size_of_val(&i),
        mlua::Value::Number(n) => std::mem::size_of_val(&n),
        _ => {
            return Err(mlua::Error::RuntimeError(
                "sizeof only accepts string, integer, and number arguments".into(),
            ));
        }
    };
    Ok(size)
}
