fn main() {
    // integer addition
    println!("1+2 = {}", 1u32 + 2);
    
    // integer substraction
    println!("1 - 2 = {}",1i32 - 2);

    // scientific noteation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // short-circuting boolian logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // bitwise operations
    // let rust infer that 0b0011 is actually 0b0011u23
    println!("0011 AND 0101 is {:04b}", 0b011 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b011 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b011 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // you can use underscores to improve readability
    println!("One million is written as {}",1_000_000u32);
}

