fn main() {
    let mut s = String::from("hello");
    println!("Наш пример {}", s);

    // не может быть других ссылок на это же значение.
    // Код, который пытается создать две изменяемые ссылки на s,
    // завершится ошибкой:
    change(&mut s);

    println!("Наш пример {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Наш пример hello
// Наш пример hello, world
