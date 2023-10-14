fn main() {
    // Variables can be type annotated
    let logical: bool = true;
    println!("logical: {x}",x=logical);

    let a_float: f64 = 1.0; // Regular annotation
    println!("a float: {}",a_float);
    let an_interger = 5i32; // suffix annotation
    println!("an integer: {}",an_interger);
    
    // or a default type will be used
    let default_float = 3.0; // `f64`
    println!("default float: {}",default_float);
    let default_integer = 7; // `i32` (!)
    println!("default integer: {}",default_integer);
    
    // A type can also be inferred from context
    let mut inferred_type = 12; // the type (=i64) will be inferred from the next line
    println!("inferred type: {}",inferred_type);
    inferred_type = 42i64;
    println!("inferred type: {}",inferred_type);

}
