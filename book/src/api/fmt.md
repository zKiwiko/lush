# fmt - Format

`fmt` provides different formatting options for strings and various logging methods to help you debug and display information.

Formatting logic and syntax is solely inspired by Rust.

## print

Provides standard output logging with Rust-style formatting.

### Parameters

| Parameter | Type   | Description                            |
| --------- | ------ | -------------------------------------- |
| format    | string | Format string with `{}` placeholders   |
| ...       | any    | Values to substitute into placeholders |

### Returns

None

### Example

```lua
fmt.print("Value: {}", 42)
fmt.print("Multiple: {} and {}", "first", "second")
```
