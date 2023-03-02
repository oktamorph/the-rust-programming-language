fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("First: {}, Second: {}, Third: {}", five_hundred, six_point_four, one);

    // Array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let b = [3, 5]; // let b = [3, 3, 3, 3, 3];    
}
