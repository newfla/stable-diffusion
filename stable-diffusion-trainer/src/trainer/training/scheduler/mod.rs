//! Learning rate scheduler module.

use crate::prelude::*;
use std::fmt::Display;

/// The learning rate scheduler structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct LearningRate {
    /// The amount of the learning rate.
    #[serde(default = "default_amount")]
    pub amount: f32,
    /// The learning rate scheduler.
    pub scheduler: LearningRateScheduler
}

fn default_amount() -> f32 { 0.001 }

impl Default for LearningRate {
    fn default() -> Self {
        let amount = default_amount();
        let scheduler = LearningRateScheduler::Constant;
        LearningRate { amount, scheduler }
    }
}

impl LearningRate {
    /// Create a new learning rate structure.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the amount of the learning rate.
    pub fn with_amount(mut self, amount: f32) -> Self {
        self.amount = amount;
        self
    }

    /// Set the learning rate scheduler.
    pub fn with_scheduler(mut self, scheduler: LearningRateScheduler) -> Self {
        self.scheduler = scheduler;
        self
    }
}

/// The learning rate scheduler enumeration.
#[derive(Debug, Serialize, Deserialize)]
pub enum LearningRateScheduler {
    /// Adafactor learning rate scheduler.
    Adafactor,
    /// Constant learning rate scheduler.
    Constant,
    /// Constant with warmup learning rate scheduler.
    ConstantWithWarmup,
    /// Cosine learning rate scheduler.
    Cosine,
    /// Cosine with restarts learning rate scheduler.
    CosineWithRestarts,
    /// Linear learning rate scheduler.
    Linear,
    /// Polynomial learning rate scheduler.
    Polynomial
}

impl Display for LearningRateScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LearningRateScheduler::Adafactor => write!(f, "adafactor"),
            LearningRateScheduler::Constant => write!(f, "constant"),
            LearningRateScheduler::ConstantWithWarmup => write!(f, "constant_with_warmup"),
            LearningRateScheduler::Cosine => write!(f, "cosine"),
            LearningRateScheduler::CosineWithRestarts => write!(f, "cosine_with_restarts"),
            LearningRateScheduler::Linear => write!(f, "linear"),
            LearningRateScheduler::Polynomial => write!(f, "polynomial")
        }
    }
}