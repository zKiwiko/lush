use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub enum BuildSystem {
    Raw,   // Direct compiler invocation
    CMake,
    Meson,
}

impl BuildSystem {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "cmake" => Some(BuildSystem::CMake),
            "meson" => Some(BuildSystem::Meson),
            "raw" | "none" | "" => Some(BuildSystem::Raw),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BuildSystem::Raw => "raw",
            BuildSystem::CMake => "cmake",
            BuildSystem::Meson => "meson",
        }
    }
}

pub struct GeneratorConfig {
    pub language: String,
    pub files: Vec<String>,
    pub includes: Vec<String>,
    pub defines: Vec<String>,
    pub link_libs: Vec<String>,
    pub flags: Vec<String>,
    pub optimize: Option<String>,
    pub debug: bool,
    pub output_name: String,
    pub frameworks: Vec<String>, // For Objective-C
}

/// Generate CMakeLists.txt from builder configuration
pub fn generate_cmake(config: &GeneratorConfig) -> std::io::Result<()> {
    let mut file = File::create("CMakeLists.txt")?;

    let mut content = String::from("cmake_minimum_required(VERSION 3.10)\n");
    content.push_str(&format!("project({})\n\n", config.output_name));

    // Language setup
    match config.language.as_str() {
        "c" => content.push_str("enable_language(C)\n"),
        "cpp" | "c++" => content.push_str("enable_language(CXX)\n"),
        "objc" => content.push_str("enable_language(OBJC)\n"),
        _ => {}
    }

    // Include directories
    for inc in &config.includes {
        content.push_str(&format!("include_directories({})\n", inc));
    }

    // Compiler definitions
    for define in &config.defines {
        content.push_str(&format!("add_compile_definitions({})\n", define));
    }

    // Compiler flags
    for flag in &config.flags {
        content.push_str(&format!("add_compile_options({})\n", flag));
    }

    if config.debug {
        content.push_str("add_compile_options(-g)\n");
    }

    if let Some(opt) = &config.optimize {
        content.push_str(&format!("add_compile_options(-{})\n", opt));
    }

    // Executable target
    let file_list = config.files.join(" ");
    content.push_str(&format!("\nadd_executable({} {})\n", config.output_name, file_list));

    // Link libraries
    for lib in &config.link_libs {
        content.push_str(&format!("target_link_libraries({} {})\n", config.output_name, lib));
    }

    // Objective-C frameworks
    for framework in &config.frameworks {
        content.push_str(&format!(
            "find_library(FRAMEWORK_{} NAMES {} TYPE FRAMEWORK)\n",
            framework.to_uppercase(),
            framework
        ));
        content.push_str(&format!(
            "target_link_libraries({} ${{FRAMEWORK_{}}})\n",
            config.output_name,
            framework.to_uppercase()
        ));
    }

    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Generate meson.build from builder configuration
pub fn generate_meson(config: &GeneratorConfig) -> std::io::Result<()> {
    let mut file = File::create("meson.build")?;

    let lang_str = match config.language.as_str() {
        "c" => "c",
        "cpp" | "c++" => "cpp",
        "objc" => "objc",
        _ => "c",
    };

    let mut content = format!("project('{}', '{}')\n\n", config.output_name, lang_str);

    // Build arguments
    let mut build_args = Vec::new();

    for inc in &config.includes {
        build_args.push(format!("'-I{}'", inc));
    }

    for define in &config.defines {
        build_args.push(format!("'-D{}'", define));
    }

    for flag in &config.flags {
        build_args.push(format!("'{}'", flag));
    }

    if config.debug {
        build_args.push("'-g'".to_string());
    }

    if let Some(opt) = &config.optimize {
        build_args.push(format!("'-{}'", opt));
    }

    // Executable definition
    content.push_str(&format!("executable('{}',\n", config.output_name));

    for file in &config.files {
        content.push_str(&format!("  '{}',\n", file));
    }

    if !build_args.is_empty() {
        content.push_str(&format!("  c_args: [{}],\n", build_args.join(", ")));
    }

    // Link libraries
    if !config.link_libs.is_empty() {
        let libs_str = config
            .link_libs
            .iter()
            .map(|l| format!("'{}'", l))
            .collect::<Vec<_>>()
            .join(", ");
        content.push_str(&format!("  dependencies: [{}],\n", libs_str));
    }

    // Objective-C frameworks
    if !config.frameworks.is_empty() {
        let frameworks_str = config
            .frameworks
            .iter()
            .map(|f| format!("'{}'", f))
            .collect::<Vec<_>>()
            .join(", ");
        content.push_str(&format!("  link_args: [{}],\n", frameworks_str));
    }

    content.push_str(")\n");

    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Execute CMake build
pub fn execute_cmake(output_name: &str) -> std::io::Result<()> {
    // Create build directory
    std::fs::create_dir_all("build")?;

    // Run cmake
    let status = std::process::Command::new("cmake")
        .args(&["--build", "build", "--target", output_name])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "CMake build failed",
        ))
    }
}

/// Execute Meson build
pub fn execute_meson(_output_name: &str) -> std::io::Result<()> {
    // Setup meson if not already done
    if !Path::new("build").exists() {
        std::process::Command::new("meson")
            .args(&["setup", "build"])
            .status()?;
    }

    // Run ninja
    let status = std::process::Command::new("meson")
        .args(&["compile", "-C", "build"])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Meson build failed",
        ))
    }
}
