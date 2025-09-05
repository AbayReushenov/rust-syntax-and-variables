fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        println!("number {}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
// 3!
// number 3!
// 2!
// number 2!
// 1!
// number 1!
// LIFTOFF!!!
