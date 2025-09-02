use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
// Корректный доступ к элементу массива
// $ cargo run
//    Compiling variables v0.1.0 (/home/aaaaa/rust/variables)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
//      Running `target/debug/variables`
// Please enter an array index.
// 4
// The value of the element at index 4 is: 5

// Некорректный доступ к элементу массива
// $ cargo run
//    Compiling variables v0.1.0 (/home/aaaaa/rust/variables)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
//      Running `target/debug/variables`
// Please enter an array index.
// 99

// thread 'main' panicked at src/main.rs:19:19:
// index out of bounds: the len is 5 but the index is 99
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
