# ExportBranch

`ExportBranch` is a command-line tool used in conjunction with `Compex` to compile programs written in the Harbour programming language. The primary functionality of this tool is to filter files and characters within branches before `Compex` begins compiling the project.

### License

This project is licensed under the [**MIT License**](LICENSE)

### Building

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository
```
git clone https://github.com/MateusZanchoNeto/exportbranch.git
```
3. Navigate to the project directory
```
cd exportbranch
```
4. Build the project
```
cargo build --release
```
5. Run
```
./target/release/exportbranch
```