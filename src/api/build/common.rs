use glob::glob as glob_pattern;
use mlua::prelude::*;
use std::process::Command;

/// Library information found by pkg-config
#[derive(Debug, Clone)]
pub struct LibraryInfo {
    pub include_paths: Vec<String>,
    pub lib_paths: Vec<String>,
    pub libs: Vec<String>,
}

/// Supported compilers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compiler {
    Gxx,
    Gcc,
    Clang,
}

impl Compiler {
    pub fn as_str(&self) -> &'static str {
        match self {
            Compiler::Gxx => "g++",
            Compiler::Gcc => "gcc",
            Compiler::Clang => "clang",
        }
    }

    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(Compiler::Gxx),
            1 => Some(Compiler::Gcc),
            2 => Some(Compiler::Clang),
            _ => None,
        }
    }
}

/// Build result containing success status and metadata
#[derive(Debug, Clone)]
pub struct BuildResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub exit_code: Option<i32>,
}

impl BuildResult {
    pub fn new_success(output: String) -> Self {
        Self {
            success: true,
            output,
            error: None,
            exit_code: None,
        }
    }

    pub fn new_error(error: String, exit_code: i32) -> Self {
        Self {
            success: false,
            output: String::new(),
            error: Some(error),
            exit_code: Some(exit_code),
        }
    }
}

/// Helper to convert Lua table values to strings vector
pub fn table_to_strings(_lua: &Lua, table: &LuaTable) -> LuaResult<Vec<String>> {
    let mut result = Vec::new();
    let mut i = 1;
    loop {
        match table.get::<mlua::Value>(i)? {
            mlua::Value::String(s) => {
                result.push(s.to_str()?.to_owned());
                i += 1;
            }
            mlua::Value::Nil => break,
            _ => return Err(LuaError::RuntimeError("Expected strings in table".into())),
        }
    }
    Ok(result)
}

/// Expand glob patterns in a list of file paths
/// If a path contains glob characters (*, ?, [), it's treated as a pattern
/// Otherwise, it's used as-is (even if it doesn't exist)
pub fn expand_glob_patterns(patterns: Vec<String>) -> LuaResult<Vec<String>> {
    let mut files = Vec::new();

    for pattern in patterns {
        // Check if this looks like a glob pattern
        if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
            match glob_pattern(&pattern) {
                Ok(paths) => {
                    for entry in paths.flatten() {
                        files.push(entry.to_string_lossy().to_string());
                    }
                }
                Err(e) => {
                    return Err(LuaError::RuntimeError(format!(
                        "Invalid glob pattern '{}': {}",
                        pattern, e
                    )));
                }
            }
        } else {
            // Treat as literal file path
            files.push(pattern);
        }
    }

    // Sort for consistent builds
    files.sort();

    Ok(files)
}

/// Find a library using pkg-config
pub fn find_library(name: &str) -> LuaResult<LibraryInfo> {
    match pkg_config::probe_library(name) {
        Ok(lib) => {
            let mut include_paths = Vec::new();
            let mut lib_paths = Vec::new();
            let mut libs = Vec::new();

            // Extract include directories
            for include_path in &lib.include_paths {
                include_paths.push(include_path.to_string_lossy().to_string());
            }

            // Extract library directories
            for lib_path in &lib.link_paths {
                lib_paths.push(lib_path.to_string_lossy().to_string());
            }

            // The library name itself
            libs.push(name.to_string());

            Ok(LibraryInfo {
                include_paths,
                lib_paths,
                libs,
            })
        }
        Err(e) => Err(LuaError::RuntimeError(format!(
            "Failed to find library '{}': {}",
            name, e
        ))),
    }
}

/// Execute a build command
pub fn execute_build(compiler: &str, args: &[String]) -> LuaResult<BuildResult> {
    let mut cmd = Command::new(compiler);
    for arg in args {
        cmd.arg(arg);
    }

    match cmd.output() {
        Ok(output_status) => {
            let status = output_status.status;
            if status.success() {
                Ok(BuildResult::new_success("Build successful".to_string()))
            } else {
                let stderr = String::from_utf8_lossy(&output_status.stderr).to_string();
                Ok(BuildResult::new_error(stderr, status.code().unwrap_or(-1)))
            }
        }
        Err(e) => Ok(BuildResult::new_error(e.to_string(), -1)),
    }
}

/// Convert BuildResult to Lua table
pub fn result_to_lua_table(lua: &Lua, result: &BuildResult) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("success", result.success)?;
    table.set("output", result.output.clone())?;
    if let Some(err) = &result.error {
        table.set("error", err.clone())?;
    }
    if let Some(code) = result.exit_code {
        table.set("exit_code", code)?;
    }
    Ok(table)
}
