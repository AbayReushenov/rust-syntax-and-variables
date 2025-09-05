// В любой момент времени у вас может быть одна (но не обе) изменяемая ссылка или любое количество неизменяемых ссылок.
// Все ссылки должны быть действительными.

fn main() {
    let string = no_dangle();

    println!("Test {}", string)
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// Test hello
