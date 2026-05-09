# build

Build API for C-family targets. `build.c` is callable and returns a chainable task object.

## Quick Start

```lua
local result = build.c()
  :compiler(build.c.GCC)
  :language(build.c.LANGUAGE.C)
  :std(build.c.STD.C11)
  :files({ "src/*.c" })
  :include_dirs({ "include" })
  :defines({ "APP_NAME=\"lush\"" })
  :optimize(build.c.OPTIMIZE.O2)
  :warnings(build.c.WARNINGS.ALL)
  :output("app")
  :run()

fmt.print("{}", result)
```

## Chain Methods

All methods return the same task object.

### compiler(compiler)

Compiler constant: `build.c.GXX`, `build.c.GCC`, `build.c.CLANG`.

### language(language)

Language constant: `build.c.LANGUAGE.C`, `build.c.LANGUAGE.CPP`, `build.c.LANGUAGE.OBJC`.

### std(std)

Language standard constant, e.g. `build.c.STD.C11`, `build.c.STD.CXX20`.

### files(files)

Array of source file paths or glob patterns.

### output(name)

Output binary name. Defaults to `a.out`.

### optimize(level)

Optimization constant from `build.c.OPTIMIZE` (`O0`, `O1`, `O2`, `O3`, `OS`, `OZ`).

### debug(enabled)

Boolean flag for debug symbols (`-g` in raw mode).

### warnings(level)

Warning constant from `build.c.WARNINGS` (`NONE`, `NORMAL`, `ALL`, `EXTRA`, `PEDANTIC`).

### include_dirs(dirs)

Array of include directories.

### defines(defs)

Array of preprocessor defines (without `-D`).

### link_libs(libs)

Array of library names (without `-l`).

### frameworks(frameworks)

Array of framework names (used for Objective-C style linking).

### flags(flags)

Array of extra raw compiler flags.

### find_library(name)

Resolves a system library and appends include dirs/lib paths/link libs/flags to the task.

### generator(gen)

Build generator: `build.c.GENERATOR.RAW`, `build.c.GENERATOR.CMAKE`, `build.c.GENERATOR.NINJA`.

### generate()

Generates build files for the selected generator (`CMAKE` or `NINJA`).

### run()

Executes the build and returns a result table.

## Result Table

`generate()` and `run()` return a table with fields:

- `success` (boolean)
- `output` (string)
- `error` (string or nil)
- `exit_code` (number or nil)

## Constants

### Compiler

- `build.c.GXX`
- `build.c.GCC`
- `build.c.CLANG`

### `build.c.OPTIMIZE`

- `O0`, `O1`, `O2`, `O3`, `OS`, `OZ`

### `build.c.STD`

- C: `C89`, `C99`, `C11`, `C17`, `C2X`
- C++: `CXX98`, `CXX03`, `CXX11`, `CXX14`, `CXX17`, `CXX20`, `CXX23`

### `build.c.WARNINGS`

- `NONE`, `NORMAL`, `ALL`, `EXTRA`, `PEDANTIC`

### `build.c.GENERATOR`

- `RAW`, `CMAKE`, `NINJA`

### `build.c.LANGUAGE`

- `C`, `CPP`, `OBJC`
