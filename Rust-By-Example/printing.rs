
fn main() {

    // Prints to standard console without newline io::stdout
print!(" Doesn't append a new line");
print!("{} Cheeky",21);

// Formatting using positional arguments / Named arguments
print!(" {car} {colour} {model} ",car="Tesla", colour="blue",model="S");
print!("{0} This is {1}. This is {1}","Alice","Bob");

// Special formatting ( replaces it with binary of number)
println!("{} of {:b} people know binary, the other half don't",1,2);

// Padding number with extra zeroes
// Outputs 0000001
println!("{number:0>width$}",number=1,width=6);

println!("Pi is roughly {0:.3}",3.141592);

println!("Testing out debug {0:?}",123);

#[allow(dead_code)]

struct Structure(i32);

// println!("Struct won't print by itself",Structure(3))
}

