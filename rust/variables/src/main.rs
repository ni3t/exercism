fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 2;
    println!("The value of x is {}", x);
    let x = x + 2;
    println!("The value of x is {}", x);
    let x = x % 2;
    println!("The value of x is {}", x);
    let tup: (i32, i32) = (24, 24);
    println!("The value of tup is {:?}", tup);
    println!("The value of tup 0 is {} and tup 1 is {}", tup.0, tup.1);
    let (y, z) = tup;
    println!("The value of tup y is {} and tup z is {}", y, z);
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    println!("the tenth item is {}", a[index]);
}
