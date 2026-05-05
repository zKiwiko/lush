use super::common::{
    Compiler, execute_build, expand_glob_patterns, find_library, result_to_lua_table,
    table_to_strings,
};
use super::generators::{
    BuildSystem, GeneratorConfig, execute_cmake, execute_ninja, generate_cmake, generate_ninja,
};
use mlua::prelude::*;

/// Initialize C++-specific constants
pub fn register_constants(lua: &Lua, build_table: &LuaTable) -> LuaResult<()> {
    let cpp_table = lua.create_table()?;

    // Optimization flags
    let opt_table = lua.create_table()?;
    opt_table.set("O0", "O0")?;
    opt_table.set("O1", "O1")?;
    opt_table.set("O2", "O2")?;
    opt_table.set("O3", "O3")?;
    opt_table.set("OS", "Os")?;
    opt_table.set("OZ", "Oz")?;
    cpp_table.set("OPTIMIZE", opt_table)?;

    // Standard versions
    let std_table = lua.create_table()?;
    std_table.set("CXX98", "c++98")?;
    std_table.set("CXX03", "c++03")?;
    std_table.set("CXX11", "c++11")?;
    std_table.set("CXX14", "c++14")?;
    std_table.set("CXX17", "c++17")?;
    std_table.set("CXX20", "c++20")?;
    std_table.set("CXX23", "c++23")?;
    cpp_table.set("STD", std_table)?;

    // Warning levels
    let warn_table = lua.create_table()?;
    warn_table.set("NONE", "")?;
    warn_table.set("NORMAL", "Wall")?;
    warn_table.set("ALL", "Wall")?;
    warn_table.set("EXTRA", "Wextra")?;
    warn_table.set("PEDANTIC", "pedantic")?;
    cpp_table.set("WARNINGS", warn_table)?;

    build_table.set("cpp", cpp_table)?;
    Ok(())
}

