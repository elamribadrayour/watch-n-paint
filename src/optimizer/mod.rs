mod sgd;
mod optimizer_impl;

pub use sgd::Sgd;
pub use optimizer_impl::{Optimizer, get_optimizer};