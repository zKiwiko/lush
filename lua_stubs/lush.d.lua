---@meta
-- fmt module
---@class fmt
fmt = {}
--- Joins multiple path segments into a single path string, ensuring that there are no duplicate slashes between segments.
---@return string Result The joined path string.
function fmt.path_join(...) end

--- Prints a formatted string to the console. The first argument is a template string that can contain placeholders in the form of `{}`, and the subsequent arguments are the values to be formatted into the template. The function formats the string by replacing the placeholders with the provided arguments and then prints the resulting string to the console.
---@return nil
function fmt.print(...) end

--- Formats a string using a template and a variable number of arguments. The first argument is the template string, and the subsequent arguments are the values to be formatted into the template. The template can contain placeholders in the form of `{}` which will be replaced by the corresponding arguments in order.
---@return string Result The formatted string resulting from replacing the placeholders in the template with the provided arguments.
function fmt.string(...) end

--- Converts a number to its binary representation, prefixed with "0b".
---@param value integer|number The number to convert to binary.
---@return string Result
function fmt.to_bin(value) end

--- Converts a number to its hexadecimal representation, prefixed with "0x".
---@param value integer|number The number to convert to hexadecimal.
---@return string Result
function fmt.to_hex(value) end

--- Converts a number to its octal representation, prefixed with "0o".
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


-- str module
---@class str
str = {}
--- Splits a string into a table of substrings based on the specified separator. If the separator is an empty string, it defaults to splitting on whitespace.
---@param string string
---@param sep string
---@return table
function str.split(string, sep) end

--- Trims leading and trailing whitespace from the input string.
---@param string string
---@return string
function str.trim(string) end


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

