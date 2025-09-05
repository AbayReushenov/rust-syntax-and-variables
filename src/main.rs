fn first_word(s:  &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);

    println!(" ТЕСТ {}", word);

    let word = first_word(&my_string[..]);

    println!(" ТЕСТ 2 {}", word);// `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);
    println!(" ТЕСТ 3 {}", word);
    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    println!(" ТЕСТ 4 {}", word);
    let word = first_word(&my_string_literal[..]);
    println!(" ТЕСТ 5 {}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!(" ТЕСТ 6 {}", word);
}


// ТЕСТ hello
// ТЕСТ 2 hello
// ТЕСТ 3 hello
// ТЕСТ 4 hello
// ТЕСТ 5 hello
// ТЕСТ 6 hello
