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
    traits::{backend::Backend, deser::DeSer},
    Index,
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

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
