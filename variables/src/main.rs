fn main() {
    /* 
    Rust’s naming convention for constants is to use all uppercase with underscores between words.
     The compiler is able to evaluate a limited set of operations at compile time,
      which lets us choose to write out this value in a way that’s easier to understand 
      and verify, rather than setting this constant to the value 10,800. 
      See the Rust Reference’s section on constant evaluation for more information on what 
      operations can be used when declaring constants.

    Constants are valid for the entire time a program runs, 
    within the scope they were declared in. This property makes constants useful for values 
    in your application domain that multiple parts of the program might need to know about,
     such as the maximum number of points any player of a game is allowed to earn or the speed 
     of light.

Naming hardcoded values used throughout your program as constants is useful in conveying 
the meaning of that value to future maintainers of the code. It also helps to have only one place
 in your code you would need to change if the hardcoded value needed to be updated in the future.
    */
    // Constants are ALWAYS immutable and MUST be annotated 
    // declared using constant keyword
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Normal variables are immutable by default
    let test = 123;

    // Can be declared mutable with mut keyword
    // Tradeoff in performance must be kept in mind
    // For example, for data structures with instances
    // Mutable keyword makes sense sometimes rather than redeclaring
    // and re initializing the entire data structure.
    let mut test2 = 1234;

    // Shadowing a variable by reassigning it to a different value.
    let test1 = 123;

    let test1 = "abc";

    // Keep in mind that Rust is a statically typed language, 
    // which means that it must know the types of all variables at compile time.

    // Note that shadowing is different from mutating
    // You need to redeclare the variable using let for shadowing
    // For mutating just reassign the variable ( after declaration )

    let mut space = "   ";

    // Example of mutating.
    // Error operation Mutable types doesn't mean you can mutate variable types
    space = space.len();

    /* 
    A scalar type represents a single value. 
    Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    */

}
