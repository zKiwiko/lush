---@meta
-- fmt module
---@class fmt
fmt = {}
---@desc Format bytes into a more readable format (e.g., 1024 becomes "1 KB").
---@param bytes integer|number The number of bytes to format.
---@return string The formatted byte size as a human-readable string.
function fmt.bytes(bytes) end

---@desc Pads the input value on the left with the specified fill string until it reaches the desired width. If the input value is already wider than or equal to the specified width, it is returned unchanged. The function accepts a value of any type (string, integer, or number), a target width as an integer, and an optional fill string (defaulting to a single space if not provided). The function returns a new string with the input value padded on the left to achieve the specified width.
---@param value any
---@param width integer
---@param fill? string
---@return string
function fmt.pad_left(value, width, fill) end

---@desc Pads the input value on the right with the specified fill string until it reaches the desired width. If the input value is already wider than or equal to the specified width, it is returned unchanged. The function accepts a value of any type (string, integer, or number), a target width as an integer, and an optional fill string (defaulting to a single space if not provided). The function returns a new string with the input value padded on the right to achieve the specified width.
---@param value any
---@param width integer
---@param fill? string
---@return string
function fmt.pad_right(value, width, fill) end

---@desc Joins multiple path segments into a single path string, ensuring that there are no duplicate slashes between segments.
---@return string Result The joined path string.
function fmt.path_join(...) end

---@desc Prints a formatted string to the console. The first argument is a template string that can contain placeholders in the form of `{}`, and the subsequent arguments are the values to be formatted into the template. The function formats the string by replacing the placeholders with the provided arguments and then prints the resulting string to the console.
---@return nil
function fmt.print(...) end

---@desc Prints a formatted string to the console. The first argument is a template string that can contain placeholders in the form of `{}`, and the subsequent arguments are the values to be formatted into the template. The function formats the string by replacing the placeholders with the provided arguments and then prints the resulting string to the console.
---@return nil
function fmt.println(...) end

---@desc Formats a string using a template and a variable number of arguments. The first argument is the template string, and the subsequent arguments are the values to be formatted into the template. The template can contain placeholders in the form of `{}` which will be replaced by the corresponding arguments in order.
---@return string Result The formatted string resulting from replacing the placeholders in the template with the provided arguments.
function fmt.string(...) end

---@desc Format time into a more readable format (ms)
---@param time integer|number The time in milliseconds to format.
---@return string The formatted time as a human-readable string.
function fmt.time(time) end

---@desc Converts a number to its binary representation, prefixed with "0b".
---@param value integer|number The number to convert to binary.
---@return string Result
function fmt.to_bin(value) end

---@desc Converts a number to its hexadecimal representation, prefixed with "0x".
---@param value integer|number The number to convert to hexadecimal.
---@return string Result
function fmt.to_hex(value) end

---@desc Converts a number to its octal representation, prefixed with "0o".
---@param value integer|number The number to convert to octal.
---@return string Result
function fmt.to_oct(value) end


-- json module
---@class json
json = {}
---@desc Reads a JSON file from disk and converts it into native Lua values.
---@param path string Path to the JSON file.
---@return any Parsed Lua value (typically a table).
function json.read_file(path) end

---@desc Parses a JSON string and converts it into native Lua values.
---@param json_str string JSON source string.
---@return any Parsed Lua value (typically a table).
function json.read_string(json_str) end

---@desc Serializes a Lua value to pretty-printed JSON and writes it to a file.
---@param path string Destination file path.
---@param value any Lua value to serialize.
---@return nil
function json.write_file(path, value) end

---@desc Serializes a Lua value into a pretty-printed JSON string.
---@param value any Lua value to serialize.
---@return string JSON output string.
function json.write_string(value) end


-- lush module
---@class lush
lush = {}
function lush.rule(...) end

function lush.target(...) end

---@desc Register a new task for Lush to execute.
---@param name string Name of the task. This will be used to execute it.
---@param depends? table Execute other tasks before this one.
---@param handler function The function to execute for this task.
---@overload fun(name: string, handler: function)
function lush.task(name, depends, handler) end


