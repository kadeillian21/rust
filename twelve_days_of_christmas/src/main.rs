fn tweleve_days_of_christmas() {
    let christmas_items: &[&str] = &[
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

    for (i, number) in PRIMIAL_NUMBERS.iter().enumerate() {
        println!(
            "On the {} day of Christmas, my true love gave to me...",
            number
        );
        for item in (0..=i).rev() {
            println!("{}", christmas_items[item]);
        }
    }
}

fn main() {
    tweleve_days_of_christmas();
}
