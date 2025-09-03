fn main() {
    another_function(56);
}

// В сигнатурах функций вы обязаны указывать тип каждого параметра.
fn another_function(x: i8) {
    println!("The value of x is: {x}");
}

// The value of x is: 56
