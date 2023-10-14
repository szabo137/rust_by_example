// Debug


// This structure cannot be printed either with `fmt::Display` or
// with``fmt::Debug`
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);


// Derive the `fmt::Debug` implementation for `Structure`. 
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable 
// via the debug channel, too.
#[derive(Debug)]
struct Deep(Structure);


// One can `derive` for even more complicate4d structures:
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // Printing over the debug channel is done using '{:?}' instead of '{}'.
    println!("This is printable {:?}",DebugPrintable(3));
    

    // In general, printing with '{:?}' intriduces more verbosity
    println!("{:?} months in a year.",12);

    // `Structure` is printable, too!
    println!("{:?} is printable!",Structure(3));

    // The problem with `derive` is there is no control over how the result look.
    // What if I want this to just show a '7'.
    println!("{:?} is printable, too.", Deep(Structure(3)));

   // Lets print more complicated structs which derived from `Debug`:
   let name = "Peter";
   let age = 12;
   let peter = Person {name, age};

   println!("{:#?}", peter)
}
