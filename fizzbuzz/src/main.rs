fn fizzbuzz() {
    for number in 1..100 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", number);
        }
    }
}

fn main() {
    fizzbuzz()
}
