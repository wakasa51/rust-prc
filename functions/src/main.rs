fn main() {
    let x = another_function(5, 6);
    println!("x is {:?}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value is {}", x);
    println!("The value is {}", y)
}
