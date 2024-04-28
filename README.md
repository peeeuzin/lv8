# 🚀 LV8
LV8 (pronounced "levieight") is interpreted dynamically typed programming language. The language is heavily inspired by Lua, Python and Elixir.

# Features
- [x] ~~Variables~~
- [x] ~~Functions~~
- [x] ~~Function calls~~
- [x] ~~Math expressions~~
- [x] ~~Comments~~
- [ ] Standard library (Currently only `print` function is available)
- [ ] Flow control (if, else, while, for)
- [ ] Garbage collection
- [ ] Error handling
- [ ] Structs
- [ ] Enums 

# 🚨 Warning
This language is not meant to be used in production, it is just a fun project to learn how to make a programming language.

# 🔗 Contributing
If you would like to contribute to this project, feel free to fork the repository and make a pull request. I am open to any suggestions or improvements.

# ⬇️ Installation
Currently, the only way to install the language is from crates.io. You can install it using cargo.
## Cargo
```bash
cargo install lv8
```

# Examples
## 👋 Hello World
```lv8
fun main() do
  print("Hello, World!")
end

main()
```

run: `cargo run examples/hello_world.lv`

## 🧮 Math Expression
```lv8
fun calculate_math(a, b, c) do
  return a * (b + c)
end

print("2 * (3 + 4) =", calculate_math(2, 3, 4))
```

run: `cargo run examples/math_expression.lv`

# 📜 License
This project is licensed under the MIT License. For more information, please read the [LICENSE](LICENSE) file.