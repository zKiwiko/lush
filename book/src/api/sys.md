# sys

System and shell helpers.

## exec

Executes a shell command.

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| command   | string | Command to run. Must not be empty. |

### Returns

None

## getenv

Gets an environment variable value.

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| var       | string | Environment variable name. |

### Returns

String value, or runtime error if missing.

## setenv

Sets an environment variable.

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| var       | string | Environment variable name. |
| value     | string | Value to set. |

### Returns

None

## find

Checks whether a path exists and matches a requested type.

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| what      | number | `sys.FILE`, `sys.DIRECTORY`, or `sys.SYMLINK`. |
| name      | string | Path to check. |

### Returns

`true` if path exists and matches type, else `false`.

## mkdir

Creates a directory recursively.

## rm

Removes a file or directory recursively.

## cp

Copies a file from `src` to `dst`.

## mv

Renames or moves path from `src` to `dst`.

## cwd

Returns current working directory as a string.

## envs

Returns a table of all environment variables.

## os

Returns OS name string.

## arch

Returns architecture string.

## which

Returns executable path for `command`, or runtime error if not found.

## grep

Regex search over multiline text.

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| pattern   | string | Regex pattern. |
| text      | string | Input text. |

### Returns

Array table of matching lines.

## popen

Runs command and returns captured stdout as a string.

## Constants

- `sys.FILE` = `0`
- `sys.DIRECTORY` = `1`
- `sys.SYMLINK` = `2`

## Example

```lua
local out = sys.popen("echo hello")
fmt.print("{}", out)

if sys.find(sys.FILE, "lush.lua") then
  fmt.print("found lush.lua")
end
```
