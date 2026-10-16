local lush = {}
-- Definition file for Lush's functions.
-- Mostly likely wouldnt be needed by an LSP, mostly here for help with documentation.

--- Differs from `lush.exec` as it doesnt return anything, and passes any output directly to you. Use `lush.vexec` if you want to handle status code or stdout/err.
--- @param name string Name of the task to create/run
--- @param depends_on string[] List of tasks to run before this one
--- @param handler function Function to run when the task is called in the cli
--- @return nil
function lush.task(name, depends_on, handler) end

--- @param cmd string Command to run in the shell. On windows it chooses `cmd`. On unix it chooses whatever is defined in `$SHELL` (`sh` if none)
--- @return nil
function lush.exec(cmd) end

--- Differs from `lush.exec` as it explicitly returns status code, stdout/err output to be handled by you.
--- @param cmd string Command to run in the shell. On windows it chooses `cmd`. On unix it chooses whatever is defined in `$SHELL` (`sh` if none)
--- @return number status
--- @return string stdout
--- @return string stderr
--- @nodiscard
function lush.vexec(cmd) return 0, "", "" end
