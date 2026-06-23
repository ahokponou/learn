# Guessing Game

[Working project directory](../projects/guessinggame)

## What do I learn in this program?

- read user input
- variable declaration 
- store value in a variable

## Variable declaration

```rust
let number = 5; // Declare a variable called "number" and assign value 5
```

Note: In rust, variable are immutable by default. That means if a variable have a value it won't change.

The line below declare mutable variable

```rust
let mut number = 5;
```

## Assign a type to variable

The line below create a mutable variable and assign an empty String as type.

```rust
let mut name = String::new();
```

## Receiving user input

We import a standard library: 

```rust 
use std::io;
```

And call the io::stdin to handle user input.
Note: We can call function io::stdin without import it like this

```rust
std::io::stdin()
```

The method .read_line() with argument &mut guess as describe below 

```rust
io::stdin()
    .read_line(&mut guess)
```

handle to get input from user. 

Note: & indicates that the argument is a reference. Like variable reference are immutable by default.

The line 

```rust 
    .expect("Failed to read line");
```

is a method that handle error in case of result failure.
Note: handle error is necessary

## Use println! placeholder to print variable value

```rust
let name = "Kondo";
println! ("Hi, I'm {name}"); // Print: Hi, I'm Kondo
println! ("Hi, I'm {}", name); // Also print: Hi, I'm Kondo
```

## Use crate to get more functionnality


