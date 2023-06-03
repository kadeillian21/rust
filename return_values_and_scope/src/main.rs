fn main() {
    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("give back a string");

    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    // ! Return multiple values using a tuple
    let string1 = String::from("tuple string moment");
    let (string2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", string2, len);
}

fn gives_ownership() -> String {
    let some_string = String::from(
        "this function gives ownership of the returned 'some_string' to the function that calls it",
    );
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
