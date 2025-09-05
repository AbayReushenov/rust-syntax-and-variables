fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    // s1 = hello, s2 = hello
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    // s1 = hello, s2 = hello
    // x = 5, y = 5

    let s01 = String::from("hello");
    let s02 = s01;

    // println!("{s01}, world!"); // borrow of moved
    println!("{s02}, world!");

    let s001 = "hello";
    let s002 = s001;

    println!("{s001}, world!");
    println!("{s002}, world!");
}

// s1 = hello, s2 = hello
// x = 5, y = 5
// hello, world!
// hello, world!
// hello, world!
