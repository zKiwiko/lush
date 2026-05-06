# sys

The `sys` table provides shell-like functions and methods for system operations. Most commands mimic standard Unix utilities.

## exec

Executes a command with the user's preferred shell.

On Linux, this may be `zsh`, `bash`, `fish`, etc... \
On Windows, this will always target command prompt: `cmd`.

### Parameters

| Parameter | Type   | Description                                         |
| --------- | ------ | --------------------------------------------------- |
| command   | string | The command to be ran. Is always wrapped in quotes. |

### Returns

None

### Example

```lua
sys.exec("echo 'Hello World!'")
sys.exec("go run main.go")
```

## getenv

Retrieve an environment variable's value. Similar to `echo $VAR` in shell.

### Parameters

| Parameter | Type   | Description                                     |
| --------- | ------ | ----------------------------------------------- |
| key       | string | The name of the `key` to retrieve the value of. |

### Returns

Returns the `key`'s value as a string if found;\
Returns a `RuntimeError` if not.

### Example

```lua
local shell = sys.getenv("SHELL")
local home = sys.getenv("HOME")
```

## setenv

Set an environment variable's value.

### Parameters

| Parameter | Type   | Description                          |
| --------- | ------ | ------------------------------------ |
| var       | string | The name of the variable to set.     |
| value     | string | The value to assign to the variable. |

### Returns

None

### Example

```lua
sys.setenv("MY_VAR", "hello")
sys.setenv("PATH", "/usr/bin:/usr/local/bin")
```

## find

Find files or directories by type. Similar to the `find` command.

### Parameters

| Parameter | Type   | Description                                                       |
| --------- | ------ | ----------------------------------------------------------------- |
| what      | number | Type to search for: `sys.FILE`, `sys.DIRECTORY`, or `sys.SYMLINK` |
| name      | string | Pattern or name to search for.                                    |

### Returns

Returns `true` if found, `false` otherwise.

### Example

```lua
local files = sys.find(sys.FILE, "lush.lua")
local dirs = sys.find(sys.DIRECTORY, "src")
```

## mkdir

Create a directory. Similar to `mkdir -p` (creates parent directories as needed).

### Parameters

| Parameter | Type   | Description                  |
| --------- | ------ | ---------------------------- |
| path      | string | Path to directory to create. |

### Returns

None

### Example

```lua
sys.mkdir("build/output")
sys.mkdir("./tmp/nested/dirs")
```

## rm

Remove a file or directory. Similar to `rm -rf`.

### Parameters

| Parameter | Type   | Description                          |
| --------- | ------ | ------------------------------------ |
| path      | string | Path to file or directory to remove. |

### Returns

None

### Example

```lua
sys.rm("build")
sys.rm("output.o")
```

## cp

Copy a file or directory. Similar to `cp -r`.

### Parameters

| Parameter | Type   | Description       |
| --------- | ------ | ----------------- |
| src       | string | Source path.      |
| dst       | string | Destination path. |

### Returns

None

### Example

```lua
sys.cp("main.c", "main.c.bak")
sys.cp("src", "src_backup")
```

## mv

Move or rename a file or directory. Similar to `mv`.

### Parameters

| Parameter | Type   | Description       |
| --------- | ------ | ----------------- |
| src       | string | Source path.      |
| dst       | string | Destination path. |

### Returns

None

### Example

```lua
sys.mv("old_name.lua", "new_name.lua")
sys.mv("./output", "./build/output")
```

## pwd

Get the current working directory. Similar to `pwd`.

### Parameters

None

### Returns

Returns the current working directory as a string.

### Example

```lua
local current = sys.cwd()
print("Working in: " .. current)
```

## envs

Get all environment variables as a table.

### Parameters

None

### Returns

Returns a table with all environment variables as key-value pairs.

### Example

```lua
local all_env = sys.envs()
for key, value in pairs(all_env) do
    print(key .. " = " .. value)
end
```

## os

Get the operating system name.

### Parameters

None

### Returns

Returns the OS as a string: `"linux"`, `"windows"`, `"macos"`, etc.

### Example

```lua
local os_name = sys.os()
if os_name == "windows" then
    sys.exec("cls")
else
    sys.exec("clear")
end
```

## arch

Get the system architecture.

### Parameters

None

### Returns

Returns the architecture as a string: `"x86_64"`, `"aarch64"`, `"x86"`, etc.

### Example

```lua
local arch = sys.arch()
print("Running on: " .. arch)
```

## which

Find the full path to an executable. Similar to `which` command.

### Parameters

| Parameter | Type   | Description         |
| --------- | ------ | ------------------- |
| command   | string | Name of executable. |

### Returns

Returns the full path to the executable as a string, or `RuntimeError` if not found.

### Example

```lua
local gcc_path = sys.which("gcc")
local node_path = sys.which("node")
```

## grep

Search for text patterns in a string. Similar to the `grep` command.

### Parameters

| Parameter | Type   | Description                  |
| --------- | ------ | ---------------------------- |
| pattern   | string | Regex pattern to search for. |
| text      | string | Text to search in.           |

### Returns

Returns a table (array) of matching lines.

### Example

```lua
local content = sys.read("lush.lua")
local matches = sys.grep("task", content)
for i, line in ipairs(matches) do
    print(line)
end
```

## popen

Execute a command and capture its output. Similar to piping output.

### Parameters

| Parameter | Type   | Description         |
| --------- | ------ | ------------------- |
| command   | string | Command to execute. |

### Returns

Returns the command's output as a string.

### Example

```lua
local version = sys.popen("gcc --version")
print(version)
```

## Constants

### FILE

Constant used with `sys.find()` to search for files.

Real Value: `0`

```lua
local files = sys.find(sys.FILE, "*.rs")
```

### DIRECTORY

Constant used with `sys.find()` to search for directories.

Real Value: `1`

```lua
local dirs = sys.find(sys.DIRECTORY, "src")
```

### SYMLINK

Constant used with `sys.find()` to search for symbolic links.

Real Value: `2`

```lua
local links = sys.find(sys.SYMLINK, "*")
```

### VERSION

The Lush version string. Comes from `CARGO_PKG_VERSION` environment variable.

```lua
print("Lush version: " .. sys.VERSION)
```
