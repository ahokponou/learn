# Hello World

[Project folder](../projects/helloworld)

## Anatomy of rust program

[Reference file](../projects/helloworld/main.rs)


This define a function called 'main'. This function is special as it is always the first code that runs in every executable rust program.

```rust
fn main ()
{

}
```

The body of function 'main' contains the following code. It prints text "Hello, world!" to the screen. 

```rust
println!("Hello, world!");
```

### Compile a rust program

```shell
rustc main.rs
```

The command output a binary executable. The name of the executable is the name of source file without ".rs" To run it : 

```shell
./main
```