-- math module
---@class math
math = {}
---@desc Clamp a number between a minimum and maximum value.
---@param value integer|number The number to clamp.
---@param min integer|number The minimum value to clamp to.
---@param max integer|number The maximum value to clamp to.
---@return number Clamped_value The clamped value.
function math.clamp(value, min, max) end

---@desc Linearly interpolates between two values based on a parameter t.
---@param value integer|number The starting value.
---@param target integer|number The target value to interpolate towards.
---@param t integer|number The interpolation factor (0.0 to 1.0). A value of 0.0 will return the starting value, while a value of 1.0 will return the target value.
---@return number Interpolated_value The result of the linear interpolation between the starting value and the target value based on the interpolation factor t.
function math.lerp(value, target, t) end

---@desc Calculates the mean (average) of a table of numbers. The function accepts a single argument, which must be a table containing numeric values. It iterates through the values in the table, sums them up, and divides by the count of values to compute the mean. If the table is empty or contains non-numeric values, an error is returned.
---@param values table A table containing numeric values for which to calculate the mean. The table can be an array-like table (with integer keys starting from 1) or a table with arbitrary keys, as long as the values are numeric (integers or numbers).
---@return number Mean The calculated mean (average) of the numeric values in the table. The result is returned as a
function math.mean(values) end

---@desc Calculates the median of a table of numbers. The function accepts a single argument, which must be a table containing numeric values. It collects the values from the table, sorts them, and then computes the median based on whether the count of values is odd or even. If the table is empty or contains non-numeric values, an error is returned.
---@param values table A table containing numeric values for which to calculate the median. The table can be an array-like table (with integer keys starting from 1) or a table with arbitrary keys, as long as the values are numeric (integers or numbers).
---@return number Median The calculated median of the numeric values in the table.
function math.median(values) end

---@desc Returns the sign of a number. The function accepts a single numeric argument and returns -1 if the number is negative, 1 if the number is positive, and 0 if the number is zero.
---@param value integer|number The number for which to determine the sign. The function accepts any numeric value, including integers and floating-point numbers. The sign is determined based on whether the value is negative, positive, or zero.
---@return number Sign The sign of the input number. The function returns -1 if the input value is negative, 1 if the input value is positive, and 0 if the input value is zero. The result is returned as a number (integer or floating-point) depending on the input type, but it will always be one of the three possible values: -1, 0, or 1.
function math.sign(value) end


-- string module
---@class string
string = {}
---@desc Splits a string into a table of substrings based on the specified separator. If the separator is an empty string, it defaults to splitting on whitespace.
---@param string string
---@param seperator string
---@return table
function string.split(string, seperator) end

---@desc Trims leading and trailing whitespace from the input string.
---@param string string
---@return string
function string.trim(string) end


-- sys module
---@class sys
sys = {}
---@desc Returns the CPU architecture name.
---@return string Architecture name (for example `x86_64` or `aarch64`).
function sys.arch(...) end

---@desc Copies a file from one path to another.
---@param src string Source file path.
---@param dst string Destination file path.
---@return nil
function sys.cp(src, dst) end

---@desc Returns the current working directory.
---@return string Absolute path of the current working directory.
function sys.cwd(...) end

---@desc Returns a table containing all current environment variables.
---@return table A key-value table of environment variables.
function sys.envs(...) end

---@desc Executes a shell command and streams output to the terminal.
---@param command string Command to run. Must not be empty.
---@return nil
function sys.exec(command) end

---@desc Checks whether a path exists and matches a requested type.
---@param what integer One of `sys.FILE`, `sys.DIRECTORY`, or `sys.SYMLINK`.
---@param name string Path to test.
---@return boolean True if the path exists and matches the requested type, otherwise false.
function sys.find(what, name) end

---@desc Gets the value of an environment variable.
---@param var string Environment variable name.
---@return string The environment variable value.
function sys.getenv(var) end

---@desc Performs regex matching over multiline text and returns matching lines.
---@param pattern string Regular expression pattern.
---@param text string Input text to search.
---@return table Array-like table containing matching lines.
function sys.grep(pattern, text) end

---@desc Creates a directory and any missing parent directories.
---@param path string Directory path to create.
---@return nil
function sys.mkdir(path) end

