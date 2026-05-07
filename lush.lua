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

lush.task("c", function(...)
    fmt.print("Running C task with args: {}", table.concat({ ... }, " "))
    local result = build.c()
        :generator(build.c.GENERATOR.NINJA) -- Use Ninja generator
        :files({ "main.c" })                -- Glob pattern
        :output("main")
        :run()

    fmt.print("{}", result)

    if result.success then
        print(result.output)
    else
        print(result.error)
    end
end)
