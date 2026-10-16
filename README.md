# Lush

A super simple lua task runner. A tiny replacement for `make`.

## Table

Just incase this gets long.

- [Task running](#task-running)
- [Lua API](#lua-api)
- [License](#license)

## Task running

To run a task, simply define one in a lua file with `lush.task()`
(Lush looks for a `lush.lua` file in the current directory by default)

then you can run the task with `lush <task_name>` in the CLI.

You can pass arguments to the task with `--`. e.g; `lush <task_name> -- arg arg2 arg3`
If a task handler does not accept variadic arguments, it receives only the
number of arguments defined by its parameters.

```lua
-- lush echo -- meow meow mew
-- `mew` is ignored because the handler accepts only two arguments.
lush.task("echo", {}, function(one, two) end)
```

# Lua API

For specific details on each function listed here (like parameters or return values),
then please refer to the [definition file](./lush.d.lua)

## **lush.task**

Registers a task you can call from the CLI.

**Example**:

```lua
-- The args `...` collect arguments passed to the task with `--` in the CLI call.
lush.task("echo", {}, function(...)
    local args = table.concat({ ... }, " ")
    print(args)
end)
```

## **lush.exec**

Executes a command in the shell and returns its output.

```lua
lush.exec("cargo build")
```

## **lush.vexec**

Verbose version of exec that executes a command in the shell and returns the status code, stdout/err.

```lua
local status, stdout, stderr = lush.vexec("git rev-parse --short HEAD")

if status ~= 0 then
    error(("Error obtaining git commit hash: " .. stderr), status)
    os.exit(status)
end
```

## License

The license for the project can be viewed in the [LICENSE](./LICENSE) file.
