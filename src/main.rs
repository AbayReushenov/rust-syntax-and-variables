fn main() {
    let string = no_dangle();

    println!("Test {}", string)
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// Test hello
