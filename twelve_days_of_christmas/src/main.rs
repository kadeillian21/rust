fn tweleve_days_of_christmas() {
    const CHRISTMAS_ITEMS: &[&str] = &[
        "a partridge in a pear tree.",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese-a-laying",
        "seven swans-a-swimming",
        "eight maids-a-milking",
        "nine ladies dancing",
        "ten lords-a-leeping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    const PRIMIAL_NUMBERS: &[&str] = &[
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelvth",
    ];

    for number in PRIMIAL_NUMBERS {
        println!(
            "On the {} day of Christmas, my true love gave to me...",
            number
        );
        for item in CHRISTMAS_ITEMS {
            println!("{}", item);
        }
    }
}

fn main() {
    tweleve_days_of_christmas();
}
