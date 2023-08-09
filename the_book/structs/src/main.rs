struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // tuple struct
struct Point(i32, i32, i32); // tuple struct

struct AlwaysEqual; // unit-like struct

fn create_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // uses Field Init shorthand
        email, // uses Field Init shorthand
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1name"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    println!("user1.active == {}", user1.active);
    println!("user1.username == {}", user1.username);
    println!("user1.email == {}", user1.email);
    println!("user1.sign_in_count == {}", user1.sign_in_count);

    user1.email = String::from("user1newemail@example.com");
    println!("user1.email == {}", user1.email);

    let mut user2 = create_user(String::from("user2@example.com"), String::from("user2name"));

    println!("user2.active == {}", user1.active);
    println!("user2.username == {}", user1.username);
    println!("user2.email == {}", user1.email);
    println!("user2.sign_in_count == {}", user1.sign_in_count);    

    user2.email = String::from("user2newemail@example.com");

    println!("user2.email == {}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}
