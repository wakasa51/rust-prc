fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let tup = (1, "str", 3444444);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    let a = [1, 2, 3];
    println!("a: {:?}", a);
}
