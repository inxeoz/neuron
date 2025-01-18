use serde::{Deserialize, Serialize};
use crate::neural::{sigmoid_derivative, Neuron};

// Represents a single layer of neurons
#[derive(Serialize, Deserialize)]
pub struct Layer {
   pub neurons: Vec<Neuron>,
}

impl Layer {
   pub  fn new(num_neurons: usize, num_inputs: usize) -> Self {
        Self {
            neurons: (0..num_neurons).map(|_| Neuron::new(num_inputs)).collect(),
        }
    }

    pub fn feed_forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        self.neurons
            .iter_mut()
            .map(|neuron| {
                neuron.activate(inputs);
                neuron.value
            })
            .collect()
    }

    pub fn adjust_weights(&mut self, inputs: &[f64], learning_rate: f64, errors: &[f64]) {
        for (neuron, error) in self.neurons.iter_mut().zip(errors.iter()) {
            for (i, input) in inputs.iter().enumerate() {
                neuron.weights[i] += learning_rate * error * sigmoid_derivative(neuron.value) * input;
            }
            neuron.bias += learning_rate * error * sigmoid_derivative(neuron.value);
        }
    }
}