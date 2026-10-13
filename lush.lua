lush.task("release", {}, function()
    lush.exec("cargo build --release")
end)

lush.task("install", { "release" }, function(where)
    lush.exec("cp ./target/release/lush " .. where)
end)

lush.task("build", {}, function(...)
    lush.exec("cargo build")
end)

lush.task("echo", {}, function(...)
    local args = table.concat({ ... }, " ")
    print(args)
end)
