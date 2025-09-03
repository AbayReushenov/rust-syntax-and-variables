fn main() {
    let x = 5;

    let x = x + 1;
    
    // shadowing (переопределение переменных)
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}


// $ cargo run
//    Compiling variables v0.1.0 (/home/aaaaa/rust/variables)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
//      Running `target/debug/variables`
// The value of x in the inner scope is: 12
// The value of x is: 6
