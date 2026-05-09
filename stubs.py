#!/usr/bin/env python3
import re
from pathlib import Path
from collections import defaultdict

def first_type_token(type_and_desc):
    """Extract Lua type token from '@param' or '@return' text."""
    return type_and_desc.strip().split()[0] if type_and_desc.strip() else "any"

def split_build_constant(const_name):
    """Map flat Rust constant names to nested Lua paths."""
    if const_name.startswith("COMPILER_"):
        return ("COMPILER", const_name[len("COMPILER_"):])
    if const_name.startswith("OPTIMIZE_"):
        return ("OPTIMIZE", const_name[len("OPTIMIZE_"):])
    if const_name.startswith("STD_"):
        return ("STD", const_name[len("STD_"):])
    if const_name.startswith("WARNINGS_"):
        return ("WARNINGS", const_name[len("WARNINGS_"):])
    if const_name.startswith("GENERATOR_"):
        return ("GENERATOR", const_name[len("GENERATOR_"):])
    if const_name.startswith("LANGUAGE_"):
        return ("LANGUAGE", const_name[len("LANGUAGE_"):])
    return None

# Map of (module, func_name) -> metadata
metadata = defaultdict(dict)

# Scan all .rs files in src/api (except build)
for rs_file in Path("src/api").rglob("*.rs"):
    # Skip build module for now (handled separately)
    if "build" in rs_file.parts:
        continue

    content = rs_file.read_text()

    # Find doc comment blocks followed by fn (public or private)
    pattern = r'((?:\/\/\/.*\n)*)\s*(?:#\[.*?\]\s*)*(?:(?:pub\s+)?(?:async\s+)?fn)\s+(\w+)'
    for match in re.finditer(pattern, content):
        doc_lines = match.group(1)
        func_name = match.group(2)

        # Extract metadata tags from doc comments
        meta = {}
        desc_match = re.search(r'\/\/\/\s*@desc\s+(.+)', doc_lines)
        if desc_match:
            meta['desc'] = desc_match.group(1).strip()

        params = []
        for param_match in re.finditer(r'\/\/\/\s*@param\s+([A-Za-z_]\w*\??)\s+(.+)', doc_lines):
            raw_param_name = param_match.group(1)
            param_name = raw_param_name.rstrip("?")
            param_optional = raw_param_name.endswith("?")
            params.append((param_name, param_optional, param_match.group(2).strip()))
        if params:
            meta['params'] = params

        return_match = re.search(r'\/\/\/\s*@return\s+(.+)', doc_lines)
        if return_match:
            meta['return'] = return_match.group(1).strip()

        # Infer module from file path
        rel_path = rs_file.relative_to("src/api")
        if rel_path.name == "mod.rs":
            module = rel_path.parent.name if rel_path.parent.name else None
        else:
            module = rel_path.parent.name if rel_path.parent.name else rel_path.stem

        if module and meta:
            metadata[(module, func_name)] = meta


# Extract pub const variables with documentation
variables = defaultdict(dict)  # {module: {const_name: {desc, value}}}

for rs_file in Path("src/api").rglob("*.rs"):
    if "build" in rs_file.parts:
        continue

    content = rs_file.read_text()

    # Find pub const with doc comments: /// @desc ... pub const NAME: Type = value;
    pattern = r'((?:\s*\/\/\/.*\n)*)\s*pub const\s+(\w+)(?:\s*:\s*[\w&<>:]+)?\s*=\s*([^;]+);'

    for match in re.finditer(pattern, content):
        doc_lines = match.group(1)
        const_name = match.group(2)
        const_value = match.group(3).strip()

        meta = {}
        desc_match = re.search(r'\/\/\/\s*@desc\s+(.+)', doc_lines)
        if desc_match:
            meta['desc'] = desc_match.group(1).strip()
            meta['value'] = const_value

            # Infer module from file path
            rel_path = rs_file.relative_to("src/api")
            if rel_path.name == "mod.rs":
                module = rel_path.parent.name if rel_path.parent.name else None
            else:
                module = rel_path.parent.name if rel_path.parent.name else rel_path.stem

            if module:
                variables[(module, const_name)] = meta

