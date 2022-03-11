use ndarray::Array2;

use std::collections::BTreeMap;

#[cfg(feature = "persist")]
use serde::{Deserialize, Serialize};

// def normalize_bipolar()

#[derive(Debug)]
#[cfg_attr(feature = "persist", derive(Serialize, Deserialize))]
pub struct Kohonen<K: Ord> {
    weights: BTreeMap<K, (Array2<i64>, i64)>,
    dim: (usize, usize),
}

pub fn normalize(a: &Array2<u8>) -> Array2<i64> {
    Array2::from_shape_vec(
        a.dim(),
        a.into_iter().map(|&v| if v > 0 { 1 } else { -1 }).collect(),
    )
    .unwrap()
}

impl<K: Ord> Kohonen<K> {
    pub fn init<I>(dim: (usize, usize), keys: I) -> Self
    where
        I: Iterator<Item = K>,
    {
        let mut weights = BTreeMap::new();

        keys.for_each(|k| {
            weights.insert(k, (Array2::zeros(dim), 0));
        });

        Self { weights, dim }
    }

    pub fn teach(&mut self, key: K, value: &Array2<u8>) {
        assert_eq!(self.dim, value.dim());

        let normalized = normalize(value);

        let (weights, count) = self
            .weights
            .entry(key)
            .or_insert_with(|| (Array2::zeros(self.dim), 0));

        *weights += &normalized;
        *count += 1;
    }

    pub fn guess(&self, value: &Array2<u8>) -> (&K, f32) {
        let normalized = normalize(value);

        self.weights
            .iter()
            .map(|(k, (w, c))| {
                let score = (w * &normalized).sum() / c;
                (k, score)
            })
            .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
            .map(|(k, v)| (k, v as f32 / self.ideal_score(k).unwrap() as f32))
            .unwrap()
    }

    fn ideal_score(&self, key: &K) -> Option<i64> {
        self.weights
            .get(key)
            .map(|(w, c)| w.into_iter().map(|&v| i64::abs(v)).sum::<i64>() / *c)
    }
}

#[cfg(feature = "persist")]
pub mod persist {
    use anyhow::Result;
    use serde::{de::DeserializeOwned, ser::Serialize};

    use std::{
        fs::{File, OpenOptions},
        io::{BufReader, BufWriter},
        path::Path,
    };

    use super::*;

    impl<K> Kohonen<K>
    where
        K: Ord + Serialize + DeserializeOwned,
    {
        pub fn save_to(&self, p: impl AsRef<Path>) -> Result<()> {
            let mut options = OpenOptions::new();
            let writer = BufWriter::new(
                options
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(p.as_ref())?,
            );

            serde_json::to_writer(writer, self)?;

            Ok(())
        }

        pub fn load_from(p: impl AsRef<Path>) -> Result<Self> {
            let reader = BufReader::new(File::open(p.as_ref())?);
            let kohonen = serde_json::from_reader(reader)?;

            Ok(kohonen)
        }
    }
}
