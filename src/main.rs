fn main() {
    // tuple - кортеж;
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of x is: {x}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    // tuple indexing by position
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");
}
// The value of y is: 6.4
// The value of z is: 1
// The value of x is: 500
// The value of five_hundred is: 500
// The value of six_point_four is: 6.4
// The value of one is: 1
