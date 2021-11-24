extern crate pbrusty;

use pbrusty::geometry::Vector;

fn main() {
    let a = Vector::new(1., 0., 0.);
    let b = Vector::new(0., 1., 0.);
    let c = Vector::new(0., 0., 1.);
    let d = Vector::cross(a, b);
    println!("{}", c == d);
}
