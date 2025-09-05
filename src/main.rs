fn main() {
    let reference_to_nothing = dangle();

    println!("Test {}", reference_to_nothing)
}


// this function's return type contains a borrowed value, but there is no value for it to be borrowed from

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
