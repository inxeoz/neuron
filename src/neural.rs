use rand::Rng;

// Sigmoid activation function
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

// Derivative of sigmoid function
pub fn sigmoid_derivative(x: f64) -> f64 {
    x * (1.0 - x)
}


pub fn xavier_init(num_connections: usize) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(-1.0..1.0) / (num_connections as f64).sqrt()
}

// Represents a single neuron
pub struct Neuron {
    pub value: f64,
    pub weights: Vec<f64>,
    pub bias: f64,
}

impl Neuron {

    pub fn new(num_connections: usize) -> Self {
        Self {
            value: 0.0,
            weights: (0..num_connections).map(|_| xavier_init(num_connections)).collect(),
            bias: xavier_init(num_connections),
        }
    }

   pub fn activate(&mut self, inputs: &[f64]) {
        let weighted_sum: f64 = inputs
            .iter()
            .zip(self.weights.iter())
            .map(|(input, weight)| input * weight)
            .sum::<f64>()
            + self.bias;
        self.value = sigmoid(weighted_sum);
    }
}