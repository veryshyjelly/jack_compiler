# Jack Compiler

## Introduction

This project is an implementation of the Jack Compiler, written in Rust. The Jack Compiler translates high-level Jack programming language code into VM code, which can then be translated into Hack assembly language and executed on the Hack computer described in the "From NAND to Tetris" course.

## Jack Language Grammar

The Jack programming language is a simple, object-oriented language. The grammar of the Jack language includes the following elements:

- **Class**: The blueprint for creating objects, containing fields and methods.
- **Subroutines**: Methods, functions, and constructors within a class.
- **Statements**: Instructions that control the flow of execution.
    - `let`: Assigns a value to a variable.
    - `if`: Conditional execution.
    - `while`: Looping construct.
    - `do`: Calls a subroutine.
    - `return`: Returns a value from a subroutine.
- **Expressions**: Combinations of variables, constants, and operators that produce a value.
- **Terminals**: The basic elements such as identifiers, constants, and keywords.

### Jack Language Contract

1. **Class Declarations**:
    ```jack
    class ClassName {
        field int x, y;
        static boolean flag;

        constructor ClassName new() {
            // constructor code
        }

        function void compute() {
            // function code
        }

        method int getValue() {
            return x;
        }
    }
    ```

2. **Subroutine Declarations**:
    - Constructor, function, and method declarations.
    - Syntax:
      ```jack
      constructor Type new() { statements }
      function Type functionName() { statements }
      method Type methodName() { statements }
      ```

3. **Variable Declarations**:
    - Declares local or field variables.
    - Syntax:
      ```jack
      var int a, b;
      ```

4. **Statements**:
    - `let`, `if`, `while`, `do`, and `return`.
    - Syntax examples:
      ```jack
      let x = 5;
      if (x > 0) { do Output.printInt(x); }
      while (x > 0) { let x = x - 1; }
      do Output.printString("Hello, World!");
      return x;
      ```

5. **Expressions**:
    - Combinations of variables, constants, and operators.
    - Syntax examples:
      ```jack
      let y = x + 3;
      if (x < 5) { let y = y * 2; }
      ```

## Features of the Jack Compiler

- **Lexical Analysis**: Tokenizes Jack source code into a stream of tokens.
- **Syntax Analysis**: Parses the token stream according to the Jack grammar.
- **Code Generation**: Translates the parsed syntax tree into VM code.
- **Error Handling**: Provides informative error messages for syntax and semantic errors.
- **User-Friendly CLI**: Easy-to-use command-line interface for compiling `.jack` files or directories containing Jack files.

## Installation

To use this compiler, you'll need to have Rust installed on your machine. If you don't have Rust installed, you can get it [here](https://www.rust-lang.org/tools/install).

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/yourusername/jack-compiler.git
cd jack-compiler
```

Build the project using Cargo:

```sh
cargo build --release
```

The executable will be located in the `target/release` directory.

## Usage

To compile a Jack file or directory containing Jack files, run the following command:

```sh
./jack-compiler path/to/yourfile.jack
```

or

```sh
./jack-compiler path/to/yourdirectory
```

This will generate the corresponding VM files in the same directory as the input file(s).

## Example

Given the following Jack code in `Main.jack`:

```jack
class Main {
    function void main() {
        do Output.printString("Hello, World!");
        return;
    }
}
```

Running the compiler:

```sh
./jack-compiler Main.jack
```

Will produce the following `Main.vm` file:

```vm
function Main.main 0
push constant 0
call Output.printString 1
pop temp 0
push constant 0
return
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The creators of the "From NAND to Tetris" course, Noam Nisan and Shimon Schocken, for providing the framework and inspiration for this project.
- The Rust community for their excellent documentation and support.
