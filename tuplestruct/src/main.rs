struct Triangle (u32, u32, u32);

fn is_equilateral(triangle1: Triangle) -> bool {
    triangle1.0 == triangle1.1 && triangle1.1 == triangle1.2
}

fn main () {
    let triangle1 = Triangle(3, 3, 3);
    let result = is_equilateral(triangle1);
    println!("{}", result);
}
