# lush

Core runtime methods for task registration.

## task

Registers a named task.

### Signatures

```lua
lush.task(name, handler)
lush.task(name, opts, handler)
```

### Parameters

| Parameter | Type | Description |
| --------- | ---- | ----------- |
| name      | string | Task name used on CLI (`lush <name>`). |
| opts      | table/nil | Optional dependency table. Can be `{"dep1", "dep2"}` or `{ depends = "dep1" }` or `{ depends = {"dep1", "dep2"} }`. |
| handler   | function | Task body to execute. |

### Returns

None

### Example

```lua
lush.task("build", function()
  sys.exec("gcc main.c -o main")
end)

lush.task("run", { depends = { "build" } }, function()
  sys.exec("./main")
end)
```

## rule

Placeholder API (currently no-op).

### Signature

```lua
lush.rule(output, input, handler)
```

## target

Placeholder API (currently no-op).

### Signature

```lua
lush.target(files, opts)
```

## Constants

### VERSION

Current Lush version string.
