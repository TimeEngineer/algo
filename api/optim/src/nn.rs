use matrix::Matrix;
use rand::{
    distributions::Uniform,
    prelude::{Distribution, SliceRandom, ThreadRng},
    Rng,
};
use rand_distr::Normal;
use std::fmt::Debug;
use vector::VectorOp;

#[derive(Clone)]
pub struct NN<const IN: usize, const HIDDEN: usize, const OUT: usize> {
    w0: Matrix<f32, HIDDEN, IN>,
    b0: [f32; HIDDEN],
    hidden0: [f32; HIDDEN],
    w1: Matrix<f32, OUT, HIDDEN>,
    b1: [f32; OUT],
    pub output: [f32; OUT],
    t: i32,
    mt: [f32; OUT],
    vt: [f32; OUT],
}

impl<const IN: usize, const HIDDEN: usize, const OUT: usize> std::fmt::Debug
    for NN<IN, HIDDEN, OUT>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "const W0: [[f32; {IN}]; {HIDDEN}] = {:.03?};", self.w0.0)?;
        writeln!(f, "const B0: [f32; {HIDDEN}] = {:.03?};", self.b0)?;
        writeln!(
            f,
            "const W1: [[f32; {HIDDEN}]; {OUT}] = {:.03?};",
            self.w1.0
        )?;
        writeln!(f, "const B1: [f32; {OUT}] = {:.03?};", self.b1)
    }
}

fn gen_rand_mat<const N: usize, const M: usize>(
    rng: &mut ThreadRng,
    d: impl Distribution<f32>,
) -> [[f32; M]; N] {
    let mut mat = [[0.; M]; N];
    mat.iter_mut()
        .for_each(|x| x.iter_mut().for_each(|x| *x = rng.sample(&d)));
    mat
}

pub fn input_normalization<const IN: usize>(input: &mut [f32; IN]) {
    let mean = input.iter().sum::<f32>() / IN as f32;
    let var = input.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / IN as f32;
    let c = 1. / (var.sqrt() + 1e-8);
    input.iter_mut().for_each(|x| *x = (*x - mean) * c);
}

impl<const IN: usize, const HIDDEN: usize, const OUT: usize> Default for NN<IN, HIDDEN, OUT> {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            w0: Matrix::from_raw(gen_rand_mat(
                &mut rng,
                Normal::new(0., (2. / IN as f32).sqrt()).unwrap(),
            )),
            b0: [0.; HIDDEN],
            hidden0: [0.; HIDDEN],
            w1: Matrix::from_raw(gen_rand_mat(&mut rng, Uniform::new(-0.3, 0.3))),
            b1: [0.; OUT],
            output: [0.; OUT],
            t: 0,
            mt: [0.; OUT],
            vt: [0.; OUT],
        }
    }
}

impl<const IN: usize, const HIDDEN: usize, const OUT: usize> NN<IN, HIDDEN, OUT> {
    pub fn reset_adam(&mut self) {
        self.t = 0;
        self.mt = [0.; OUT];
        self.vt = [0.; OUT];
    }

    pub fn helper(&self) {
        println!("
fn input_normalization<const IN: usize>(input: &mut [f32; IN]) {{
    let mean = input.iter().sum::<f32>() / IN as f32;
    let var = input.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / IN as f32;
    let c = 1. / (var.sqrt() + 1e-8);
    input.iter_mut().for_each(|x| *x = (*x - mean) * c);
}}

fn layer_forward<const N: usize, const M: usize>(w: [[f32; M]; N], b: [f32; N], input: [f32; M]) -> [f32; N] {{
    let mut out = [0.; N];
    for i in 0..N {{
        for (j, x) in input.iter().enumerate() {{
            out[i] += w[i][j] * x;
        }}
        out[i] += b[i];
    }}
    out
}}

fn relu<const N: usize>(output: &mut [f32; N]) {{
    output.iter_mut().for_each(|h| {{
        if *h < 0. {{
            *h = 0.
        }}
    }});
}}

fn forward(mut input: [f32; {IN}]) -> [f32; {OUT}] {{
    input_normalization(&mut input);
    let mut hidden0 = layer_forward(W0, B0, input);
    relu(&mut hidden0);

    input_normalization(&mut hidden0);
    layer_forward(W1, B1, hidden0)
}}")
    }

    pub fn load(path: &str) -> std::io::Result<Self> {
        let lines = std::fs::read_to_string(path)?;
        let lines = lines.lines().collect::<Vec<_>>();
        let mut out = Self::default();

        let mut start = 0;

        for i in 0..HIDDEN {
            out.w0[i] = lines[start]
                .split_whitespace()
                .map(|x| x.parse::<f32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            start += 1;
        }

        out.b0 = lines[start]
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        start += 1;

        for i in 0..OUT {
            out.w1[i] = lines[start]
                .split_whitespace()
                .map(|x| x.parse::<f32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            start += 1;
        }

        out.b1 = lines[start]
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        start += 1;

        out.t = lines[start].parse::<i32>().unwrap();
        start += 1;

        out.mt = lines[start]
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        start += 1;

        out.vt = lines[start]
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        start += 1;

        assert_eq!(start, HIDDEN + OUT + 5);
        Ok(out)
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut out = String::new();

        for row in self.w0.0 {
            let values = row
                .iter()
                .map(|v| format!("{v:.03}"))
                .collect::<Vec<_>>()
                .join(" ");
            out.push_str(&values);
            out.push('\n');
        }

        let values = self
            .b0
            .iter()
            .map(|v| format!("{v:.03}"))
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&values);
        out.push('\n');

        for row in self.w1.0 {
            let values = row
                .iter()
                .map(|v| format!("{v:.03}"))
                .collect::<Vec<_>>()
                .join(" ");
            out.push_str(&values);
            out.push('\n');
        }

        let values = self
            .b1
            .iter()
            .map(|v| format!("{v:.03}"))
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&values);
        out.push('\n');

        out.push_str(&self.t.to_string());
        out.push('\n');

        let values = self
            .mt
            .iter()
            .map(|v| format!("{v:.04}"))
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&values);
        out.push('\n');

        let values = self
            .vt
            .iter()
            .map(|v| format!("{v:.04}"))
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&values);
        out.push('\n');

