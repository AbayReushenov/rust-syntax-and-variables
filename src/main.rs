fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
}

//   |              -- value moved here
// 4 |
// 5 |     println!("{s1}, world!");
//   |                ^^ value borrowed here after move
