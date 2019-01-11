fn main() {
    section_one();
    section_two();
    section_three();
}

// 1. Defining and instantiating Structs

fn section_one() {
    // define structs as follows
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    // instantiate them by letting vars = struct {}
    let mut user1 = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };
    // use dot notation to access fields
    println!(
        "user 1 email={} username={} active={} sign_in_count={}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );
    // use dot notation to set fields (when _mut_)
    user1.email = String::from("test2@test.com");
    println!("user email changed to {}", user1.email);

    // example of builder.
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    let mut user2 = build_user(String::from("test2@test.com"), String::from("test2"));
    println!(
        "user 2 email={} username={} active={} sign_in_count={}",
        user2.email, user2.username, user2.active, user2.sign_in_count
    );

    // update synyax
    let mut user3 = User {
        email: String::from("email@email.com"),
        ..user2
    };
    println!(
        "user 3 email={} username={} active={} sign_in_count={}",
        user3.email, user3.username, user3.active, user3.sign_in_count
    );

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 2. Example program using structs

fn section_two() {
    // simple example that ...works
    let width1 = 50;
    let height1 = 50;
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    println!(
        "the area of a rectangle {}x{} is {}",
        width1,
        height1,
        area(width1, height1)
    );

    // refactored to use structs
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    };
    fn area_from_rect(rect: &Rectangle) -> u32 {
        rect.height * rect.width
    }
    let r = Rectangle {
        width: 25,
        height: 25,
    };
    println!("rectangle from area_from_rect: {}", area_from_rect(&r));

    // derived traits

    println!("rectangle: {:#?}", r);
}

// 3. Method syntax on structs

fn section_three() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    };

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let rect = Rectangle {
        width: 30,
        height: 40,
    };

    println!("using Rectangle.area {}", rect.area());

    // methods with more parameters

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

    println!("rect1 can hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 ? {}", rect1.can_hold(&rect3));

    // associated functions

    let square = Rectangle::square(25);

    println!("square: {:#?}", square)
}
