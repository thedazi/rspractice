//! Rust uses the os command prompt as its main terminal.
//! Continuing in this regard, rust files need to be confiled within the
//! terminal before creating a runnable .exe file (or pdb file for mac/linux)
//! for basics, command prompt needs to directed to the file location with
//! cd (or change desination) till it is where at the .rs file is located
//! from that point the command rustc [filename.rs] can be used to produce
//! .exe file. From that point, the .exe is ran from .\[filename.exe] for output
//! For all of this to work, MS Visual Studio Build tools with C++ build tools
//! needs to be installed or else a RUST error link.exe not found will be raised

fn main() {
    println!("hello world")
}