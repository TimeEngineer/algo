mod monte_carlo;
mod nn;
mod simplex;

pub use monte_carlo::*;
pub use nn::*;
pub use simplex::*;

pub enum Obj {
    Max,
    Min,
}
