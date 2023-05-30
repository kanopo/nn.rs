mod matrix;
mod network;

use matrix::*;
use network::*;


fn main() {
    let a = Matrix::random(3, 3);

    a.print()



}
