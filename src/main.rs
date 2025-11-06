fn main() {
    // basically here im saying x can be changed, and y is a mutable reference.
    // so i can change the value of x through y
    let mut x = 5;
    let y = &mut x;

    *y = 7;

    println!("{:?}", x);
}
