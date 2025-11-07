use crate::shapes::Shape;

mod shapes;

fn main() {
    // basically here im saying x can be changed, and y is a mutable reference.
    // so i can change the value of x through y
    // let mut x = 5;
    // let y = &mut x;

    // *y = 7;

    // println!("{:?}", x);

    learning_shapes();
}

fn learning_shapes() {
    let circle = shapes::Circle { diameter: 2.0 };
    println!("{}", circle.area())
}
