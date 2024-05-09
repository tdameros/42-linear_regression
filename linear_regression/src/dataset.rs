use std::error::Error;
use std::fs::File;

#[derive(Debug, Default)]
pub struct DatasetRow {
    data: Vec<f64>,
    pub min: f64,
    pub max: f64,
}

impl DatasetRow {
    pub fn push(&mut self, data: f64) {
        self.data.push(data);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn normalize(&mut self) {
        self.min = self.data.iter().cloned().fold(f64::INFINITY, f64::min);
        self.max = self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        let range = self.max - self.min;

        for value in self.data.iter_mut() {
            *value = (*value - self.min) / range;
        }
    }

    fn denormalize(&mut self) {
        let range = self.max - self.min;
        for value in self.data.iter_mut() {
            *value = *value * range + self.min;
        }
    }
}

#[derive(Debug, Default)]
pub struct Dataset {
    pub x: DatasetRow,
    pub y: DatasetRow,
}

enum DatasetError {
    IsEmpty,
    CouldNotOpenFile(String),
    InvalidFormat(String),
}

impl Error for DatasetError {}

impl std::fmt::Display for DatasetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatasetError::IsEmpty => write!(f, "Dataset is empty"),
            DatasetError::CouldNotOpenFile(msg) => write!(f, "CouldNotOpenFile: {}", msg),
            DatasetError::InvalidFormat(msg) => write!(f, "InvalidFormat: {}", msg),
        }
    }
}

impl std::fmt::Debug for DatasetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DatasetError::{}", self)
    }
}

impl Dataset {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let file =
            File::open(path).map_err(|err| DatasetError::CouldNotOpenFile(format!("{}", err)))?;
        let mut reader = csv::Reader::from_reader(file);
        let mut dataset: Self = Default::default();
        for result in reader.deserialize() {
            let record: (f64, f64) =
                result.map_err(|err| DatasetError::InvalidFormat(format!("{}", err)))?;
            dataset.push(record);
        }
        if dataset.is_empty() {
            return Err(Box::new(DatasetError::IsEmpty));
        }
        Ok(dataset)
    }

    pub fn push(&mut self, row: (f64, f64)) {
        self.x.push(row.0);
        self.y.push(row.1);
    }

    pub fn len(&self) -> usize {
        self.x.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn normalize(&mut self) {
        self.x.normalize();
        self.y.normalize();
    }

    pub fn denormalize(&mut self) {
        self.x.denormalize();
        self.y.denormalize();
    }

    pub fn get_x_min(&self) -> f64 {
        self.x.min
    }

    pub fn get_x_max(&self) -> f64 {
        self.x.max
    }

    pub fn get_y_min(&self) -> f64 {
        self.y.min
    }

    pub fn get_y_max(&self) -> f64 {
        self.y.max
    }
}

impl<'a> IntoIterator for &'a Dataset {
    type Item = (&'a f64, &'a f64);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let keys_ref: &'a Vec<f64> = &self.x.data;
        let values_ref: &'a Vec<f64> = &self.y.data;
        let tuples = keys_ref.iter().zip(values_ref.iter());
        tuples.collect::<Vec<_>>().into_iter()
    }
}

impl IntoIterator for Dataset {
    type Item = (f64, f64);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let keys = self.x.data;
        let values = self.y.data;
        let tuples = keys.into_iter().zip(values);
        tuples.collect::<Vec<_>>().into_iter()
    }
}
