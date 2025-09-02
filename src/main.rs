// fn main() {
// let mut x = 5;
// println!("The value of x is: {}", x);
// x = 6;
// println!("The value of x is: {}", x);

// #![allow(unused)]
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// println!("The value of x is: {}", THREE_HOURS_IN_SECONDS);

// }

fn main() {
    let x = 5;
    println!("1. The value of x is: {x}");

    let x = x + 1;

    println!("2. The value of x is: {x}");

    {
        let x = x * 2;
        println!("3. The value of x in the inner scope is: {x}");
    }

    println!("4. The value of x is: {x}");
    // 1. The value of x is: 5
    // 2. The value of x is: 6
    // 3. The value of x in the inner scope is: 12
    // 4. The value of x is: 6

    let spaces  = "   ";
    let spaces = spaces.len();

    println!("5. The value of spaces is: {spaces}");
    // 5. The value of spaces is: 3
    

}