---@desc Renames or moves a file or directory.
---@param src string Source path.
---@param dst string Destination path.
---@return nil
function sys.mv(src, dst) end

---@desc Returns the operating system name.
---@return string OS name (for example `linux`, `macos`, or `windows`).
function sys.os(...) end

---@desc Executes a shell command and returns captured standard output.
---@param command string Command to run.
---@return string Captured standard output.
function sys.popen(command) end

---@desc Removes a file or directory recursively.
---@param path string Path to remove.
---@return nil
function sys.rm(path) end

---@desc Sets an environment variable for the current process.
---@param var string Environment variable name.
---@param value string Environment variable value.
---@return nil
function sys.setenv(var, value) end

---@desc Returns the size in bytes of a Lua string or numeric value.
---@param value any A Lua string, integer, or number.
---@return integer Size in bytes for the provided value.
function sys.sizeof(value) end

---@desc Finds an executable in the system PATH.
---@param command string Executable name to search for.
---@return string Absolute path to the executable.
function sys.which(command) end


-- build module
---@class build
build = {}

--- Language constructor for C
---@return table build_task
function build.c() end

-- C constants
---@diagnostic disable: inject-field
---@desc Clang compiler
build.c.CLANG = 2

---@desc GCC compiler
build.c.GCC = 1

---@desc Use CMake generator
build.c.GENERATOR = build.c.GENERATOR or {}
build.c.GENERATOR.CMAKE = "cmake"

---@desc Use Ninja generator
build.c.GENERATOR.NINJA = "ninja"

---@desc Use raw compiler invocation (no generator)
build.c.GENERATOR.RAW = "raw"

---@desc G++ compiler
build.c.GXX = 0

---@desc C language mode
build.c.LANGUAGE = build.c.LANGUAGE or {}
build.c.LANGUAGE.C = "c"

---@desc C++ language mode
build.c.LANGUAGE.CPP = "cpp"

---@desc Objective-C language mode
build.c.LANGUAGE.OBJC = "objc"

---@desc Optimization level O0 - no optimization
build.c.OPTIMIZE = build.c.OPTIMIZE or {}
build.c.OPTIMIZE.O0 = "O0"

---@desc Optimization level O1 - minimize size
build.c.OPTIMIZE.O1 = "O1"

---@desc Optimization level O2 - optimize
build.c.OPTIMIZE.O2 = "O2"

---@desc Optimization level O3 - maximize performance
build.c.OPTIMIZE.O3 = "O3"

---@desc Optimization level Os - optimize for size
build.c.OPTIMIZE.OS = "Os"

---@desc Optimization level Oz - aggressively optimize for size
build.c.OPTIMIZE.OZ = "Oz"

---@desc C11 standard
build.c.STD = build.c.STD or {}
build.c.STD.C11 = "c11"

---@desc C17 standard
build.c.STD.C17 = "c17"

---@desc C2X standard (upcoming C standard)
build.c.STD.C2X = "c2x"

---@desc C89 standard
build.c.STD.C89 = "c89"

---@desc C99 standard
build.c.STD.C99 = "c99"

---@desc C++03 standard
build.c.STD.CXX03 = "c++03"

---@desc C++11 standard
build.c.STD.CXX11 = "c++11"

---@desc C++14 standard
build.c.STD.CXX14 = "c++14"

---@desc C++17 standard
build.c.STD.CXX17 = "c++17"

---@desc C++20 standard
build.c.STD.CXX20 = "c++20"

---@desc C++23 standard
build.c.STD.CXX23 = "c++23"

---@desc C++98 standard
build.c.STD.CXX98 = "c++98"

---@desc All warnings (-Wall)
build.c.WARNINGS = build.c.WARNINGS or {}
build.c.WARNINGS.ALL = "Wall"

---@desc Extra warnings (-Wextra)
build.c.WARNINGS.EXTRA = "Wextra"

---@desc No warnings
build.c.WARNINGS.NONE = ""

---@desc Normal warnings (-Wall)
build.c.WARNINGS.NORMAL = "Wall"

---@desc Pedantic warnings (-pedantic)
build.c.WARNINGS.PEDANTIC = "pedantic"

---@diagnostic enable: inject-field
