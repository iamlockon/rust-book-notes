use rand::Rng; // Use library crate
use std::{io, cmp::Ordering}; // we can use `self` in the braces for nested path, e.g. {self, xxx}

const MAX_GUESS: u32 = 101; // const must be annotated, numeric literals can use underscore to improve readability.

fn main() {
    println!("Guess the number!"); // Exclamation mark denotes that this println is a macro
    // Chapter1~3
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // Declar&define mutable variable from associative method new of String.
        io::stdin()
            .read_line(&mut guess) // Tell read_line() that `guess` here is mutable
            .expect("Failed to read line"); // If success, expect will have Ok as Result, otherwise Err with provided message
        
        let guess: u32 = match guess.trim().parse() { // Annotate the type for type conversion, then use match expression to
            Ok(num) => num,                           // match the Result with arms like Ok, Err. Note that guess is shadowed.
            Err(e) => {                               // And you don't need to declare it as `mut` to shadow it, just `let` will do.
                println!("Error: {}, plz try again.", e);
                continue;
            }
        };
        println!("You guessed: {}", guess); // Format string with curly bracket
        if guess == 5 {
            println!("You guess 5");
        }
        match guess.cmp(&rand::thread_rng().gen_range(1, MAX_GUESS+1)) { // If guess = 55, rand = 32, will match Greater.
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Good");
                break;
            },
            _ => () // `_` is placeholder, `()` is unit value.
        }
    }

    let mut ctr = 0;
    let a = [1,2,4,5,6];
    
    while ctr < 10 {
        ctr += 1;
    }
    
    for ele in a.iter() { // Safe even when array length changes
        println!("{}", ele);
    }

    for num in (1..4).rev() {
        println!("{}", num);
    }
}
