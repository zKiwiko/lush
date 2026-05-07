#!/usr/bin/env python3
import re
from pathlib import Path
from collections import defaultdict

# Map of (module, func_name) -> metadata
metadata = defaultdict(dict)

# Scan all .rs files in src/api
for rs_file in Path("src/api").rglob("*.rs"):
    content = rs_file.read_text()

    # Find doc comment blocks followed by pub fn (with optional attributes in between)
    pattern = r'((?:\/\/\/.*\n)*)\s*(?:#\[.*?\]\s*)*(?:pub fn|pub async fn)\s+(\w+)'
    for match in re.finditer(pattern, content):
        doc_lines = match.group(1)
        func_name = match.group(2)

        # Extract metadata tags from doc comments
        meta = {}
        desc_match = re.search(r'\/\/\/\s*@desc\s+(.+)', doc_lines)
        if desc_match:
            meta['desc'] = desc_match.group(1).strip()

        params = []
        for param_match in re.finditer(r'\/\/\/\s*@param\s+(\w+)\s+(.+)', doc_lines):
            params.append((param_match.group(1), param_match.group(2).strip()))
        if params:
            meta['params'] = params

        return_match = re.search(r'\/\/\/\s*@return\s+(.+)', doc_lines)
        if return_match:
            meta['return'] = return_match.group(1).strip()

        # Infer module from file path
        rel_path = rs_file.relative_to("src/api")
        if rel_path.name == "mod.rs":
            module = rel_path.parent.name if rel_path.parent.name != "." else None
        else:
            module = rel_path.parent.name if rel_path.parent.name != "." else rel_path.stem

        if module and meta:
            metadata[(module, func_name)] = meta

# Parse runtime to get registration mapping (func_name_rust -> lua_name)
runtime_src = Path("src/runtime/mod.rs").read_text()
reg_mapping = defaultdict(dict)  # module -> {lua_name: rust_func_name}

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

# Generate stubs
stubs = []
stubs.append("---@meta")
for module in sorted(reg_mapping.keys()):
    stubs.append(f"-- {module} module")
    stubs.append(f"---@class {module}")
    stubs.append(f"{module} = {{}}")

    for lua_name, rust_func in sorted(reg_mapping[module].items()):
        meta = metadata.get((module, rust_func), {})

        if 'desc' in meta:
            stubs.append(f"--- {meta['desc']}")

        if 'params' in meta:
            for param_name, param_type in meta['params']:
                stubs.append(f"---@param {param_name} {param_type}")

        if 'return' in meta:
            stubs.append(f"---@return {meta['return']}")

        # Build function signature
        if 'params' in meta:
            param_str = ", ".join(p[0] for p in meta['params'])
            stubs.append(f"function {module}.{lua_name}({param_str}) end")
        else:
            stubs.append(f"function {module}.{lua_name}(...) end")

        stubs.append("")

    stubs.append("")

out = "\n".join(stubs)
Path("lua_stubs/lush.d.lua").write_text(out)
print("Wrote definitions for {} functions.".format(sum(len(funcs) for funcs in reg_mapping.values())))
print("Modules: {}".format(", ".join(sorted(reg_mapping.keys()))))
print("... to lua_stubs/lush.d.lua")