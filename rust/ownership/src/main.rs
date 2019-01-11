// Rust Book Chapter 4 -- ownership
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
fn main() {
    string();
    variables();
    functions();
    return_values();
    tuples();
    tuples_borrowing();
    mutable_borrowing();
    slices();
}

// 1. What is Ownership?

fn string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn variables() {
    let s1 = String::from("hello");
    let s2 = s1;
    // this will fail to compile, since s2 shallow copies s1 and replaces it on the stack
    //println!("{}", s1);

    let s3 = s2.clone();
    println!("s2={}, s3={}", s2, s3);

    let x = 5;
    let y = x;
    // this works, because integers are a known size and allocated on the stack
    println!("x={}, y={}", x, y);

    // Rust won’t let us annotate a type with the Copy trait if the type,
    // or any of its parts, has implemented the Drop trait

    // generally - scalars can Copy, allocations cannot Copy
}

fn functions() {
    // Passing a variable to a function will move or copy, just as assignment does.
    let s = String::from("Hello");
    takes_ownership(s);
    // this will now fail
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    // this doesn't fail
    // println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn return_values() {
    // Returning values can also transfer ownership;
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn tuples() {
    let s1 = String::from("tuples");
    let (s2, len) = calculate_length(s1);
    println!("the length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// 2. References and borrowing

fn tuples_borrowing() {
    let s1 = String::from("tuples borrowing");
    let len = calculate_length_borrowing(&s1);
    println!("The length of '{}' is {}", s1, len)
}

fn calculate_length_borrowing(s: &String) -> usize {
    s.len()
}

fn mutable_borrowing() {
    // borrowed vars are immutable
    // use &mut syntax to mutate
    let mut s = String::from("mutable string");
    mutate_string(&mut s);
    println!("a mutated string: {}", s)
}

fn mutate_string(some_string: &mut String) {
    some_string.push_str(", mutated.");
}

// 3. Slice Type

fn slices() {
    let s = String::from("hello world!");
    let hello = &s[0..5];
    let world = &s[6..11];
    let hello_incl_space = &s[0..=5];
    let hello_from_start = &s[..5];
    let world_from_end = &s[6..];

    println!(
        "hello is '{}', world is '{}', hello_incl_space is '{}', hello_from_start is '{}', world_from_end is '{}'",
        hello, world, hello_incl_space, hello_from_start, world_from_end
    );
}
