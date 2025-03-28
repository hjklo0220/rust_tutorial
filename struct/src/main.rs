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

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );
    println!("{}", rect1.height);

    println!("{:#?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect1.area());
    println!("{} {}", rect1.width, rect1.width());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}