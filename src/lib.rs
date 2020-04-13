use nalgebra::DMatrix;
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

pub const SIGMOID: fn(f32) -> f32 = |x| 1.0 / (1.0 + (-x).exp());

pub struct NeuralNet {
    layers: Vec<Layer>,
    weights: Vec<DMatrix<f32>>,
    biases: Vec<DMatrix<f32>>,
    activation: fn(f32) -> f32,
}

impl NeuralNet {
    // TODO: Add check to see if more than 1 layer was supplied
    pub fn new(node_counts: Vec<usize>, activation: fn(f32) -> f32, rng: &mut impl Rng) -> Self {
        Self {
            layers: node_counts.iter().map(|c| Layer::new(*c)).collect(),
            weights: (0..node_counts.len() - 1)
                .map(|i| gen_random_matrix(node_counts[i + 1], node_counts[i], rng))
                .collect(),
            biases: (1..node_counts.len())
                .map(|i| gen_random_matrix(node_counts[i], 1, rng))
                .collect(),
            activation,
        }
    }

    pub fn feedforward(&mut self, inputs: Vec<f32>) -> Vec<f32> {
        let num_layers = self.layers.len();
        self.layers[0].value = DMatrix::from_row_slice(self.layers[0].node_count, 1, &inputs);

        for i in 1..num_layers {
            let mut value = &self.weights[i - 1] * &self.layers[i - 1].value + &self.biases[i - 1];

            for x in value.iter_mut() {
                *x = (self.activation)(*x);
            }

            self.layers[i].value = value;
        }

        self.layers[num_layers - 1].value.iter().cloned().collect()
    }
}

pub struct Layer {
    pub node_count: usize,
    pub value: DMatrix<f32>,
}

impl Layer {
    pub fn new(node_count: usize) -> Self {
        Self {
            node_count,
            value: DMatrix::zeros(node_count, 1),
        }
    }
}

fn gen_random_matrix(rows: usize, cols: usize, rng: &mut impl Rng) -> DMatrix<f32> {
    let elements = rows * cols;
    let range = Uniform::new_inclusive(-1.0, 1.0);
    DMatrix::from_iterator(rows, cols, (0..elements).map(|_| range.sample(rng)))
}
