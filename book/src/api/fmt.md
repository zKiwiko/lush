# fmt - Format

`fmt` provides formatting helpers, path joining, and numeric base conversion.

## print

Prints a formatted string to stdout.

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| template  | string | Format string using `{}` placeholders. |
| ...       | any    | Values inserted into placeholders in order. |

### Returns

None

### Notes

- `{{` prints `{` and `}}` prints `}`.
- Returns a runtime error if placeholder count and argument count do not match.

### Example

```lua
fmt.print("Value: {}", 42)
fmt.print("Braces: {{}}")
```

## string

Returns a formatted string (same formatting rules as `fmt.print`).

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| template  | string | Format string using `{}` placeholders. |
| ...       | any    | Values inserted into placeholders in order. |

### Returns

Formatted string.

### Example

```lua
local msg = fmt.string("{} + {} = {}", 1, 2, 3)
```

## path_join

Joins path segments with `/` and removes duplicate surrounding slashes.

### Parameters

| Parameter | Type                 | Description |
| --------- | -------------------- | ----------- |
| ...       | string/integer/number | Path segments to join. |

### Returns

Joined path string.

### Example

```lua
local p = fmt.path_join("/usr/", "local", "bin")
-- "usr/local/bin"
```

## to_hex

Converts a number to uppercase hexadecimal with `0x` prefix.

### Parameters

| Parameter | Type           | Description |
| --------- | -------------- | ----------- |
| value     | integer/number | Value to convert. |

### Returns

Hex string.

### Example

```lua
fmt.print("{}", fmt.to_hex(255)) -- 0xFF
```

## to_bin

Converts a number to binary with `0b` prefix.

### Parameters

| Parameter | Type           | Description |
| --------- | -------------- | ----------- |
| value     | integer/number | Value to convert. |

### Returns

Binary string.

### Example

```lua
fmt.print("{}", fmt.to_bin(10)) -- 0b1010
```

## to_oct

Converts a number to octal with `0o` prefix.

### Parameters

| Parameter | Type           | Description |
| --------- | -------------- | ----------- |
| value     | integer/number | Value to convert. |

### Returns

Octal string.

### Example

```lua
fmt.print("{}", fmt.to_oct(8)) -- 0o10
```
