use csv::Writer;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

use crate::dataset::Dataset;

enum LinearModelError {
    CouldNotOpenFile(String),
    InvalidFormat(String),
    CouldNotSaveFile(String),
    CouldNotSerialize(String),
}

impl Error for LinearModelError {}

impl std::fmt::Display for LinearModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LinearModelError::CouldNotOpenFile(msg) => write!(f, "CouldNotOpenFile: {}", msg),
            LinearModelError::InvalidFormat(msg) => write!(f, "InvalidFormat: {}", msg),
            LinearModelError::CouldNotSaveFile(msg) => write!(f, "CouldNotSaveFile: {}", msg),
            LinearModelError::CouldNotSerialize(msg) => write!(f, "CouldNotSerialize: {}", msg),
        }
    }
}

impl std::fmt::Debug for LinearModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LinearModelError::{}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearModel {
    pub a: f64,
    pub b: f64,
    pub learning_rate: f64,
}

impl LinearModel {
    pub fn new(learning_rate: f64) -> Self {
        LinearModel {
            a: 0.,
            b: 0.,
            learning_rate,
        }
    }

    pub fn estimate(&self, x: f64) -> f64 {
        self.a * x + self.b
    }

    pub fn load(model_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(model_path)
            .map_err(|err| LinearModelError::CouldNotOpenFile(format!("{}", err)))?;
        let mut reader = csv::Reader::from_reader(file);
        if let Some(result) = reader.deserialize().next() {
            let model: Self =
                result.map_err(|err| LinearModelError::InvalidFormat(format!("{}", err)))?;
            Ok(model)
        } else {
            Err(Box::new(LinearModelError::InvalidFormat("".to_string())))
        }
    }

    pub fn save(&self, model_path: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::from_path(model_path)
            .map_err(|err| LinearModelError::CouldNotSaveFile(format!("{}", err)))?;
        writer
            .serialize(self)
            .map_err(|err| LinearModelError::CouldNotSerialize(format!("{}", err)))?;
        writer
            .flush()
            .map_err(|err| LinearModelError::CouldNotSaveFile(format!("{}", err)))?;
        Ok(())
    }

    pub fn train(&mut self, dataset: &Dataset, iterations: usize) {
        for _ in 0..iterations {
            self.gradient_descent(dataset);
        }
    }

    fn gradient_descent(&mut self, dataset: &Dataset) {
        let tmp_a = self.a - self.learning_rate * self.cost_a(dataset);
        let tmp_b = self.b - self.learning_rate * self.cost_b(dataset);
        self.a = tmp_a;
        self.b = tmp_b;
    }

    fn cost_a(&self, dataset: &Dataset) -> f64 {
        let mut result: f64 = 0.;
        for (key, value) in dataset {
            result += (self.estimate(*key) - *value) * *key;
        }
        result / dataset.len() as f64
    }

    fn cost_b(&self, dataset: &Dataset) -> f64 {
        let mut result: f64 = 0.;
        for (key, value) in dataset {
            result += self.estimate(*key) - *value;
        }
        result / dataset.len() as f64
    }

    pub fn denormalize(&mut self, dataset: &Dataset) {
        let range_x = dataset.x.max - dataset.x.min;
        let range_y = dataset.y.max - dataset.y.min;
        self.a *= (range_y) / (range_x);
        self.b = range_y * self.b + dataset.y.min - range_y / range_x * dataset.x.min * self.a;
    }

    pub fn mean_absolute_percentage_error(&self, dataset: &Dataset) -> f64 {
        let mut result: f64 = 0.;
        for (key, value) in dataset {
            result += (self.estimate(*key) - *value).abs() / *value;
        }
        result / dataset.len() as f64
    }
}

impl Default for LinearModel {
    fn default() -> Self {
        LinearModel {
            a: 0.,
            b: 0.,
            learning_rate: 0.1,
        }
    }
}
