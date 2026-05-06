# lush

Provides core methods for Lush's functionality.

## task

Create and register a task to be called.

### Parameters

| Parameter  | Type               | Description                                                                          |
| ---------- | ------------------ | ------------------------------------------------------------------------------------ |
| Name       | string             | Sets the name of the task.                                                           |
| Depends On | string array       | Provide a list of tasks to be ran _before_ this one does. This parameter is optional |
| Code       | Function or Method | The code to be ran when the task is called.                                          |

### Returns

Whatever is returned by the `Code` parameter.

### Example

```lua

lush.task("build", function()
    sys.exec("gcc main.c -o main")
end)

lush.task("run", {"build"}, function() -- Runs "build" first, then runs its body.
    sys.exec("./main)
end)
```

Run with: `lush build` or `lush run`
