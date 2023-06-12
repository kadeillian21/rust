struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
fn main() {
    // this is the lame way to instantiate an object in Rust... let's get a bit more functional with the build_user function call
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = build_user(
        String::from("email@params.com"),
        String::from("hiIAmAUsername"),
    );
    user1.email = String::from("anotheremail@example.com");

    let user3 = User {
        email: String::from("ThisIsHowYouUpdateAnObjectInRust@gmail.com"),
        ..user2
    };

    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    println!("{}", user3.email);
}
