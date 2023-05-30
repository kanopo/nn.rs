#![allow(dead_code)]

use crate::matrix::*;

pub struct Network {
    pub layers: Vec<usize>,
    pub weights: Vec<Matrix>,
    pub biases: Vec<Matrix>,
    pub activations: Vec<Matrix>,
}

impl Network {
    pub fn new(layers: Vec<usize>) -> Network {
        assert!(layers.first() > Some(&0));
        assert!(layers.last() > Some(&0));

        let mut weights = Vec::new();
        let mut biases = Vec::new();
        let mut activations = Vec::new();

        Network {
            layers,
            weights,
            biases,
            activations,
        }
    }
}
