pub mod build;
pub mod dict_term;
pub mod doc_vec;
pub mod lock_step;
pub mod presets;
pub mod vector;
pub mod weight;

use dict_term::DictTerm;
use doc_vec::DocVector;
use index_framework::{
    traits::{backend::Backend, deser::DeSer, dictionary::IndexDictionary},
    Index,
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use vector::Vector;

/// A generic VSM index type
#[derive(Serialize, Deserialize)]
pub struct VSMIndexGen<B, D, M> {
    index: Index<B, DictTerm, DocVector<D>>,
    metadata: Option<M>,
}

impl<B, D, M> VSMIndexGen<B, D, M>
where
    B: Backend<DictTerm, DocVector<D>>,
    D: DeSer,
{
    #[inline]
    pub(crate) fn new(index: Index<B, DictTerm, DocVector<D>>, metadata: Option<M>) -> Self {
        Self { index, metadata }
    }

    #[inline]
    pub fn get_metadata(&self) -> Option<&M> {
        self.metadata.as_ref()
    }

    #[inline]
    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    /// Build a new query vector with given terms. Returns `None` if
    /// the vector is empty which is the case if no term could be found
    /// in the indexes dictionary
    pub fn new_query<I, T>(&self, terms: I) -> Option<Vector>
    where
        I: IntoIterator<Item = T>,
        T: Into<DictTerm>,
    {
        self.new_query_with_weigts(terms.into_iter().map(|i| (i, 1.0)))
    }

    /// Similar to `new_query` but allows custom weights
    pub fn new_query_with_weigts<I, T>(&self, terms: I) -> Option<Vector>
    where
        I: IntoIterator<Item = (T, f32)>,
        T: Into<DictTerm>,
    {
        let t_ids = terms
            .into_iter()
            .filter_map(|(t, w)| Some((self.index.dict().get_id(t)?, w)));
        let vec = Vector::create_new_raw(t_ids);
        (!vec.is_empty()).then(|| vec)
    }
}

impl<B, D, M> Deref for VSMIndexGen<B, D, M>
where
    B: Backend<DictTerm, DocVector<D>>,
    D: DeSer,
{
    type Target = Index<B, DictTerm, DocVector<D>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.index
    }
}
