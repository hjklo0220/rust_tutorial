struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("jun"),
        email: String::from("jun@test.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("jun@example.com");
    println!("{}", user1.email);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("test@test.com"),
        sign_in_count: user1.sign_in_count
    };

    // let user3 = User {
    //     email: String::from("user3@test.com"),
    //     ..user1
    // };
    // println!("{}", user1.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);

    let subject = AlwaysEqual;
    

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}