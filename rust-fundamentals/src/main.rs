// fn say(s: String) {
//     println!("I say, {}", s);
// }
// 
// fn main() {
//     let mut a = String::from("hello");
//     a.push_str(", world!");
//     println!("I say, {}", a);
//     say(a);
// }

fn say(s: String) {
    print!("I say, {}", s);
}

fn main() {
    let a = String::from("hello");
    say(a.clone());
    print!("using a again: {}", a);
}
