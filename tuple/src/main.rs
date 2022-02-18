fn main() {
    let tup = (1, 'x', true);
    let first = tup.0;
    let second = tup.1;

    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
    println!("{} {}", first, second);
}
