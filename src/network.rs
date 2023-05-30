#![allow(dead_code)]

use crate::matrix::*;

// pub struct Neuron {
//     inputs: Vec<f64>,
//     weights: Vec<f64>,
//     bias: f64,
//     activation: fn(f64) -> f64,
// }
//
// impl Neuron {
//     pub fn new(inputs: Vec<f64>, weights: Vec<f64>, bias: f64) -> Neuron {
//         Neuron {
//             inputs,
//             weights,
//             bias,
//             activation: |x| x * 2.0,
//         }
//     }
//
//     pub fn feedforward(&self) -> f64 {
//         let mut total = 0.0;
//         for i in 0..self.inputs.len() {
//             total += self.inputs[i] * self.weights[i];
//         }
//         total + self.bias
//     }
// }

pub struct Layer {
    inputs: Vec<Matrix>,
    biases: Vec<Matrix>,
    weights: Vec<Matrix>,
    activation: fn(f64) -> f64,
    output: Vec<Matrix>,
}
