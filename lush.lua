-- lush install <path>
---@param where string
---@return nil
lush.task("install", function(where)
    sys.exec("cp ./target/release/lush " .. where)
end)

lush.task("book:build", function()
    sys.exec("mdbook build book")
end)

lush.task("echo", function(...)
    local args = table.concat({ ... }, " ")
    print(args)
end)
