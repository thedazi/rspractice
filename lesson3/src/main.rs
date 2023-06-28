//!Primitive Data Types in Rust
//! bool(een) = false can be 0, and true can be 1
//! char(acater)
//! str(ing)
//! i8-i64(128) ((signed) integers)
//! isize
//! u8-u64(128) (unsigned integers, cannot go below zero, but smaller)
//! usize
//! f32, or single precision.           8  expon, precision doesn't matter, less resources, useful for games
//! f64, or double precision (DEFAULT). 11 expon, precision matters, accurate but more resources, useful for sci. calc
//! array, or a fixed-sizearray, denoted[T;N], for the element type T, and the non-
//! negative compile-time constant size, N.
//! slice, dynamically-sized view into a contiguous sequence
//! tuple, a finite heterogeneous sequence

//!Crates/libraries and Modules are accessed with the 'use' command, while '::' is used to bridge crates to specific
//! modules/functions.
//! 'std' or standard library (really bad naming here) is one of the more generalized libraries, and inputs use
//! the io, or input/output module. SO useful!

use std::io;
fn main() {
    let t = true;
    let f = false;
    println!("t is {} and f is {}", t, f);

    let a = 'a';
    let b = 'b';
    println!("{} is for apple and {} is for banana.", a, b);

    let x = -540;
    let y: u32 = 31;
    println!("integers can be positive and negative like {}, but unsigned intergers can only be positive like {}", x, y);
    /* the u and i both work in terms of bianary, therefore, depending on the size on bits,
    higher numbers can be used
    EX) u8 -> 0 ~ 2^8       : numbers    0 ~ 255 
        i8 -> -2^7 ~ 2^7 - 1: numbers -128 ~ 127 */

    let _floating_point = 10.92;

    let tup = (1, true, 's');
    /* cannot be printed normally, but instead with index */
    println!("this tuple ends with {}, and has {} {} value.", tup.2, tup.0, tup.1);

    let _arr = [1, 2, 3, 4, 5];
    /* arrays need to be singular type and elements cannot be added to arrays  */

    println!("Hello, world! Now you say hello back!");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);

}
