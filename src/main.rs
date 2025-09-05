// Управляющие конструкции
// Возможности запуска некоторого кода в зависимости от некоторого условия, и циклического выполнения некоторого кода,
// являются базовыми элементами в большинстве языков программирования. Наиболее распространёнными конструкциями,
// позволяющими управлять потоком выполнения кода Rust, являются выражения if и циклы.
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Ошибка, Rust ожидал тип bool

    // let number_test_two = 3;

    // if number_test_two {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    let condition = true;
    let number_condition = if condition { 5 } else { 6 };

    println!("The value of number_condition is: {number_condition}");

    // let condition_2 = true;

    // let number_2 = if condition_2 { 5 } else { "six" };

    // Compiling variables v0.1.0 (/home/aaaaa/rust/variables)
    // error[E0308]: `if` and `else` have incompatible types
    //   --> src/main.rs:33:48
    //    |
    // 33 |     let number_2 = if condition_2 { 5 } else { "six" };
    //    |                                     -          ^^^^^ expected integer, found `&str`
    //    |                                     |
    //    |                                     expected because of this

    // For more information about this error, try `rustc --explain E0308`.
    // println!("The value of number is: {number_2}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
// condition was true
// number was something other than zero
