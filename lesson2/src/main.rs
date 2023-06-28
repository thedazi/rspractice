//! Lesson 2 is mainly about creating a cargo workspace, which is how rust
//! manages data organization and connects files.
//! In order to create a cargo workplace, first the command prompt needs to be
//! navigated the location, and then input 'cargo new [workplace name]'.
//! After this >cd [workplace name] then 'cargo build', which will COMPILE everything
//! This will create a src (containing the main.rs to code in), .gitignore 
//! (which tells Git which files to ignore), Cargo.lock (which contains exact 
//! information about dependencies. THIS IS MAINTAINED BY CARGO AND DON'T NEED 
//! TO BE MANUALLY EDITED), and Cargo.toml (containing programmers description
//! of dependencies broadly, and is written by the editor themself)
//! All of the .exe and debugging files will be found in the target folder.
//! To run/debug, command prompt (CP) should be > cd target > cd debug then ran normally
//! To shorten this, the cp can be navigated to the cargo workplace and input
//! 'cargo run' and this will also output the results
//! 'cargo check' can be used to run the program for errors
//! Lastly, use 'cargo fmt' to format all files or if navigated to the src file,
//!  the command rustfmt [file.rs] can be used to automatically format the code properly.

fn main() {
    let x = 4;
    println!("x is: {}", x);
    /* variables are auto-immutable, so they cannot be redefined*/ 
    let mut y = 10;
    println!("y is: {}", y); 
    y = 13;
    println!("y is: {}", y);
    /* command 'mut' makes the variables redefinable (like in python) */
    let x = x + 3;
    println!("x is: {} (should be 7)", x);
    /* to get around the immutable attribute in order to reiterate variables
    if the 'let' command is used about to redefine, then the previous usage of the
    variable will be used within the newly defined variable, which can be a new type */
    {
        let y = 2;
        println!("y is: {}", y);
        let x = x + 3;
        println!("x is: {} (should be 10)", x)
        /* due to the bracket, this cell is putting this variable into a different
        section than the other variables, and therefore can share the name 'y' with
        another outside of this scope, but at the same time, can lend from variables
        in the exterior scope as well. This is called name shadowing */
    }

    const SECONDS_IN_MINUTE: i32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
    /* unlike variables, constants cannot be redefined under any circumstance */

}
