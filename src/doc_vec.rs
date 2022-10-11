use serde::{Deserialize, Serialize};
use sparse_vec::SpVec32;
use std::ops::{Deref, DerefMut};

/// A document vector
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocVector<D> {
    document: D,
    vec: SpVec32,
}

impl<D> DocVector<D> {
    #[inline]
    pub fn new(document: D, vec: SpVec32) -> Self {
        Self { document, vec }
    }

    #[inline]
    pub fn document(&self) -> &D {
        &self.document
    }

    #[inline]
    pub fn vec(&self) -> &SpVec32 {
        &self.vec
    }

    #[inline]
    pub fn into_vec(self) -> SpVec32 {
        self.vec
    }

    #[inline]
    pub fn into_doc(self) -> D {
        self.document
    }

    #[inline]
    pub fn doc_mut(&mut self) -> &mut D {
        &mut self.document
    }

    #[inline]
    pub fn vec_mut(&mut self) -> &mut SpVec32 {
        &mut self.vec
    }
}

impl<D> Deref for DocVector<D> {
    type Target = SpVec32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<D> DerefMut for DocVector<D> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
