// use eframe::egui;
// use eframe::egui::{Color32, Pos2, Stroke};
// use crate::neural_net::NeuralNetwork;
//
// fn visualize_network(ui: &mut egui::Ui, network: &NeuralNetwork, activations: &[Vec<f64>]) {
//     let neuron_radius = 20.0;
//     let layer_spacing = 150.0;
//     let neuron_spacing = 80.0;
//
//     let mut x_offset = 50.0;
//
//     for (layer_index, layer) in network.layers.iter().enumerate() {
//         let y_offset_start = 50.0;
//         let mut y_offset = y_offset_start;
//
//         // Label the layer name
//         ui.label(format!("Layer: {}", layer.layer_name));
//
//         for (neuron_index, neuron) in layer.neurons.iter().enumerate() {
//             let neuron_pos = Pos2::new(x_offset, y_offset);
//
//             let activation = activations.get(layer_index).and_then(|act| act.get(neuron_index)).unwrap_or(&0.0);
//             let color = if *activation > 0.0 {
//                 Color32::from_rgb(100, (*activation * 255.0) as u8, 100)
//             } else {
//                 Color32::from_rgb((activation.abs() * 255.0) as u8, 100, 100)
//             };
//
//             // Draw neuron
//             ui.painter().circle_filled(neuron_pos, neuron_radius, color);
//
//             // Display input values and weight details
//             ui.label(format!("Inputs: {:?}", neuron.input_values));
//             ui.label(format!("Output: {:.2}", neuron.value));
//             ui.label(format!("Bias: {:.2}", neuron.bias));
//
//             for (weight_index, weight) in neuron.weights.iter().enumerate() {
//                 ui.label(format!("Weight[{}]: {:.2}", weight_index, weight));
//             }
//
//             if layer_index > 0 {
//                 let previous_layer_activations = &activations[layer_index - 1];
//
//                 for (prev_neuron_index, weight) in neuron.weights.iter().enumerate() {
//                     let prev_neuron_pos = Pos2::new(
//                         x_offset - layer_spacing,
//                         50.0 + prev_neuron_index as f32 * neuron_spacing,
//                     );
//                     let weight_color = if *weight > 0.0 {
//                         Color32::from_rgb(0, (*weight * 255.0) as u8, 0)
//                     } else {
//                         Color32::from_rgb((weight.abs() * 255.0) as u8, 0, 0)
//                     };
//
//                     ui.painter().line_segment(
//                         [prev_neuron_pos, neuron_pos],
//                         Stroke::new(2.0, weight_color),
//                     );
//                 }
//             }
//
//             y_offset += neuron_spacing;
//         }
//
//         x_offset += layer_spacing;
//     }
// }
//
// pub fn main() -> Result<(), eframe::Error> {
//     let network = NeuralNetwork::new(
//         &[2, 2, 1],
//         vec!["Input Layer".to_string(), "Hidden Layer".to_string(), "Output Layer".to_string()],
//     );
//
//     let training_data = vec![
//         (vec![0.0, 0.0], vec![0.0]),
//         (vec![0.0, 1.0], vec![1.0]),
//         (vec![1.0, 0.0], vec![1.0]),
//         (vec![1.0, 1.0], vec![0.0]),
//     ];
//
//     let activations: Vec<Vec<f64>> = vec![];
//
//     let app = MyApp {
//         network,
//         training_data,
//         activations,
//         epoch: 0,
//     };
//
//     let options = eframe::NativeOptions::default();
//     eframe::run_native("Neural Network Visualization", options, Box::new(|_| Ok(Box::new(app))))
// }
//
// struct MyApp {
//     network: NeuralNetwork,
//     training_data: Vec<(Vec<f64>, Vec<f64>)>,
//     activations: Vec<Vec<f64>>,
//     epoch: usize,
// }
//
// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.label(format!("Epoch: {}", self.epoch));
//
//             if ui.button("Train Epoch").clicked() {
//                 for (inputs, _expected_outputs) in &self.training_data {
//                     self.network.feed_forward(inputs);
//                 }
//
//                 // Capture activations for visualization
//                 self.activations.clear();
//                 let mut current_inputs = vec![];
//                 for layer in &mut self.network.layers {
//                     current_inputs = layer.neurons.iter().map(|neuron| neuron.value).collect();
//                     self.activations.push(current_inputs.clone());
//                 }
//
//                 self.epoch += 1;
//             }
//
//             // Visualize the network with labels
//             visualize_network(ui, &self.network, &self.activations);
//         });
//
//         ctx.request_repaint();
//     }
// }
