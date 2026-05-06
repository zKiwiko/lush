# Overview

Lush is a lightweight task runner and pseudo-build tool written in Rust, configured entirely in Lua (JIT). It combines the simplicity of shell scripting with the power and flexibility of a real programming language.

Think of it as **Make meets Lua** — you get task dependencies, a lua runtime with modern features, and native scripting without the (truly) cryptic syntax or limitations of traditional build systems.

# Building

```bash
# Clone the Repo
git clone https://github.com/zkiwiko/lush

# Cd into it
cd lush

# Build
cargo build --release

# install (optional)
cp ./target/release/lush /usr/bin/lush
```

# Quick Examples

## C File Compilation

**main.c**

```cpp
#include <stdio.h>

int main(int argc, char* argv[]) {
    printf("Hello, World!\n");
    return 0;
}
```

**lush.lua**

```lua
lush.task("build", function()
    local result = build.c()
                    :compiler(build.COMPILER.GCC) -- Default
                    :optimize(build.c.OPTIMIZE.O2)
                    :files( {"main.c"} )   -- Glob pattern
                    :output("main")
                    :run()

    if result.success then
        print(result.output)
    else
        print(result.error)
    end
end)

lush.task("run", { "build" }, function()
    sys.exec("./main")
end)

```

Use with: `lush run`

Expected Output:
`Hello, World!`
