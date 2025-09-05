fn main() {
    {                      // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
        println!("s {}", s)
    }
    // this scope is now over, and s is no longer valid

    let s = "hello 333";
    println!("s {}", s);

    // Оператор "Двойное двоеточие" :: позволяет использовать пространство имён
    // данной конкретной функции from с типом String
    let mut s3 = String::from("hello ");

    println!("S#3 {}", s3);

    s3.push_str(", world!"); // push_str() appends a literal to a Stri

    println!("S#3 {}", s3);

    // s hello
    // s hello 333
    // S#3 hello
    // S#3 hello , world!
}
