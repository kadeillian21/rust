// * the variable 's' is only currently valid within the 'variable_scope()' function.  Once the function has ran, 's' is out of scope.
fn variable_scope() {
    let s = "scopy";
    println!("{}", s);
}

fn strings() {
    // * The commented out code below throws an error because we cannot mutate a string literal.  In order to mutate a string, we have to use type 'String::from'
    // let string_literal = "s";
    // let mango = string_literal + "mango";
    // println!("{}", mango)
    // * Using type 'String::from' allows us to edit a string with the adition of the 'mut' keyword
    let mut string = String::from("mangossss");
    string.push_str(" are delicious");
    println!("{}", string);
    // * The difference between string literals and strings in Rust is that string_literals are stored on the stack because we know the contents at compile time.  A generic string's data is stored on the heap.
}

// ! A move in Rust is the concept of copying only the information of a variable, not the content itself (merely a pointer to the content)
fn moves_in_rust() {
    // * Below is an example of a shallow copy, a memory allocation pattern that Rust takes advantage of frequently.  S1 is stored completely on the heap, but s2 is stored on the stack because it only contains a pointer to s1, it's capactity,  and it's length (size).  When s2 is created, s1 goes out of scope automatically so as to avoid a 'double free' error.  This makes this example an example of a MOVE because Rust automatically causes the original copy, s1, to go out of scope, MOVING the necessary data to s2, which is valid until it is out of scope.
    let s1 = String::from("this is a shallow copy of data, a move");
    let s2 = s1;
    println! {"{}", s2};
}

// ! A clone in Rust is the concept of not merely using a 'move' action, but actually creating a 'deep copy' of the data, not only copying the information about the content, but the content itself
fn clones_in_rust() {
    let s1 = String::from("this is a deep copy of data, a clone");
    // * Clone() not only copies the pointer, capacity, and length of a piece of memory, but also the contents of it as well, causing for s1 here to not go out of scope.
    let s2 = s1.clone();
    println! {"s1 = {}, s2 = {}", s1, s2};
}

// ! Copies in Rust works because data that is stored on the stack, such as integers, booleans, floating points, chars, and any tuple that only consists of the afformentioned types, have a known size at compile time.  This allows us to copy the data in the stack, allowing for variables to not go out of scope.
fn copies_in_rust() {
    let x = 5;
    let y = x;
    // * Variable 'x' is still considered valid because its contents are known completely on the stack, allowing for us to copy it.
    println!("x = {}, y = {}", x, y);
}

fn main() {
    // * The commented out code below throws an error because variable 's', from the function 'variable_scope()', has already gone out of scope.
    // println!("{}", s);
    variable_scope();
    strings();
    moves_in_rust();
    clones_in_rust();
    copies_in_rust();
}
