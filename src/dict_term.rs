use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

/// Dictionary term in the VSM index
#[derive(Serialize, Deserialize, Clone)]
pub struct DictTerm {
    pub(crate) term: String,
    pub(crate) frequency: f32,
}

impl DictTerm {
    /// Creates a new DictTerm with 0.0 as frequency value
    #[inline]
    pub fn new(term: String) -> Self {
        Self {
            term,
            frequency: 0.0,
        }
    }

    /// Creates a new DictTerm with a custom frequency value
    #[inline]
    pub fn with_frequency(term: String, frequency: f32) -> Self {
        Self { term, frequency }
    }

    #[inline]
    pub fn term(&self) -> &str {
        self.term.as_ref()
    }

    #[inline]
    pub fn frequency(&self) -> f32 {
        self.frequency
    }
}

impl Hash for DictTerm {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.term.hash(state);
    }
}

impl PartialOrd for DictTerm {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.term.partial_cmp(&other.term)
    }
}

impl PartialEq for DictTerm {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.term == other.term
    }
}

impl Eq for DictTerm {}

impl Ord for DictTerm {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.term.cmp(&other.term)
    }
}

impl<U: AsRef<str>> From<U> for DictTerm {
    #[inline]
    fn from(s: U) -> Self {
        Self::new(s.as_ref().to_string())
    }
}