        std::fs::write(path, out)
    }

    pub fn linear_grad(&mut self, target: [f32; OUT]) -> [f32; OUT] {
        // Adam optimizer parameters
        const B1: f32 = 0.9;
        const B2: f32 = 0.999;
        const LR: f32 = 0.001;
        const EPS: f32 = 1e-8;

        let mut grad = [0.; OUT];
        self.t = std::cmp::max(self.t + 1, i32::MAX);
        for i in 0..OUT {
            let gradient = self.output[i] - target[i];
            self.mt[i] = B1 * self.mt[i] + (1. - B1) * gradient;
            self.vt[i] = B2 * self.vt[i] + (1. - B2) * gradient.powi(2);
            let mhat = self.mt[i] / (1. - B1.powi(self.t));
            let vhat = self.vt[i] / (1. - B2.powi(self.t));
            grad[i] = LR * mhat / (vhat.sqrt() + EPS);
        }
        grad
    }

    pub fn relu<const N: usize>(output: &mut [f32; N]) {
        output.iter_mut().for_each(|h| {
            if *h < 0. {
                *h = 0.
            }
        });
    }

    pub fn relu_grad<const N: usize, const M: usize>(
        layer: [f32; N],
        pre_grad: [f32; M],
        pre_w: [[f32; N]; M],
    ) -> [f32; N] {
        let mut grad = [0.; N];
        grad.iter_mut().enumerate().for_each(|(i, g)| {
            *g = if layer[i] > 0. {
                pre_grad
                    .iter()
                    .enumerate()
                    .map(|(j, g)| g * pre_w[j][i])
                    .sum()
            } else {
                0.
            }
        });
        grad
    }

    pub fn softmax(input: &[f32; OUT], output: &mut [f32; OUT]) {
        let max_x = input
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let c = 1. / input.iter().map(|x| (x - max_x).exp()).sum::<f32>();
        input
            .iter()
            .zip(output)
            .for_each(|(x, y)| *y = (x - max_x).exp() * c);
    }

    pub fn train_wb<const N: usize, const M: usize>(
        w: &mut [[f32; M]; N],
        b: &mut [f32; N],
        grad: [f32; N],
        layer: [f32; M],
    ) {
        (0..N).for_each(|i| {
            (0..M).for_each(|j| {
                w[i][j] -= grad[i] * layer[j];
            });
            b[i] -= grad[i];
        });
    }

    pub fn forward(&mut self, mut input: [f32; IN]) -> usize {
        input_normalization(&mut input);
        self.hidden0 = &self.w0 * input;
        VectorOp::add_assign(&mut self.hidden0, &self.b0);
        Self::relu(&mut self.hidden0);

        input_normalization(&mut self.hidden0);
        self.output = &self.w1 * self.hidden0;
        VectorOp::add_assign(&mut self.output, &self.b1);

        self.output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0
    }

    pub fn replay(&mut self, states: &mut Vec<([f32; IN], usize)>, reward: f32) {
        let (input, action) = states.pop().unwrap();

        self.learn(input, action, 0., reward);
        let mut i = self.forward(input);
        let mut next_qmax = self.output[i];

        while let Some((input, action)) = states.pop() {
            self.learn(input, action, next_qmax, 0.);
            i = self.forward(input);
            next_qmax = self.output[i];
        }
    }

    pub fn replay2(&mut self, states: &mut Vec<([f32; IN], usize)>, reward: f32) {
        let (input, action) = states.pop().unwrap();

        self.learn(input, action, 0., reward);
        let mut i = self.forward(input);
        let mut next_qmax = self.output[i];

        while let Some((input, action)) = states.pop() {
            self.learn(input, action, -next_qmax, 0.);
            i = self.forward(input);
            next_qmax = self.output[i];
        }
    }

    pub fn learn(&mut self, input: [f32; IN], action: usize, next_qmax: f32, reward: f32) {
        // Q-learning parameters
        const DF: f32 = 0.999;

        self.forward(input);

        let mut target = self.output;
        // Forget others move
        target.iter_mut().for_each(|t| *t *= 0.999);
        target[action] = reward + DF * next_qmax;

        self.backward(input, target);
    }

    pub fn backward(&mut self, input: [f32; IN], target: [f32; OUT]) {
        self.forward(input);

        // Gradient
        let grad1 = self.linear_grad(target);
        // Hidden-Output weights and bias
        Self::train_wb(&mut self.w1.0, &mut self.b1, grad1, self.hidden0);

        // Gradient
        let grad0 = Self::relu_grad(self.hidden0, grad1, self.w1.0);
        // Input-Hidden weights and bias
        Self::train_wb(&mut self.w0.0, &mut self.b0, grad0, input);
    }

    pub fn vs_random<const STATS_SIZE: usize, G: Game>(
        &mut self,
        player: usize,
        game: &G,
        actions: &mut Vec<usize>,
        rng: &mut ThreadRng,
    ) -> [i32; STATS_SIZE] {
        const TOTAL: usize = 10_000;
        let mut stats = [0; STATS_SIZE];

        for _ in 0..TOTAL {
            let mut game = game.clone();

            loop {
                game.fill(actions);

                if actions.is_empty() {
                    break;
                }

                let action = if game.turn() % 2 == player {
                    // Forward
                    self.forward(game.input());
                    // Get the most rated action
                    self.output
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| actions.contains(i))
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap()
                        .0
                } else {
                    *actions.choose(rng).unwrap()
                };

                game.update(action);
            }

            // STATS
            match game.status() {
                Status::Win(i) => stats[i] += 1,
                Status::Draw => stats[STATS_SIZE - 1] += 1,
                _ => panic!("Something goes wrong {game:?}"),
            }
        }
        stats.iter_mut().for_each(|s| *s /= (TOTAL / 100) as i32);
        stats
    }

    pub fn train_random<G: Game, const EPOCH: usize>(
        &mut self,
        player: usize,
        game: &G,
        states: &mut Vec<([f32; IN], usize)>,
        actions: &mut Vec<usize>,
        rng: &mut ThreadRng,
        eps: f32,
    ) {
        for _ in 0..EPOCH {
            let mut game = game.clone();
            states.clear();

            loop {
                let input = game.input();

                game.fill(actions);

                if actions.is_empty() {
                    break;
                }

                let action = if rng.gen_range(0f32..1f32) < eps || game.turn() % 2 != player {
                    // Explore
                    *actions.choose(rng).unwrap()
                } else {
                    // Forward
                    self.forward(input);
                    // Get the most rated action
                    self.output
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| actions.contains(i))
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap()
                        .0
                };

                if game.turn() % 2 == player {
                    states.push((input, action));
                }

                game.update(action);
            }

            let reward = game.reward(player);

            self.replay(states, reward);
        }
    }

    pub fn train<G: Game, const EPOCH: usize>(
        &mut self,
        game: &G,
        states: &mut Vec<([f32; IN], usize)>,
        actions: &mut Vec<usize>,
        rng: &mut ThreadRng,
        eps: f32,
    ) {
        for _ in 0..EPOCH {
            let mut game = game.clone();
            states.clear();

            loop {
                // Parse input to nn
                let input = game.input();

                game.fill(actions);

                if actions.is_empty() {
                    break;
                }

                let action = if rng.gen_range(0f32..1f32) < eps {
                    // Explore
                    *actions.choose(rng).unwrap()
                } else {
                    // Forward
                    self.forward(input);
                    // Get the most rated action
                    self.output
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| actions.contains(i))
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap()
                        .0
                };

                states.push((input, action));
                game.update(action);
            }

            let reward = if game.turn() % 2 == 0 {
                game.reward(0)
            } else {
                game.reward(1)
            };

            self.replay2(states, reward);
        }
    }
}

#[test]
fn backward() {
    let mut nn = NN::<9, 36, 9>::default();
    let mut rng = rand::thread_rng();

    let mut input = [0f32; 9];
    rng.fill(&mut input);

    let action = nn.forward(input);

    let old = nn.output;
    let mut output = nn.output;
    output[action] += 1.;

    nn.backward(input, output);

    nn.forward(input);

    assert!(nn.output[action] > old[action]);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Win(usize),
    Draw,
    #[default]
    None,
}

pub trait Game: Clone + Debug {
    /// Flatten into NN input
    fn input<const N: usize>(&self) -> [f32; N];
    fn turn(&self) -> usize;
    fn status(&self) -> Status;
    fn reward(&self, player: usize) -> f32;
    /// Don't forget to clear and check if the game ended
    fn fill(&self, actions: &mut Vec<usize>);
    fn update(&mut self, action: usize);
}
