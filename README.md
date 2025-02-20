```

# Aurascript Macro

Aurascript Macro is a Rust procedural macro library that provides custom macros to generate functions and control their execution order without allowing the user to define an entry point function (`main`). This ensures a clean and controlled execution flow.

## Features

- **Custom Macros**: Generate functions with custom messages.
- **Controlled Execution**: Use the `execute` macro to control the order of function execution.
- **No User-defined Entry Point**: The user cannot define an entry point function, ensuring a consistent execution flow.

## Macros

### `print`

Generates a function that prints a custom message.

```rust
print!("Hello, Macro");
```
This will generate:

```rust
fn print_message() {
    println!("Hello, Macro!");
}
```

### `greet`

Generates a function that prints a greeting message.

```rust
greet!("World");
```

This will generate:

```rust
fn greet() {
    println!("Hello, World!");
}
```

### `farewell`

Generates a function that prints a farewell message.

```rust
farewell!("everyone");
```

This will generate:

```rust
fn farewell_message() {
    println!("Goodbye, everyone!");
}
```

### `thank`

Generates a function that prints a thank you message.

```rust
thank!("Rust");
```

This will generate:

```rust
fn thank_message() {
    println!("Thank you, Rust!");
}
```

### `execute`

Controls the entry point and the order of function execution. The `execute` macro takes a list of function names as strings and generates a `main` function that calls these functions in the specified order.

```rust
execute!("print", "greet");
```

This will generate:

```rust
fn main() {
    print_message();
    greet();
}
```

## Usage

1. Add the `aurascript_macro` crate to your `Cargo.toml`:

    ```toml
    [dependencies]
    aurascript_macro = { path = "../aurascript_macro" }
    ```

2. Use the macros in your `main.rs` file:

    ```rust
    use aurascript_macro::{print, greet, farewell, thank, execute};

    // Define the functions
    print!("Hello, Macro");
    greet!("World");
    farewell!("everyone");
    thank!("Rust");

    // Execute specific functions in order
    execute!("print", "greet", "farewell", "thank");
    ```

## Example

Here is a complete example of how to use the Aurascript Macro library:

```rust
use aurascript_macro::{print, greet, farewell, thank, execute};

// Define the functions
print!("Hello, Macro");
greet!("World");
farewell!("everyone");
thank!("Rust");

// Execute specific functions in order
execute!("print", "greet", "farewell", "thank");
```

This will generate a `main` function that calls the generated functions in the specified order:

```rust
fn main() {
    print_message();
    greet();
    farewell_message();
    thank_message();
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgements

Special thanks to the Rust community for their support and contributions.