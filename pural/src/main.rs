fn main() {
    let s = String::from("book");
    print!(
        "I have on {}, you have two {}",
        s,
        puralize(&s)
    );
}

fn puralize(s: &str) -> String {
    let mut s = s.to_string();
    s.push_str("s");
    s
}
