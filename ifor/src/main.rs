fn main() {
    println!("Hello, world!");
    discount(10);
}

// if expression {
//     // ...
// } else if expression {
//     // ...
// } else {
//     // ...
// }
// 
// loop {
//     // ...
// }   
//


fn discount(day_of_month: u8) {
    let amount = if day_of_month % 2 == 0 {
        50 
    } else {
        10
    };

    println!("Your discount is {}", amount);
}
