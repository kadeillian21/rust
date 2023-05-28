fn main() {
    let number: u8 = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // ? Else If

    let mango: &str = "This is a mango moment";

    if mango == "This is a mango moment" {
        println!("Wow, this is epic");
    } else if mango == "pie" {
        println!("Pie moment");
    } else {
        println!("This input not valid");
    }

    // ! Using "if" in a "let" statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
