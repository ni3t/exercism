fn main() {
    println!("Hello, world!");
    another_function();
    a_third_func(5);
    multi_args(6, 7);
    expressions();
    let five = five();
    println!("return value is {}", five);
    let x = 5;
    println!("{} plus_one {}", x, plus_one(x))
}

fn another_function() {
    println!("Another function.")
}

fn a_third_func(x: i32) {
    println!("third func called with {}", x)
}

fn multi_args(x: i32, y: i32) {
    println!("multi_args called with x {} and y {}", x, y)
}

fn expressions() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x is {} y is {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
