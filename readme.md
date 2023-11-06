# Yafika-Script
Yafika-script is an implementation of an interpreter for my own scripting language written in Rust.

This is a project for me to learn more about programming languages and how they work, this is just a toy language I will use for learning, and hopefully one day I can use it to write some software for myself.

Engaging in this project has brought me immense joy and satisfaction in my programming endeavors.

## Syntax

```
// Variable declaration 
let x = 123 + 2 * 6
let y = "hello world"

// Functions
let add = fn(a,b) { return a + b}
add(5,2)

// if else 
if (2 > 1) {
  return 5
} else {
  return 4
}


```

## TODO:
- [x] Lexer

- [x] Parser

- [ ] Evaluator

- [ ] Error handling

- [ ] Performance monitoring and improvements

- [ ] Compiler

## Learning resources
[Crafting Interpreters by Robert Nystrom](https://craftinginterpreters.com/ 'Crafting interpeters')

[Writing An Interpreter In Go by Thorsten Ball](https://interpreterbook.com/ 'Writing an interpreter in Go')

## License

[MIT](https://choosealicense.com/licenses/mit/)
