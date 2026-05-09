# json

Methods for reading and writing JSON from files or strings.

## read_file

Reads a JSON file and converts it to Lua values.

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| path      | string | Path to JSON file. |

### Returns

Lua value (usually a table) representing the parsed JSON.

### Example

```lua
local data = json.read_file("data.json")
fmt.print("{}", data)
```

## read_string

Parses a JSON string into Lua values.

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| json_str  | string | JSON source string. |

### Returns

Lua value (usually a table).

### Example

```lua
local data = json.read_string('{"name":"lush","ok":true}')
```

## write_file

Serializes a Lua value to pretty JSON and writes it to a file.

### Parameters

| Parameter | Type   | Description |
| --------- | ------ | ----------- |
| path      | string | Destination file path. |
| value     | any    | Lua value to serialize. |

### Returns

None

### Example

```lua
json.write_file("./data.json", {
  user = "aria",
  id = 1
})
```

## write_string

Serializes a Lua value into a pretty-printed JSON string.

### Parameters

| Parameter | Type | Description |
| --------- | ---- | ----------- |
| value     | any  | Lua value to serialize. |

### Returns

JSON string.

### Example

```lua
local serialized = json.write_string({ user = "aria", id = 1 })
fmt.print("{}", serialized)
```

## Conversion Notes

- Lua arrays map to JSON arrays only when keys are contiguous numeric indices starting at `1`.
- Lua tables with string keys map to JSON objects.
- Unsupported Lua types (like functions/userdata) raise an error.
