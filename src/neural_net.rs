use crate::neural_layer::Layer;

// Represents the entire neural network
pub struct NeuralNetwork {
   pub  layers: Vec<Layer>,
}

impl NeuralNetwork {
  pub  fn new(layer_sizes: &[usize]) -> Self {

      let mut layers = Vec::new();
      for i in 0..layer_sizes.len() - 1 {
          layers.push(Layer::new(layer_sizes[i + 1], layer_sizes[i]));
      }
      Self { layers }
    }

    pub fn feed_forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        let mut current_inputs = inputs.to_vec();
        for layer in &mut self.layers {
            current_inputs = layer.feed_forward(&current_inputs);
        }
        current_inputs
    }

   pub  fn backpropagate(&mut self, inputs: &[f64], expected_outputs: &[f64], learning_rate: f64) {
        let mut activations = vec![inputs.to_vec()];
        for layer in &mut self.layers {
            let output = layer.feed_forward(activations.last().unwrap());
            activations.push(output);
        }

        let mut errors = vec![vec![0.0; self.layers.last().unwrap().neurons.len()]];
        for (i, neuron) in self.layers.last().unwrap().neurons.iter().enumerate() {
            errors[0][i] = expected_outputs[i] - neuron.value;
        }

        let mut layer_errors = errors.remove(0);
        for i in (0..self.layers.len()).rev() {
            let layer_inputs = &activations[i];
            self.layers[i].adjust_weights(layer_inputs, learning_rate, &layer_errors);

            if i > 0 {
                let mut new_errors = vec![0.0; self.layers[i - 1].neurons.len()];
                for (j, neuron) in self.layers[i].neurons.iter().enumerate() {
                    for (k, &weight) in neuron.weights.iter().enumerate() {
                        new_errors[k] += layer_errors[j] * weight;
                    }
                }
                layer_errors = new_errors;
            }
        }
    }
}

pub fn main() {
    let mut network = NeuralNetwork::new(&[2, 4, 1]); // Add more neurons to hidden layer

    let training_data = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];
    let learning_rate = 0.9; // Adjusted learning rate
    let epochs = 5000;      // Fewer epochs with better training

    for epoch in 0..epochs {
        for (inputs, expected_outputs) in &training_data {
            network.backpropagate(inputs, expected_outputs, learning_rate);
        }
        if epoch % 500 == 0 {
            println!("Epoch {} complete", epoch);
        }
    }

    for (inputs, _) in &training_data {
        let output = network.feed_forward(inputs);
        println!("Inputs: {:?}, Output: {:?}", inputs, output);
    }
}

