fn main() {

    let mut user1 = User {
        email: String::from("john@example.com"),
        username: String::from("john123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("john123@example.com");

    let user2 = user_builder(String::from("john123@example.com"), String::from("john123"));

    let user3 = User {
        email: String::from("john_another@example.com"),
        username: user1.username, // value moved
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // let name = user1.username; value moved, so cannot use here
    let email = user1.email; // value do not moved, available.


    let user4 = User {
        email: String::from("john_another_2@example.com"),
        // ..user1 // moved value!
        ..user3
    };

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}


struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn user_builder(email: String, username: String) -> User {
    User {
        email,
        username, // field init shorthand
        active: true,
        sign_in_count: 1,
    }
}