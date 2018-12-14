fn main() {
    iffer();
    iffer2();
    conditional_assignment();
}

fn iffer() {
    let number = 3;
    if number < 5 {
        println!("true")
    } else {
        println!("false")
    }
}

fn iffer2() {
    let number = 6;
    if number % 4 == 0 {
        println!("div by 4")
    } else if number % 3 == 0 {
        println!("div by 3")
    } else if number % 2 == 0 {
        println!("div by 2")
    }
}

fn conditional_assignment() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {}", number)
}
