lush.task("build", {}, function(...)
    lush.exec("cargo build")
end)

lush.task("echo", {}, function(...)
    local args = table.concat({ ... }, " ")
    print(args)
end)
