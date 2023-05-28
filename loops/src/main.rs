fn main() {
    let mut counter: i32 = 0;
    loop {
        println!("The value of counter is: {}", counter);
        if counter == 1000000 {
            println!("a milli a milli!");
            break;
        }
        counter += 10;
    }

    // while loop
    let mut number: i32 = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1
    }
    println!("LIFTOFF!");

    // for loop
    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("The value is: {}", element);
    }

    // for loop to count down to 🚀
    for number in (1..100).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
