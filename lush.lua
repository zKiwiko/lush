lush.task("cargo:install", function(where)
    sys.exec("cp ./target/release/lush " .. where)
end)

lush.task("cargo:fix", function()
    sys.exec("cargo fix --bin \"lush\" -p lush")
end)

lush.task("book:build", function()
    sys.exec("mdbook build book")
end)

lush.task("echo", function(...)
    local args = table.concat({ ... }, " ")
    fmt.print("{}", str.trim(args))
end)
