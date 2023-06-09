#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn rand(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.data[row][col] = rand::random::<f64>();
            }
        }
    }
}

pub fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    assert_eq!(a.cols, b.rows);
    let mut result = Matrix::new(a.rows, b.cols);
    for row in 0..result.rows {
        for col in 0..result.cols {
            let mut sum = 0.0;
            for i in 0..a.cols {
                sum += a.data[row][i] * b.data[i][col];
            }
            result.data[row][col] = sum;
        }
    }
    result
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                s.push_str(&format!("{:.2} ", self.data[row][col]));
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}
