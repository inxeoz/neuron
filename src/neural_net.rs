use crate::neural_layer::Layer;

// Represents the entire neural network
pub struct NeuralNetwork {
   pub  layers: Vec<Layer>,
}

impl NeuralNetwork {
  pub  fn new(layer_sizes: &[usize]) -> Self {

      let mut layers = Vec::new();
      for i in 0..layer_sizes.len() - 1 {
          layers.push(Layer::new(layer_sizes[i + 1], layer_sizes[i])); //first layer_sizes[0] for inputs of model
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

    pub fn backpropagate(&mut self, inputs: &[f64], expected_outputs: &[f64], learning_rate: f64) {
        // Feedforward: Calculate activations for each layer
        let mut activations = vec![inputs.to_vec()];
        for layer in &mut self.layers {
            activations.push(layer.feed_forward(activations.last().unwrap()));
        }

        // Calculate output errors
        let mut errors = self.layers.last().unwrap().neurons.iter().enumerate()
            .map(|(neuron_index, neuron)| expected_outputs[neuron_index] - neuron.value)
            .collect::<Vec<_>>();

        // Backpropagate errors and adjust weights
        for i in (0..self.layers.len()).rev() {
            let layer_inputs = &activations[i];
            self.layers[i].adjust_weights(layer_inputs, learning_rate, &errors);

            // Prepare errors for the previous layer (if not the input layer)
            if i > 0 {
                // Access the neurons of the previous layer immutably after mutable borrow of `self.layers[i]`
                let previous_layer_neurons = &self.layers[i].neurons;

                // Calculate the errors for the previous layer
                errors = previous_layer_neurons.iter()
                    .map(|neuron| neuron.weights.iter()
                        .zip(errors.iter())
                        .map(|(weight, &error)| weight * error)
                        .sum())
                    .collect::<Vec<_>>();
            }
        }

    }

}



