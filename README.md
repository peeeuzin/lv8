# 🚀 LV8
LV8 (pronounced "levieight") is interpreted dynamically typed programming language. The language is heavily inspired by Lua, Python and Elixir.

# Features
- [x] ~~Variables~~
- [x] ~~Functions~~
- [x] ~~Function calls~~
- [x] ~~Math expressions~~
- [x] ~~Comments~~
- [x] ~~Modules~~
- [x] ~~Imports~~
- [ ] Flow control (if, else, while, for) (Currently only `if` statement is available)
- [ ] Standard library (Currently only `print` function is available)
- [ ] Garbage collection
- [ ] Error handling

# 🚨 Warning
This language is not meant to be used in production, it is just a fun project to learn how to make a programming language.

# 🔗 Contributing
If you would like to contribute to this project, feel free to fork the repository and make a pull request. I am open to any suggestions or improvements.

# Getting Started
Currently, the project is not published to any package manager, so you will need to clone the repository and build the project yourself.

## Prerequisites
- **Rust (1.76.0)**

## Installation
1. Clone the repository
```bash
git clone https://github.com/peeeuzin/lv8
```

2. Build LV8
```bash
cd lv8
cargo install --path lv8
```

3. Run the project
```bash
lv8 examples/hello_world.lv
```

# Examples
## 👋 Hello World
```lv8
fun main() do
  print("Hello, World!")
end

main()
```

run: `lv8 examples/hello_world.lv`

## 🧮 Math Expression
```lv8
fun calculate_math(a, b, c) do
  return a * (b + c)
end

print("2 * (3 + 4) =", calculate_math(2, 3, 4))
```

run: `lv8 examples/math_expression.lv`

# LV8 Syntax

## Variables
Variables in LV8 are dynamically typed, which means you don't need to specify the type of the variable when declaring it.
```lv8
my_variable = 10
my_variable = "Hello, World!"

# You can also declare multiple variables in one line
value1, value2 = "Hello!"
```

## Functions
Functions in LV8 are defined using the `fun` keyword, followed by the function name and the parameters. The function body is defined using the `do` keyword, and ended with the `end` keyword.
```lv8
fun my_function() do
  print("Hello, World!")
end

my_function()

# You can also pass parameters to the function

fun calculate_sum(a, b) do
  return a + b
end

print(calculate_sum(2, 3))
```

## Modules
You can declare a module using `module` keyword, followed by the module name. The module body is defined using the `do` keyword, and ended with the `end` keyword.
```lv8
module Human do
  my_name = ""
  age = 0

  fun declare_name(new_name) do
    my_name = new_name
  end

  fun increase_age() do 
    age = age + 1
  end
end

Human.increase_age()
Human.declare_name("Pedro")

# or

Human.my_name = "John"
```

## Imports
To import another file you can use `import`, followed by file path and the module name.

```lv8
# math.lv8

fun complex_calculation(num1, num2) do
  return num1 + num2
end
```

```lv8
import "math.lv8" as Math

Math.complex_calculation(1, 2)
```


# 📜 License
This project is licensed under the MIT License. For more information, please read the [LICENSE](LICENSE) file.