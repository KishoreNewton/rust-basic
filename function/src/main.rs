fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!("{} will be {} next birthday", name, next_age);
}

fn square (num: i32) -> i32 {
    num * num
}

fn main() {
    next_birthday("John", 30);
    next_birthday("Vivan", 0);
    println!("The answer is {}", square(3));
}
