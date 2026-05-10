---@meta
-- fmt module
---@class fmt
fmt = {}
---@desc Format bytes into a readable format (e.g., 1024 becomes "1 KB").
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

---@desc Format time into a readable format (ms)
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
function json.read_file(...) end

function json.read_string(...) end

function json.write_file(...) end

function json.write_string(...) end


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
function sys.arch(...) end

function sys.cp(...) end

function sys.cwd(...) end

function sys.envs(...) end

function sys.exec(...) end

function sys.find(...) end

function sys.getenv(...) end

function sys.grep(...) end

function sys.mkdir(...) end

function sys.mv(...) end

function sys.os(...) end

function sys.popen(...) end

function sys.rm(...) end

function sys.setenv(...) end

function sys.which(...) end


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
