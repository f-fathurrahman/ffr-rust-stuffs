#[macro_use(array)]
extern crate ndarray;

use ndarray::rcarr2;

fn main() {
    let a1 = rcarr2(&[[10.1, 14.1],[19.1,20.0]]);
    println!("a1 = {:?}", a1);

    let a2 = array![ [2., 3., 1.0], [3.1, 2.1, 11.1]];
    println!("a2 = ");
    println!("{}", a2);

    let a3 = array![ [1.1, 3.1, 4.5], [5.3, 44.1, 8.2] ];
    println!("a3 = ");
    println!("{}", a3);

    let a4 = a3*a2;
    println!("a4 = ");
    println!("{}", a4);

    println!("Pass here");
}