# Parse build module methods and constants
build_methods = defaultdict(dict)  # {language: {method_name: metadata}}
build_constants = defaultdict(dict)  # {language: {const_name: metadata}}

for rs_file in Path("src/api/build").glob("*.rs"):
    if rs_file.name == "common.rs" or rs_file.name == "generators.rs":
        continue

    content = rs_file.read_text()

    # Extract constants from build module
    const_pattern = r'((?:\s*\/\/\/.*\n)*)\s*pub const\s+(\w+)(?:\s*:\s*[\w&<>:]+)?\s*=\s*([^;]+);'
    for match in re.finditer(const_pattern, content):
        doc_lines = match.group(1)
        const_name = match.group(2)
        const_value = match.group(3).strip()

        meta = {}
        desc_match = re.search(r'\/\/\/\s*@desc\s+(.+)', doc_lines)
        if desc_match:
            meta['desc'] = desc_match.group(1).strip()
            meta['value'] = const_value

            if rs_file.name == "mod.rs":
                module = "build"
            else:
                module = rs_file.stem  # c, cpp, objc

            build_constants[module][const_name] = meta

    # Extract methods from build module (skip for mod.rs)
    if rs_file.name == "mod.rs":
        continue

    # Find documented methods: /// @desc ... // comment ... let method_name_fn = lua.create_function
    # This regex captures doc comments, allowing other comments/whitespace in between
    method_pattern = r'((?:\s*\/\/\/.*\n)+)(?:(?:\s*\/\/[^\n]*\n)*)\s*let\s+(\w+)_fn\s*=\s*lua\.create_function'

    for match in re.finditer(method_pattern, content):
        doc_lines = match.group(1)
        method_name = match.group(2)

        meta = {}
        desc_match = re.search(r'\/\/\/\s*@desc\s+(.+)', doc_lines)
        if desc_match:
            meta['desc'] = desc_match.group(1).strip()

        params = []
        for param_match in re.finditer(r'\/\/\/\s*@param\s+([A-Za-z_]\w*\??)\s+(.+)', doc_lines):
            raw_param_name = param_match.group(1)
            param_name = raw_param_name.rstrip("?")
            param_optional = raw_param_name.endswith("?")
            params.append((param_name, param_optional, param_match.group(2).strip()))
        if params:
            meta['params'] = params

        return_match = re.search(r'\/\/\/\s*@return\s+(.+)', doc_lines)
        if return_match:
            meta['return'] = return_match.group(1).strip()

        if meta:
            language = rs_file.stem  # c, cpp, objc
            build_methods[language][method_name] = meta


# Parse runtime to get registration mapping (func_name_rust -> lua_name)
runtime_src = Path("src/runtime/mod.rs").read_text()
reg_mapping = defaultdict(dict)  # module -> {lua_name: rust_func_name}

# Modules created via create_table() and then assigned to globals.
module_blocks = re.finditer(
    r'if let Ok\((?P<table>\w+)_module\) = self\.lua\.create_table\(\) \{(?P<body>.*?)\n\s*\);',
    runtime_src, re.S
)

for m in module_blocks:
    table = m.group("table")
    body = m.group("body")
    # Extract "lua_name" => |...| path::to::FunctionName(...)
    for func_match in re.finditer(r'"([^"]+)"\s*=>\s*\|[^|]*\|\s*(?:\w+::)*(\w+)::(\w+)', body):
        lua_name = func_match.group(1)
        rust_func = func_match.group(3)
        reg_mapping[table][lua_name] = rust_func

# Existing global modules that are extended in place (e.g. Lua's built-in `string`).
global_extension_blocks = re.finditer(
    r'if let Ok\(\w+_module\) = self\.lua\.globals\(\)\.get::<mlua::Table>\("(?P<module>[^"]+)"\) \{(?P<body>.*?)\n\s*\);',
    runtime_src, re.S
)

