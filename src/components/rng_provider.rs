use rand::{rngs::ThreadRng, seq::IteratorRandom};
use std::collections::HashMap;

pub trait Generator<T> {
    fn generate(&self, rng: &mut ThreadRng) -> T;
}

pub struct RngKindProvider<K, T>
where
    K: Generator<T>,
{
    weights: HashMap<K, Vec<K>>,
    filters: Vec<K>,
    _phantom: std::marker::PhantomData<T>,
}

impl<K, T> Default for RngKindProvider<K, T>
where
    K: Generator<T>,
{
    fn default() -> Self {
        Self {
            weights: HashMap::default(),
            filters: Vec::default(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<K, T> RngKindProvider<K, T>
where
    K: Copy + Eq + std::hash::Hash + Generator<T>,
{
    pub fn add(&mut self, kind: K, weight: usize) -> &mut Self {
        self.weights.insert(kind, vec![kind; weight]);
        self
    }

    /// generate a rand upgrade, removing the option the select it next time
    pub fn gen(&mut self) -> Option<T> {
        let mut rng = rand::thread_rng();

        let kind = self
            .weights
            .iter()
            .filter(|(kind, _v)| !self.filters.contains(kind))
            .flat_map(|(_kind, v)| v)
            .choose(&mut rng)?;

        self.filters.push(*kind);
        let upgrade = kind.generate(&mut rng);
        Some(upgrade)
    }
}
