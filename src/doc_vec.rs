use crate::vector::Vector;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// A document vector
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocVector<D> {
    document: D,
    vec: Vector,
}

impl<D> DocVector<D> {
    #[inline]
    pub fn new(document: D, vec: Vector) -> Self {
        Self { document, vec }
    }

    #[inline]
    pub fn document(&self) -> &D {
        &self.document
    }

    #[inline]
    pub fn vec(&self) -> &Vector {
        &self.vec
    }

    #[inline]
    pub fn into_vec(self) -> Vector {
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
    pub fn vec_mut(&mut self) -> &mut Vector {
        &mut self.vec
    }
}

impl<D> Deref for DocVector<D> {
    type Target = Vector;

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
