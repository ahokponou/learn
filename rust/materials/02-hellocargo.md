# Hello, Cargo!

[Working folder](../projects)

Cargo is rust build system and package manager. Cargo handles a lot of tracks in a rust project such as building code, downloading the libraries and building those libraries.

To check if cargo is installed: 

```shell
cargo --version
```

## Creating a project with cargo

```shell
cargo new hellocargo # Create a new project called "hellocargo"
cd hellocargo # Move to directory "hellocargo"
```

## Base project structure

[Working directory](../projects/hellocargo)

cargo generates two files and one directory as describes below.

```
hellocargo/ 
|- Cargo.toml
|- src/      
|- |- main.rs
```

- cargo.toml: TOML (Tom's Obvious, Minimal Language) format in cargo configuration format
- src/main.rs: cargo generate a hello world program

## Build a cargo project

```shell
cargo build
```

Generates one more file and one directory. 

## Run the program

```shell
./target/debug/hellocargo
```
 
## Build and run

 ```shell
 cargo run
 ```

## Check cargo project without produce executable

```shell
cargo check
```