for m in global_extension_blocks:
    module = m.group("module")
    body = m.group("body")
    # Extract "lua_name" => |...| path::to::FunctionName(...)
    for func_match in re.finditer(r'"([^"]+)"\s*=>\s*\|[^|]*\|\s*(?:\w+::)*(\w+)::(\w+)', body):
        lua_name = func_match.group(1)
        rust_func = func_match.group(3)
        reg_mapping[module][lua_name] = rust_func

# Parse module loaders implemented outside runtime (e.g. src/api/lush.rs::load).
# Supports direct function refs in reg!, such as: "task" => task
for rs_file in Path("src/api").rglob("*.rs"):
    if "build" in rs_file.parts:
        continue
    content = rs_file.read_text()

    loader_blocks = re.finditer(
        r'if let Ok\((?P<var>\w+)_module\) = lua\.create_table\(\) \{(?P<body>.*?)\n\s*\}',
        content,
        re.S,
    )

    for block in loader_blocks:
        var = block.group("var")
        body = block.group("body")

        # Determine Lua module name from globals().set("<module>", <var>_module)
        mod_match = re.search(
            rf'lua\.globals\(\)\.set\("(?P<module>[^"]+)",\s*{re.escape(var)}_module\)',
            body,
        )
        if not mod_match:
            continue
        module = mod_match.group("module")

        # Find reg!(<var>_module, lua, ... );
        reg_match = re.search(
            rf'reg!\(\s*{re.escape(var)}_module\s*,\s*lua\s*,(?P<entries>.*?)\);\s*',
            body,
            re.S,
        )
        if not reg_match:
            continue
        entries = reg_match.group("entries")

        # Direct function refs: "name" => function_name
        for func_match in re.finditer(r'"([^"]+)"\s*=>\s*(\w+)', entries):
            lua_name = func_match.group(1)
            rust_func = func_match.group(2)
            reg_mapping[module][lua_name] = rust_func

# Generate stubs
stubs = []
stubs.append("---@meta")

# Map from Lua module names (from registration) to file module names (from file paths)
# This handles cases where filename != registered name (e.g., system.rs -> sys)
module_name_map = {
    "sys": "system",
}

# Generate stubs for traditional modules
for module in sorted(reg_mapping.keys()):
    stubs.append(f"-- {module} module")
    stubs.append(f"---@class {module}")
    stubs.append(f"{module} = {{}}")

    for lua_name, rust_func in sorted(reg_mapping[module].items()):
        meta = metadata.get((module, rust_func), {})

        if 'desc' in meta:
            stubs.append(f"---@desc {meta['desc']}")

        if 'params' in meta:
            for param_name, param_optional, param_type in meta['params']:
                optional_suffix = "?" if param_optional else ""
                stubs.append(f"---@param {param_name}{optional_suffix} {param_type}")

        if 'return' in meta:
            stubs.append(f"---@return {meta['return']}")

        # Emit overload for signatures where optional params appear before required ones.
        # Example: f(name, depends?, handler) -> overload f(name, handler)
        if 'params' in meta:
            params = meta['params']
            has_optional_before_required = any(
                param_optional and any(not p2[1] for p2 in params[i + 1 :])
                for i, (_, param_optional, _) in enumerate(params)
            )
            if has_optional_before_required:
                overload_parts = []
                for param_name, param_optional, param_type in params:
                    if param_optional:
                        continue
                    overload_parts.append(f"{param_name}: {first_type_token(param_type)}")
                overload_sig = ", ".join(overload_parts)
                stubs.append(f"---@overload fun({overload_sig})")

        # Build function signature
        if 'params' in meta:
            param_str = ", ".join(p[0] for p in meta['params'])
            stubs.append(f"function {module}.{lua_name}({param_str}) end")
        else:
            stubs.append(f"function {module}.{lua_name}(...) end")

        stubs.append("")

    # Add variable constants for this module
    # Map lua module name to file module name
    file_module_name = module_name_map.get(module, module)
    for const_name in sorted([k[1] for k in variables.keys() if k[0] == file_module_name]):
        const_meta = variables[(file_module_name, const_name)]
        if 'desc' in const_meta:
            stubs.append(f"---@desc {const_meta['desc']}")
        stubs.append(f"{module}.{const_name} = {const_meta['value']}")
        stubs.append("")

    stubs.append("")


