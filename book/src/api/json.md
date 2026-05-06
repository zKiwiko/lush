# json

Provides different methods for interacting with JSON files or formats.

## read_file

Read a JSON file from the given path.

### Parameters

| Parameter | Type   | Description                         |
| --------- | ------ | ----------------------------------- |
| path      | string | The path of the `json` file to open |

### Returns

A table that matches the json data

### Example

**data.json**

```json
{
  "user": {
    "name": "",
    "id": 0
  }
}
```

**lush.lua**

```lua
local data = json.read_file("data.json")
fmt.print("Data: {}", data)
```

## read_string

Reads a supplied string as JSON data.

### Parameters

| Parameter | Type   | Description        |
| --------- | ------ | ------------------ |
| json_str  | string | The string to read |

### Returns

A table that matches the json data

### Example

**data.json**

```json
{
  "user": {
    "name": "",
    "id": 0
  }
}
```

## write_file

Write a Lua table as JSON data to a file.

### Parameters

| Parameter | Type   | Description                    |
| --------- | ------ | ------------------------------ |
| path      | string | The file path to write to      |
| data      | table  | The data to write to said path |

### Returns

None

### Example

```lua
local data = {
    user = "",
    id = 0
}

json.write_file("./data.json", data)
```

## write_string

Serialize a string with JSON data from a table.

### Parameters

| Parameter | Type  | Description                    |
| --------- | ----- | ------------------------------ |
| data      | table | The data to write to serialize |

### Returns

None

### Example

```lua
local data = {
    user = "",
    id = 0
}

local serialized = json.write_string(data)
```
