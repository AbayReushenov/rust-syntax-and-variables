fn main() {
    // array - массив;1
    let a = [1, 2, 3, 4, 5];

    // array indexing by position
    let first = a[0];

    println!("The value of first is: {first}");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The value of months is: {}", months.len());
    println!("January is in the {}, it's the first position", months[0]);

    let a2 = [3; 5];
    println!("The value of a2 is: {}", a2[0]);
    println!("The value of a2 is: {}", a2[1]);
    println!("The value of a2 is: {}", a2[2]);
    println!("The value of a2 is: {}", a2[3]);
    println!("The value of a2 is: {}", a2[4]);

    let a3 = [188, 288, 388, 488, 588];

    let first = a3[0];
    let second = a3[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    // The value of first is: 188
    // The value of second is: 288


}

// The value of first is: 1
// The value of months is: 12
// January is in the January, it's the first position
// The value of a2 is: 3
// The value of a2 is: 3
// The value of a2 is: 3
// The value of a2 is: 3
// The value of a2 is: 3