/// Create C++ build task table
pub fn create_task(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("_language", "cpp")?;
    table.set("_compiler", 0i32)?; // Default: GCC
    table.set("_std", "c++17")?; // Default: C++17
    table.set("_files", lua.create_table()?)?;
    table.set("_link_libs", lua.create_table()?)?;
    table.set("_include_dirs", lua.create_table()?)?;
    table.set("_lib_paths", lua.create_table()?)?;
    table.set("_defines", lua.create_table()?)?;
    table.set("_output", mlua::Value::Nil)?;
    table.set("_flags", lua.create_table()?)?;
    table.set("_optimize", mlua::Value::Nil)?;
    table.set("_debug", false)?;
    table.set("_warnings", mlua::Value::Nil)?;
    table.set("_build_system", "raw")?;

    let metatable = lua.create_table()?;

    let compiler_fn = lua.create_function(|_lua, (table, compiler): (LuaTable, i32)| {
        if Compiler::from_int(compiler).is_none() {
            return Err(LuaError::RuntimeError("Invalid compiler".into()));
        }
        table.set("_compiler", compiler)?;
        Ok(table)
    })?;

    let std_fn = lua.create_function(|_lua, (table, std): (LuaTable, String)| {
        table.set("_std", std)?;
        Ok(table)
    })?;

    let files_fn = lua.create_function(|lua, (table, files): (LuaTable, LuaTable)| {
        let file_list = table_to_strings(lua, &files)?;
        let files_table = lua.create_table()?;
        for (i, file) in file_list.iter().enumerate() {
            files_table.set(i + 1, file.clone())?;
        }
        table.set("_files", files_table)?;
        Ok(table)
    })?;

    let output_fn = lua.create_function(|_lua, (table, name): (LuaTable, String)| {
        table.set("_output", name)?;
        Ok(table)
    })?;

    let optimize_fn = lua.create_function(|_lua, (table, level): (LuaTable, String)| {
        table.set("_optimize", level)?;
        Ok(table)
    })?;

    let debug_fn = lua.create_function(|_lua, (table, enabled): (LuaTable, bool)| {
        table.set("_debug", enabled)?;
        Ok(table)
    })?;

    let warnings_fn = lua.create_function(|_lua, (table, level): (LuaTable, String)| {
        table.set("_warnings", level)?;
        Ok(table)
    })?;

    let include_dirs_fn = lua.create_function(|lua, (table, dirs): (LuaTable, LuaTable)| {
        let dir_list = table_to_strings(lua, &dirs)?;
        let dirs_table = lua.create_table()?;
        for (i, dir) in dir_list.iter().enumerate() {
            dirs_table.set(i + 1, dir.clone())?;
        }
        table.set("_include_dirs", dirs_table)?;
        Ok(table)
    })?;

    let defines_fn = lua.create_function(|lua, (table, defs): (LuaTable, LuaTable)| {
        let def_list = table_to_strings(lua, &defs)?;
        let defs_table = lua.create_table()?;
        for (i, def) in def_list.iter().enumerate() {
            defs_table.set(i + 1, def.clone())?;
        }
        table.set("_defines", defs_table)?;
        Ok(table)
    })?;

    let link_libs_fn = lua.create_function(|lua, (table, libs): (LuaTable, LuaTable)| {
        let lib_list = table_to_strings(lua, &libs)?;
        let libs_table = lua.create_table()?;
        for (i, lib) in lib_list.iter().enumerate() {
            libs_table.set(i + 1, lib.clone())?;
        }
        table.set("_link_libs", libs_table)?;
        Ok(table)
    })?;

    let flags_fn = lua.create_function(|lua, (table, flags): (LuaTable, LuaTable)| {
        let flag_list = table_to_strings(lua, &flags)?;
        let flags_table = lua.create_table()?;
        for (i, flag) in flag_list.iter().enumerate() {
            flags_table.set(i + 1, flag.clone())?;
        }
        table.set("_flags", flags_table)?;
        Ok(table)
    })?;

    // find_library method
    let find_library_fn = lua.create_function(|_lua, (table, lib_name): (LuaTable, String)| {
        let lib_info = find_library(&lib_name)?;

        // Add include directories
        let include_dirs_table: LuaTable = table.get("_include_dirs")?;
        let mut idx = 1;
        loop {
            match include_dirs_table.get::<mlua::Value>(idx)? {
                mlua::Value::Nil => break,
                _ => idx += 1,
            }
        }
        for (i, path) in lib_info.include_paths.iter().enumerate() {
            include_dirs_table.set(idx + i, path.clone())?;
        }

        // Add library paths
        let lib_paths_table: LuaTable = table.get("_lib_paths")?;
        let mut lib_path_idx = 1;
        loop {
            match lib_paths_table.get::<mlua::Value>(lib_path_idx)? {
                mlua::Value::Nil => break,
                _ => lib_path_idx += 1,
            }
        }
        for (i, path) in lib_info.lib_paths.iter().enumerate() {
            lib_paths_table.set(lib_path_idx + i, path.clone())?;
        }

        // Add library directories (using -L flags for raw compiler)
        let flags_table: LuaTable = table.get("_flags")?;
        let mut flag_idx = 1;
        loop {
            match flags_table.get::<mlua::Value>(flag_idx)? {
                mlua::Value::Nil => break,
                _ => flag_idx += 1,
            }
        }
        for (i, path) in lib_info.lib_paths.iter().enumerate() {
            flags_table.set(flag_idx + i, format!("-L{}", path))?;
        }

        // Add library names
        let link_libs_table: LuaTable = table.get("_link_libs")?;
        let mut lib_idx = 1;
        loop {
            match link_libs_table.get::<mlua::Value>(lib_idx)? {
                mlua::Value::Nil => break,
                _ => lib_idx += 1,
            }
        }
        for (i, lib) in lib_info.libs.iter().enumerate() {
            link_libs_table.set(lib_idx + i, lib.clone())?;
        }

        Ok(table)
    })?;

    // generator method
    let generator_fn = lua.create_function(|_lua, (table, gen_name): (LuaTable, String)| {
        if BuildSystem::from_string(&gen_name).is_none() {
            return Err(LuaError::RuntimeError(format!(
                "Invalid generator: {}",
                gen_name
            )));
        }
        table.set("_build_system", gen_name)?;
        Ok(table)
    })?;

    // generate method
    let generate_fn = lua.create_function(|lua, table: LuaTable| {
        let build_system_str: String = table.get("_build_system")?;
        let build_system = BuildSystem::from_string(&build_system_str)
            .ok_or_else(|| LuaError::RuntimeError("Invalid build system".into()))?;

        if build_system == BuildSystem::Raw {
            return Err(LuaError::RuntimeError(
                "Cannot generate for 'raw' build system".into(),
            ));
        }

        let output: Option<String> = match table.get("_output")? {
            mlua::Value::String(s) => Some(s.to_str()?.to_owned()),
            mlua::Value::Nil => None,
            _ => return Err(LuaError::RuntimeError("Invalid output type".into())),
        };

        let output_name = output.unwrap_or_else(|| "a.out".to_string());

        let files_table: LuaTable = table.get("_files")?;
        let files = table_to_strings(lua, &files_table)?;
        let files = expand_glob_patterns(files)?;

        let include_dirs_table: LuaTable = table.get("_include_dirs")?;
        let includes = table_to_strings(lua, &include_dirs_table)?;

        let defines_table: LuaTable = table.get("_defines")?;
        let defines = table_to_strings(lua, &defines_table)?;

        let link_libs_table: LuaTable = table.get("_link_libs")?;
        let link_libs = table_to_strings(lua, &link_libs_table)?;

        let flags_table: LuaTable = table.get("_flags")?;
        let flags = table_to_strings(lua, &flags_table)?;

        let lib_paths_table: LuaTable = table.get("_lib_paths")?;
        let lib_paths = table_to_strings(lua, &lib_paths_table)?;

        let optimize: Option<String> = match table.get("_optimize")? {
            mlua::Value::String(s) => Some(s.to_str()?.to_owned()),
            mlua::Value::Nil => None,
            _ => return Err(LuaError::RuntimeError("Invalid optimize type".into())),
        };

        let debug: bool = table.get("_debug")?;

        let config = GeneratorConfig {
            language: "cpp".to_string(),
            files,
            includes,
            lib_paths,
            defines,
            link_libs,
            flags,
            optimize,
            debug,
            output_name,
            frameworks: vec![],
        };

        match build_system {
            BuildSystem::CMake => {
                generate_cmake(&config).map_err(|e| {
                    LuaError::RuntimeError(format!("CMake generation failed: {}", e))
                })?;
            }
            BuildSystem::Ninja => {
                generate_ninja(&config).map_err(|e| {
                    LuaError::RuntimeError(format!("Ninja generation failed: {}", e))
                })?;
            }
            BuildSystem::Raw => {
                return Err(LuaError::RuntimeError(
                    "Raw build system not supported".into(),
                ));
            }
        }

        result_to_lua_table(
            lua,
            &crate::api::build::common::BuildResult {
                success: true,
                output: format!("Generated {} configuration", build_system.as_str()),
                error: None,
                exit_code: Some(0),
            },
        )
    })?;

    let run_fn = lua.create_function(|lua, table: LuaTable| {
        let compiler_int: i32 = table.get("_compiler")?;
        let compiler = Compiler::from_int(compiler_int)
            .ok_or_else(|| LuaError::RuntimeError("Invalid compiler".into()))?;

        let build_system_str: String = table.get("_build_system")?;
        let build_system = BuildSystem::from_string(&build_system_str)
            .ok_or_else(|| LuaError::RuntimeError("Invalid build system".into()))?;

        let output: Option<String> = match table.get("_output")? {
            mlua::Value::String(s) => Some(s.to_str()?.to_owned()),
            mlua::Value::Nil => None,
            _ => return Err(LuaError::RuntimeError("Invalid output type".into())),
        };

        let output_name = output.unwrap_or_else(|| "a.out".to_string());

        // If using a build system generator, handle it differently
        if build_system != BuildSystem::Raw {
            let files_table: LuaTable = table.get("_files")?;
            let files = table_to_strings(lua, &files_table)?;
            let files = expand_glob_patterns(files)?;

            let include_dirs_table: LuaTable = table.get("_include_dirs")?;
            let includes = table_to_strings(lua, &include_dirs_table)?;

            let defines_table: LuaTable = table.get("_defines")?;
            let defines = table_to_strings(lua, &defines_table)?;

            let link_libs_table: LuaTable = table.get("_link_libs")?;
            let link_libs = table_to_strings(lua, &link_libs_table)?;

            let flags_table: LuaTable = table.get("_flags")?;
            let flags = table_to_strings(lua, &flags_table)?;

            let lib_paths_table: LuaTable = table.get("_lib_paths")?;
            let lib_paths = table_to_strings(lua, &lib_paths_table)?;

            let optimize: Option<String> = match table.get("_optimize")? {
                mlua::Value::String(s) => Some(s.to_str()?.to_owned()),
                mlua::Value::Nil => None,
                _ => return Err(LuaError::RuntimeError("Invalid optimize type".into())),
            };

            let debug: bool = table.get("_debug")?;

            let config = GeneratorConfig {
                language: "cpp".to_string(),
                files,
                includes,
                lib_paths,
                defines,
                link_libs,
                flags,
                optimize,
                debug,
                output_name: output_name.clone(),
                frameworks: vec![],
            };

            // Generate configuration
            match build_system {
                BuildSystem::CMake => {
                    generate_cmake(&config).map_err(|e| {
                        LuaError::RuntimeError(format!("CMake generation failed: {}", e))
                    })?;
                    execute_cmake(&output_name).map_err(|e| {
                        LuaError::RuntimeError(format!("CMake build failed: {}", e))
                    })?;
                }
                BuildSystem::Ninja => {
                    generate_ninja(&config).map_err(|e| {
                        LuaError::RuntimeError(format!("Ninja generation failed: {}", e))
                    })?;
                    execute_ninja(&output_name).map_err(|e| {
                        LuaError::RuntimeError(format!("Ninja build failed: {}", e))
                    })?;
                }
                BuildSystem::Raw => {}
            }

            return result_to_lua_table(
                lua,
                &crate::api::build::common::BuildResult {
                    success: true,
                    output: format!("Built with {}", build_system.as_str()),
                    error: None,
                    exit_code: Some(0),
                },
            );
        }

        // Raw compiler path (existing logic)
        let files_table: LuaTable = table.get("_files")?;
        let files = table_to_strings(lua, &files_table)?;
        let files = expand_glob_patterns(files)?;

        if files.is_empty() {
            return Err(LuaError::RuntimeError("No source files specified".into()));
        }

        let std: String = table.get("_std")?;
        let link_libs_table: LuaTable = table.get("_link_libs")?;
        let link_libs = table_to_strings(lua, &link_libs_table)?;

        let include_dirs_table: LuaTable = table.get("_include_dirs")?;
        let include_dirs = table_to_strings(lua, &include_dirs_table)?;

        let defines_table: LuaTable = table.get("_defines")?;
        let defines = table_to_strings(lua, &defines_table)?;

        let flags_table: LuaTable = table.get("_flags")?;
        let flags = table_to_strings(lua, &flags_table)?;

        let output: Option<String> = match table.get("_output")? {
            mlua::Value::String(s) => Some(s.to_str()?.to_owned()),
            mlua::Value::Nil => None,
            _ => return Err(LuaError::RuntimeError("Invalid output type".into())),
        };

        let optimize: Option<String> = match table.get("_optimize")? {
            mlua::Value::String(s) => Some(s.to_str()?.to_owned()),
            mlua::Value::Nil => None,
            _ => return Err(LuaError::RuntimeError("Invalid optimize type".into())),
        };

        let warnings: Option<String> = match table.get("_warnings")? {
            mlua::Value::String(s) => Some(s.to_str()?.to_owned()),
            mlua::Value::Nil => None,
            _ => return Err(LuaError::RuntimeError("Invalid warnings type".into())),
        };

        let debug: bool = table.get("_debug")?;

        // Build command
        let mut args = Vec::new();

        // Add standard
        args.push(format!("-std={}", std));

        // Add source files
        for file in &files {
            args.push(file.clone());
        }

        // Add include directories
        for dir in &include_dirs {
            args.push(format!("-I{}", dir));
        }

        // Add preprocessor defines
        for def in &defines {
            args.push(format!("-D{}", def));
        }

        // Add optimization
        if let Some(opt) = &optimize {
            args.push(format!("-{}", opt));
        }

        // Add warnings
        if let Some(warn) = &warnings {
            if !warn.is_empty() {
                args.push(format!("-{}", warn));
            }
        }

        // Add debug symbols
        if debug {
            args.push("-g".to_string());
        }

        // Add custom flags
        for flag in &flags {
            args.push(flag.clone());
        }

        // Add libraries
        for lib in &link_libs {
            args.push(format!("-l{}", lib));
        }

        // Set output
        let output_name = output.unwrap_or_else(|| "a.out".to_string());
        args.push("-o".to_string());
        args.push(output_name);

        let result = execute_build(compiler.as_str(), &args)?;
        result_to_lua_table(lua, &result)
    })?;

    let index_table = lua.create_table()?;
    index_table.set("compiler", compiler_fn)?;
    index_table.set("std", std_fn)?;
    index_table.set("files", files_fn)?;
    index_table.set("output", output_fn)?;
    index_table.set("optimize", optimize_fn)?;
    index_table.set("debug", debug_fn)?;
    index_table.set("warnings", warnings_fn)?;
    index_table.set("include_dirs", include_dirs_fn)?;
    index_table.set("defines", defines_fn)?;
    index_table.set("link_libs", link_libs_fn)?;
    index_table.set("flags", flags_fn)?;
    index_table.set("find_library", find_library_fn)?;
    index_table.set("generator", generator_fn)?;
    index_table.set("generate", generate_fn)?;
    index_table.set("run", run_fn)?;

    metatable.set("__index", index_table)?;
    table.set_metatable(Some(metatable))?;

    Ok(table)
}
