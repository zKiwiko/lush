# Overview

Lush is a lightweight modernized Lua runtime written in Rust, configured entirely in Lua. It gives the Lua language a modern API to allow its capabilities to soar in potential.
Its API is still in development, its structure & features may minimally or drastically.

Lush is - although - intended to be a Lua based task runner. By default, Lush uses [LuaJIT](https://luajit.org/) for maximum preformance and its C interop. To use other versions of Lua
(e.g `5.4`), you will have to build it with the corresponding `mlua` flag.

# Command options

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

## C File Compilation (Build API)

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
                    :compiler(build.c.GCC) -- Default
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
