#![allow(dead_code)]
#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: (0..rows)
                .map(|_| (0..cols).map(|_| rand::random::<f64>()).collect())
                .collect(),
        }
    }

    pub fn of(rows: usize, cols: usize, of: f64) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![of; cols]; rows],
        }
    }

    pub fn from_array(arr: Vec<Vec<f64>>) -> Matrix {
        let rows = arr.len();
        let cols = arr[0].len();
        Matrix {
            rows,
            cols,
            data: arr,
        }
    }

    pub fn to_array(&self) -> Vec<Vec<f64>> {
        self.data.clone()
    }

    pub fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{}\t", self.data[i][j]);
            }
            println!("")
        }
    }
}

pub fn matrix_moltiplication(a: Matrix, b: Matrix) -> Matrix {
    assert_eq!(a.cols, b.rows);

    let mut result = Matrix::zeros(a.rows, b.cols);

    // source: https://en.wikipedia.org/wiki/Matrix_multiplication_algorithm

    // matrix A is m x n
    // matrix B is n x p
    // matrix C is m x p

    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = 0.0;
            for k in 0..a.cols {
                sum += a.data[i][k] * b.data[k][j];
            }

            result.data[i][j] = sum;
        }
    }
    result
}

pub fn matrix_addition(a: Matrix, b: Matrix) -> Matrix {
    assert_eq!(a.rows, b.rows);
    assert_eq!(a.cols, b.cols);
    let mut result = Matrix::zeros(a.rows, a.cols);
    for i in 0..a.rows {
        for j in 0..a.cols {
            result.data[i][j] = a.data[i][j] + b.data[i][j];
        }
    }
    result
}

