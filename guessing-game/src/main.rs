use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

    println!("Guess the number");
   

    let secret_number = rand::thread_rng().gen_range(1..101);

   
    loop {

    println!("Please input your guess");
    let mut guess = String::new();


    

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    
    // Parsing and annotating string type variable to u32
    // 
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,

    };
    
    println!("You guessed: {}",guess);

    /*
    A match expression is made up of arms. An arm consists of a pattern and the code 
    that should be run if the value given to the beginning of the match expression 
    fits that arm’s pattern. Rust takes the value given to match and looks through 
    each arm’s pattern in turn. The match construct and patterns are powerful features 
    in Rust that let you express a variety of situations your code might encounter 
    and make sure that you handle them all. 

    */
  
    /* 
        Switching from an expect call to a match expression is one way of moving from crashing
         on an error to handling the error. Remember that parse returns a Result type and Result is an enum that has the variants Ok or Err.
         We’re using a match expression here, as we did with the Ordering result of the cmp method.
    */
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("Correct guess! You win");
            break;
}

    }

}

}