# Generate stubs for build module
if build_methods or build_constants:
    stubs.append("-- build module")
    stubs.append("---@class build")
    stubs.append("build = {}")
    stubs.append("")

    # Build compiler constants
    if "build" in build_constants:
        stubs.append("-- Build compiler constants")
        for const_name in sorted(build_constants["build"].keys()):
            const_meta = build_constants["build"][const_name]
            const_path = split_build_constant(const_name)
            if 'desc' in const_meta:
                stubs.append(f"---@desc {const_meta['desc']}")
            if const_path:
                group, leaf = const_path
                stubs.append(f"build.{group} = build.{group} or {{}}")
                stubs.append(f"build.{group}.{leaf} = {const_meta['value']}")
            else:
                stubs.append(f"build.{const_name} = {const_meta['value']}")
            stubs.append("")
        stubs.append("")

    # Language constructors and methods - iterate over all languages that have methods or constants
    all_langs = (set(build_methods.keys()) | set(build_constants.keys())) - {"build"}
    for lang in sorted(all_langs):
        stubs.append(f"--- Language constructor for {lang.upper()}")
        stubs.append(f"---@return table build_task")
        stubs.append(f"function build.{lang}() end")
        stubs.append("")

        # Language-specific constants
        if lang in build_constants:
            stubs.append(f"-- {lang.upper()} constants")
            # build.<lang> is intentionally a function in the runtime API.
            # LuaLS flags function field assignments (inject-field), so silence it
            # only for this generated constants block.
            stubs.append("---@diagnostic disable: inject-field")
            emitted_groups = set()
            for const_name in sorted(build_constants[lang].keys()):
                const_meta = build_constants[lang][const_name]
                const_path = split_build_constant(const_name)
                if 'desc' in const_meta:
                    stubs.append(f"---@desc {const_meta['desc']}")
                if const_path:
                    group, leaf = const_path
                    if group not in emitted_groups:
                        stubs.append(f"build.{lang}.{group} = build.{lang}.{group} or {{}}")
                        emitted_groups.add(group)
                    stubs.append(f"build.{lang}.{group}.{leaf} = {const_meta['value']}")
                else:
                    stubs.append(f"build.{lang}.{const_name} = {const_meta['value']}")
                stubs.append("")
            stubs.append("---@diagnostic enable: inject-field")
            stubs.append("")

        # Methods for this language (if any)
        if lang in build_methods:
            stubs.append(f"-- {lang.upper()} build task methods")
            stubs.append(f"---@class build.{lang}_task")
            stubs.append(f"local {lang}_task = {{}}")
            stubs.append("")

            for method_name in sorted(build_methods[lang].keys()):
                meta = build_methods[lang][method_name]

                if 'desc' in meta:
                    stubs.append(f"---@desc {meta['desc']}")

                if 'params' in meta:
                    for param_name, param_optional, param_type in meta['params']:
                        optional_suffix = "?" if param_optional else ""
                        stubs.append(f"---@param {param_name}{optional_suffix} {param_type}")

                if 'return' in meta:
                    stubs.append(f"---@return {meta['return']}")
                else:
                    stubs.append(f"---@return build.{lang}_task")

                # Build method signature
                if 'params' in meta:
                    param_str = ", ".join(p[0] for p in meta['params'])
                    stubs.append(f"function {lang}_task:{method_name}({param_str}) end")
                else:
                    stubs.append(f"function {lang}_task:{method_name}(...) end")

                stubs.append("")

out = "\n".join(stubs)
Path("lua_stubs/lush.d.lua").write_text(out)
print("Wrote definitions for {} functions.".format(sum(len(funcs) for funcs in reg_mapping.values())))
print("Modules: {}".format(", ".join(sorted(reg_mapping.keys()))))
if build_methods:
    print("Build methods: {}".format(", ".join(sorted(build_methods.keys()))))
print("... to lua_stubs/lush.d.lua")
