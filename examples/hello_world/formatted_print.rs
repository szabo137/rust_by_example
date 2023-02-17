fn main() {
    // In general, the '{}' will be automatically replaced with any 
    // arguemnt. These will be stringified.
    println!("{} days.", 31);

    // Positional arguments can be used. Specifying an integer inside '{}'
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the firmat string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    
    // As can named arguments
    println!("{subject} {verb} {object}.",
             object="the lazy dog",
             subject="The quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after ':'
    println!("Base 10:               {}"  , 69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    println!("Base 16 (hexadecimal): {:X}", 69420);

    // You can right-justify text with a specified width. This will output
    // four whitespaces and a '1', a total of width 5:
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeros, 
    // and left-adjust by flipping the sign. This will output '10000':
    println!("{number:0<5}", number=1);

    // You can use named arguments in the format specifier by appending a '$'
    //println!("number:0<width$",number=1,width=5);
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks if the number of arguments is right:
    
    println!("My name is {0}, {1} {0}.", "Bond","James");
}
