fn main () {
    let mut x: i32 = 5;
    let y = 6;
    x += 1;
    let z = x + y;

    let a = true;
    let b = false;

    if a {
        println!("a is true!");  
    }

    if b {
        println!("b is true");
    }
    println!("z is {}", z);
}
