# LV8
LV8 (pronounced "levieight") is a simple programming language that I am making for fun. It is a dynamically typed language that is interpreted. The language is heavily inspired by Lua, Python and Elixir.

# Warning
This language is not meant to be used in production, it is just a fun project to learn how to make a programming language.

# Contributing
If you would like to contribute to this project, feel free to fork the repository and make a pull request. I am open to any suggestions or improvements.

# Examples
## Hello World
```lv8
fun main() do
  print("Hello, World!")
end

main()
```

run: `cargo run examples/hello_world.lv`

## Math Expression
```lv8
fun calculate_math(a, b, c) do
    return a * (b + c)
end

print("2 * (3 + 4) =", calculate_math(2, 3, 4))
```

run: `cargo run examples/math_expression.lv`