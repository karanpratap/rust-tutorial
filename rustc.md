# Rustc compiler notes
## Compiling multiple files
```
cargo build --verbose
```
This will show you all the invocations of the rustc compiler.

To include other files' code and call the functions within those modules use the below syntax - no need to provide the name of all the constituent modules while compiling:
```
mod foo;

fn main() {
    // stuff
}
```
`foo.rs` has this:
```
pub fn hello() {
    // stuff again
}
```
To compile:
```
$ rustc main.rs
```

