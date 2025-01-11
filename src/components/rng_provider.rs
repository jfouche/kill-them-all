use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;

/// An provider of values based an values weight
pub struct RngKindProvider<T> {
    weights: HashMap<T, usize>,
    filters: Vec<T>,
}

impl<T> Default for RngKindProvider<T> {
    fn default() -> Self {
        Self {
            weights: HashMap::default(),
            filters: Vec::default(),
        }
    }
}

impl<T> RngKindProvider<T>
where
    T: Copy + Eq + std::hash::Hash,
{
    /// Add an kind based on its weigh.
    /// The hight the weight is, the most probable it will appear.
    pub fn add(&mut self, kind: T, weight: usize) -> &mut Self {
        self.weights.insert(kind, weight);
        self
    }

    /// generate a rand value, removing the option to select it next time.
    pub fn gen(&mut self, rng: &mut ThreadRng) -> Option<T> {
        let mut remaing = self
            .weights
            .iter()
            .filter(|(v, _n)| !self.filters.contains(v));

        // Get random value
        let sum = remaing.clone().map(|(_k, n)| n).sum();
        if sum == 0 {
            // No more value possible
            return None;
        }
        let sel = rng.gen_range(0..sum);

        // find where is the value in the weights
        let mut i = 0;
        let value = remaing
            .find(|(_v, n)| {
                i += *n;
                i > sel
            })
            .map(|(v, _n)| *v);

        if let Some(v) = value {
            // Remove the found value for the next time
            self.filters.push(v);
        }
        value
    }
}
