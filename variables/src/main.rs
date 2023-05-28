fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // In Rust, the convention is to UPCASE constant variable names and to seperate words with an underscore
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // All about shadowing
    let y = 31;
    println!("This be y: {}", y);
    let y = 420;
    println!("This be y: {}", y);

    // Shadowing can be used to reassign the type of a variable
    let mango = "    ";
    let mango_space = mango.len();
    println!(
        "This be a mango {} and a mango_space {}",
        mango, mango_space
    );

    // Data Types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("This is guess: {}", guess);

    // ! ALL ABOUT INTEGER DATA TYPES!
    // u8: 8-but unsigned integer, range: 0 to 255

    // i8: 8-but signed integer, range: -128 to 127

    // u16: 16-bit unsigned integer, range: 0 to 65,535

    // i16: 16-bit signed integer, range: -32,768 to 32,767

    // u32: 32-bit unsigned integer, range: 0 to 4,294,967,295

    // i32: 32-bit signed integer, range: -2,147,483,648 to 2,147,483,647

    // u64: 64-bit unsigned integer, range: 0 to 18,446,744,073,709,551,615

    // i64: 64-bit signed integer, range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

    // usize: Architecture-dependent unsigned integer, range varies based on the architecture of the system. On 64-bit systems, it can hold values up to 18,446,744,073,709,551,615.

    // isize: Architecture-dependent signed integer, range varies based on the architecture of the system. On 64-bit systems, it ranges from -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807.

    // ? The size of a signed integer is this equation: -(2^(n-1))-1 to (2^(n-1))-1

    // ? The size of an unsigned interger is this equation: 0 to (2^(n))-1

    // ! ALL ABOUT FLOATS
    // Rust supports either f32 or f64, which is the number of bits in the float

    // ! Rust Math
    // addition
    let sum: u8 = 5 + 10;
    println!("Sum is: {}", sum);

    // subtraction
    let difference: u8 = 15 - 10;
    println!("Difference is: {}", difference);

    // multiplication
    let product: u8 = 15 * 10;
    println!("Product is: {}", product);

    // division
    let quotient: u8 = 20 / 10;
    println!("Quotient is: {}", quotient);

    // remainder
    let remainder: u8 = 43 % 5;
    println!("Remainder is: {}", remainder);

    // ! Yikes, Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _y, _z) = tup;
    println!("Tuple moment: {}", x);

    // a bit of dot notation pattern matching
    let a: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = a.0;
    let _six_point_four = a.1;
    let _one = a.2;
    println!("Five Hundred: {}", five_hundred);

    // ! Array and Vector moment
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    println!("Array index 0: {}", first)
}
