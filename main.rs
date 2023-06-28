//extern crate is used to pull in crates, these dependencies are verified in 'Cargo.toml'

extern crate rand;

use std::io;
//std io = standard library input/output
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    loop{
    println!("Input a guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        //match can also be used to error proof
        Ok(num) => num,
        Err(_) => continue,
    };

                //Use shadowing to convert types without needing to create unique variables 
                //.trim will remove spaces, /n, and characters from strings
                //.parse will turn a string into an integer

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        // match takes the variable.cpm and compares it to the (&variable)
        // best to be sure that both variables are integers (use of shadowing is good here)
        // cmp method will return the three orderings, which compares the matches
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win!");
            break;
            //break and if statements can also be made with scope
        }
        }
    }
}

             
                
                