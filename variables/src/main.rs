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

    control_flow();
}

/**
 * Data Types
 *
 *  Scalar -->
 *   1. Integers - u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, isize, usize
 *   2. Floating-point - f32, f64
 *   3. Boolean - bool
 *   4. Char - 'A' (Unicode Scalar values)
 *
 *  Compound -->
 *   1. Tuples - let tup: (i32, u64, char) = (1, 123, 'a');
 *   2. Arrays - let arr: [i32; 5] = [1, 2, 3, 4, 5];
 */

fn control_flow() {
    let number = 3;

    let res = if number != 0 { number } else { 0 };

    assert_eq!(number, 3);

    println!("res = {}", res);

    let a = [1, 2, 3];
    for elem in a.iter() {
        println!("{}", elem);
    }

    for number in 4..10 {
        println!("{}!", number);
    }
}
