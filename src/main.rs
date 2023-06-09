mod matrix;

use matrix::*;

fn main() {
    let mut m = Matrix::new(2, 3);
    let mut n = Matrix::new(3, 2);
    m.rand();
    n.rand();
    
    println!("Matrix m:");
    println!("{}", m);

    println!("Matrix n:");
    println!("{}", n);

    let p = matrix_multiply(&m, &n);
    println!("Matrix p = m * n:");
    println!("{}", p);


}
