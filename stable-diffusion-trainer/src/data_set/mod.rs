//! This module contains the data set types used for training.

pub mod image_data_set;
pub use image_data_set::*;

use crate::{prelude::*, utils::Update};

/// A training data set.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TrainingDataSet {
    /// The training images.
    pub training: ImageDataSet,
    /// The regularization images.
    pub regularization: Option<ImageDataSet>,
}

impl TrainingDataSet {
    /// Create a new training data set.
    pub fn new(training: ImageDataSet) -> Self {
        let regularization = Default::default();
        TrainingDataSet { training, regularization }
    }

    /// Set the regularization images for the training data set.
    pub fn with_regularization_images(mut self, regularization_images: ImageDataSet) -> Self {
        self.regularization = Some(regularization_images);
        self
    }
}

impl Update for TrainingDataSet {
    fn update(&mut self, base: Self) {
        self.training.update(base.training);
        self.regularization.update(base.regularization);
    }
}