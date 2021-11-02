use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

    println!("Guess the number");
    println!("Please input your number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Your secret number is {}",secret_number);


    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}",guess);

    /*
    A match expression is made up of arms. An arm consists of a pattern and the code 
    that should be run if the value given to the beginning of the match expression 
    fits that arm’s pattern. Rust takes the value given to match and looks through 
    each arm’s pattern in turn. The match construct and patterns are powerful features 
    in Rust that let you express a variety of situations your code might encounter 
    and make sure that you handle them all. 

    */
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Correct guess! You win"),

    }
}