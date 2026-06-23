# Rust CheatSheet

## Start a new project

```shell 
cargo new <project name> # Create a directory and init the project
```

```shell
# If directory exist
# Move inside the directory
# Run
cargo init 
```

## Basic syntax

- Macro that print string to screen

```rust
println! ("Text to display");
```

- Variable declaration

```rust
let number = 5; // immutable => cannot change
let mut number = 5; // mutable => can change
```

- Get user input

```rust
let mut name = String::new(); 
std::io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");
```


