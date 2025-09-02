#![allow(unused)]
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");
    // The value of guess is: 42

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    // The value of x is: 2
    // The value of y is: 3

        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1

        // remainder
        let remainder = 43 % 5;

        println!("The value of sum is: {sum}");
        println!("The value of difference is: {difference}");
        println!("The value of product is: {product}");
        println!("The value of quotient is: {quotient}");
        println!("The value of truncated is: {truncated}");
        println!("The value of remainder is: {remainder}");
        // The value of sum is: 15
        // The value of difference is: 91.2
        // The value of product is: 120
        // The value of quotient is: 1.7608695652173913
        // The value of truncated is: -1
        // The value of remainder is: 3

        // boolean
        let t = true;

        let f: bool = false; // with explicit type annotation

        println!("The value of t is: {t}");
        println!("The value of f is: {f}");
        // The value of t is: true
        // The value of f is: false

        // character
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';

        println!("The value of c is: {c}");
        println!("The value of z is: {z}");
        println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
        // The value of c is: z
        // The value of z is: ℤ
        // The value of heart_eyed_cat is: 😻

        // tuple
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        println!("The value of tup is: {:?}", tup);
        // The value of tup is: (500, 6.4, 1)

        // tuple destructuring
        let (x, y, z) = tup;
        println!("The value of x is: {x}");
        println!("The value of y is: {y}");
        println!("The value of z is: {z}");
        // The value of x is: 500
        // The value of y is: 6.4
        // The value of z is: 1

        // tuple indexing
        let x: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;

        println!("The value of five_hundred is: {five_hundred}");

        println!("The value of six_point_four is: {six_point_four}");
        println!("The value of one is: {one}");
        // The value of five_hundred is: 500
        // The value of six_point_four is: 6.4
        // The value of one is: 1

        // array
        let lla = [1, 2, 3, 4, 5];

        let lkk: [i32; 5] = [1, 2, 3, 4, 5];

        let plla = [3; 5];
        // Массив в переменной a будет включать 5 элементов, значение которых будет равно 3.
        // Данная запись аналогична коду let a = [3, 3, 3, 3, 3];, но является более краткой.

        println!("The value of lla is: {:?}", lla);
        println!("The value of lkk is: {:?}", lkk);
        println!("The value of plla is: {:?}", plla);
        // The value of lla is: [1, 2, 3, 4, 5]
        // The value of lkk is: [1, 2, 3, 4, 5]
        // The value of plla is: [3, 3, 3, 3, 3]

        // array indexing
        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];

        println!("The value of first is: {first}");
        println!("The value of second is: {second}");
        // The value of first is: 1
        // The value of second is: 2

        let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

        println!("The value of months is: {:?}", months);
        // The value of months is: ["January", "February", "March", "April", "May", "June", "July", "August",
        // "September", "October", "November", "December"]

        let a = [3; 2];

        println!("The value of a is: {:?}", a);
        // The value of a is: [3, 3]

        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
}



// Таблица 3-1: целочисленные типы в Rust

// Длина	Со знаком	Без знака
// 8 бит	i8	u8
// 16 бит	i16	u16
// 32 бита	i32	u32
// 64 бита	i64	u64
// 128 бит	i128	u128
// архитектурно-зависимая	isize	usize

// Каждый вариант со знаком может хранить числа от -(2 n - 1 ) до 2 n - 1 - 1 включительно,
// где n — количество битов, которые использует этот вариант.
// Таким образом, i8 может хранить числа от -(2 7 ) до 2 7 - 1, что равно значениям от -128 до 127.
// Варианты без знака могут хранить числа от 0 до 2 n - 1, поэтому
// u8 может хранить числа от 0 до 2 8 - 1, что равно значениям от 0 до 255.

// Кроме того, типы isize и usize зависят от архитектуры компьютера, на котором выполняется программа,
// и обозначаются в таблице как "arch": 64 бита, если используется 64-битная архитектура, и 32 бита,
// если используется 32-битная архитектура.
