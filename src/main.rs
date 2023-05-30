mod nn;

use nn::Matrix;

fn main() {
    let zeros = Matrix::zeros(3, 2);
    let random = Matrix::random(3, 2);
    println!("Zeros matrix{:?}", zeros);
    println!("Random matrx: {:?}", random);
}
