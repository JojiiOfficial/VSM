use intersect_iter::TupleIntersect;
use serde::{Deserialize, Serialize};
use std::slice::IterMut;

/// A n-dimensional sparse vector
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vector {
    /// Dimensions mapped to values
    inner: Vec<(u32, f32)>,
    /// Length of the vector
    length: f32,
}

impl Vector {
    /// Creates a new empty vector
    #[inline]
    pub fn new_empty() -> Vector {
        Vector {
            inner: vec![],
            length: 0.0,
        }
    }

    /// Create a new Vec from raw values. Values don't have to be ordered
    #[inline]
    pub fn create_new_raw<I>(sparse: I) -> Self
    where
        I: IntoIterator<Item = (u32, f32)>,
    {
        let mut vec = Self {
            inner: sparse.into_iter().collect(),
            length: 0.0,
        };
        vec.update();
        vec
    }

    /// Create a new Vec from raw values. `sparse` must be sorted by dimensions
    #[inline]
    pub fn new_raw(sparse: Vec<(u32, f32)>, length: f32) -> Self {
        Self {
            inner: sparse,
            length,
        }
    }

    /// Calculates the dice similarity between two 'vectors'
    #[inline]
    pub fn dice(&self, other: &Vector) -> f32 {
        if !self.could_overlap(other) {
            return 0.0;
        }

        let both = TupleIntersect::new(self.inner.iter().copied(), other.inner.iter().copied())
            .count() as f32;
        (2.0 * both) / (self.inner.len() as f32 + other.inner.len() as f32)
    }

    /// Calculates the cosine similarity between two vectors
    #[inline]
    pub fn cosine(&self, other: &Vector) -> f32 {
        let sc = self.scalar(other);
        if sc == 0.0 {
            return 0.0;
        }
        sc / (self.length * other.length)
    }

    /// Returns a mutable reference to the inner vector
    #[inline]
    pub fn sparse_vec_mut(&mut self) -> &mut Vec<(u32, f32)> {
        &mut self.inner
    }

    /// Returns the reference to the inner vector
    #[inline]
    pub fn sparse(&self) -> &Vec<(u32, f32)> {
        &self.inner
    }

    /// Returns true if the vector is zero
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator over all overlapping dimensions and their values
    #[inline]
    pub fn overlapping<'a>(
        &'a self,
        other: &'a Vector,
    ) -> impl Iterator<Item = (u32, f32, f32)> + 'a {
        TupleIntersect::new(self.inner.iter().copied(), other.inner.iter().copied())
    }

    /// Returns `true` if both vectors have at least one dimension in common
    #[inline]
    pub fn overlaps_with(&self, other: &Vector) -> bool {
        if !self.could_overlap(other) {
            return false;
        }

        TupleIntersect::new(self.inner.iter().copied(), other.inner.iter().copied())
            .next()
            .is_some()
    }

    /// Returns `true` if both vectors could potentionally have overlapping vectors.
    /// This is just an indication whether they could overlap and therefore faster than
    /// `overlaps_with` but less accurate
    #[inline]
    pub fn could_overlap(&self, other: &Vector) -> bool {
        let cant_overlap = self.is_empty()
            || other.is_empty()
            || self.first_indice() > other.last_indice()
            || self.last_indice() < other.first_indice();

        !cant_overlap
    }

    /// Returns the amount of dimensions the vector uses
    #[inline]
    pub fn dimen_count(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if vector has a certain dimension
    #[inline]
    pub fn has_dim(&self, dim: u32) -> bool {
        self.inner.binary_search_by(|a| a.0.cmp(&dim)).is_ok()
    }

    /// Update the vector values
    #[inline]
    pub fn update(&mut self) {
        // Calculate new vector length
        self.length = self.calc_len();

        // Vector indices always have to be sorted
        self.sort();
    }

    /// Get the length of the vector
    #[inline]
    pub fn get_length(&self) -> f32 {
        self.length
    }

    /// Returns an iterator over all dimensions of the vector
    #[inline]
    pub fn dimensions(&self) -> impl Iterator<Item = u32> + '_ {
        self.inner.iter().map(|i| i.0)
    }

    /// Returns an iterator over all weights of the vector
    #[inline]
    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.inner.iter().map(|i| i.1)
    }

    /// Deletes a given dimension and its value from the vector
    #[inline]
    pub fn delete_dim(&mut self, dim: u32) {
        // dimensions are unique so we can do binary search and remove
        // only its result
        if let Ok(pos) = self.inner.binary_search_by(|(d, _)| d.cmp(&dim)) {
            self.inner.remove(pos);
        }
    }

    #[inline]
    pub fn sparse_iter_mut(&mut self) -> IterMut<(u32, f32)> {
        self.inner.iter_mut()
    }

    /// Returns the scalar product of self and `other`
    #[inline]
    fn scalar(&self, other: &Self) -> f32 {
        TupleIntersect::new(self.inner.iter().copied(), other.inner.iter().copied())
            .map(|(_, a, b)| a * b)
            .sum()
    }

    /// Returns the value of the given dimension if exists
    #[inline]
    pub fn get_dim(&self, dim: u32) -> Option<f32> {
        self.inner
            .binary_search_by(|(a, _)| a.cmp(&dim))
            .ok()
            .map(|i| self.inner[i].1)
    }

    #[inline]
    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut (u32, f32)> {
        self.inner.iter_mut()
    }

    /// Calculate the vector length
    #[inline]
    fn calc_len(&self) -> f32 {
        self.inner
            .iter()
            .map(|(_, i)| i.powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// Sort the Vec<> by the dimensions
    #[inline]
    fn sort(&mut self) {
        self.inner.sort_by(|a, b| a.0.cmp(&b.0));
        self.inner.dedup_by(|a, b| a.0 == b.0);
    }

    #[inline]
    fn last_indice(&self) -> u32 {
        self.inner.last().unwrap().0
    }

    #[inline]
    fn first_indice(&self) -> u32 {
        self.inner.first().unwrap().0
    }
}

impl PartialEq for Vector {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for Vector {}
