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

// $ cargo run
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
//      Running `target/debug/variables`
// Please enter an array index.
// 999

// thread 'main' panicked at src/main.rs:19:19:
// index out of bounds: the len is 5 but the index is 999
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
