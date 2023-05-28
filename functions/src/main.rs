fn hello_function_function(mango: &str) {
    println!("Hello Functions!");
    println!("I am a passed parameter! {}", mango);
}

fn pass_integer_parameter(x: i32) {
    println!("I am an integer parameter! {}", x);
}

fn return_number_five_expression() -> u8 {
    5
}

fn main() {
    hello_function_function("This is a certified mango moment.");
    pass_integer_parameter(5);
    let return_expression = return_number_five_expression();
    println!(
        "This is the value of our returned expression: {}",
        return_expression
    );
}
