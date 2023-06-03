fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // * The line below is invalid Rust because the argument of 'some_string', which 's' is assigned to, goes out of scope after the function is called.
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope

// return values and scope
