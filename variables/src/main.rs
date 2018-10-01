fn main() {
    //  Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //   Constants
    const MY_CONST: u32 = 100_00;
    println!("MY_CONT = {}", MY_CONST);
    
    //   Shadowing
    let spaces = "   ";
    let spaces = spaces.len();

    println!("Len = {}", spaces);
}
