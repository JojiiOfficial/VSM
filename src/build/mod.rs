use crate::{dict_term::DictTerm, doc_vec::DocVector, vector::Vector, VSMIndexGen};
use index_framework::{
    backend::memory::build::{options::BuildOption, MemIndexBuilder},
    traits::{
        backend::{Backend, NewBackend},
        build::{IndexBuilder, ItemMod},
        deser::DeSer,
        dictionary::BuildIndexDictionary,
        postings::BuildPostings,
        storage::BuildIndexStorage,
    },
};
use std::collections::HashMap;

/// Builder for VSM Indexes
pub struct Builder<B, S, DD, SS, PP> {
    builder: MemIndexBuilder<B, DictTerm, DocVector<S>, DD, SS, PP>,
    // Maps term-id to occurrence count
    term_freqs: HashMap<u32, u32>,
}

impl<B, S, DD, SS, PP> Builder<B, S, DD, SS, PP>
where
    B: Backend<DictTerm, DocVector<S>> + NewBackend<DictTerm, DocVector<S>>,
    S: DeSer,
    SS: BuildIndexStorage<DocVector<S>, Output = B::Storage> + ItemMod<S>,
    DD: BuildIndexDictionary<DictTerm, Output = B::Dict> + ItemMod<DictTerm>,
    PP: BuildPostings<Output = B::Postings, PostingList = Vec<u32>>,
{
    /// Creates a new index builder with a postings list length of 1
    #[inline]
    pub fn new() -> Self {
        Self::with_postings_len(1)
    }

    /// Creates a new index builder with a custom amount postings lists
    #[inline]
    pub fn with_postings_len(postings_len: usize) -> Self {
        let mut builder = MemIndexBuilder::with_postings_len(postings_len);
        builder.add_option(BuildOption::SortedPostings);
        let term_freqs = HashMap::new();
        Self {
            builder,
            term_freqs,
        }
    }

    /// Inserts a new document into the index builder.
    /// Returns `None` if no terms were passed
    pub fn insert_vec<I, U>(&mut self, doc: S, terms: I) -> Option<u32>
    where
        I: IntoIterator<Item = U>,
        U: Into<DictTerm>,
    {
        self.insert_vec_in_post(0, doc, terms)
    }

    /// Inserts a new document into the index builder with a given postings ID.
    /// Returns `None` if no terms were passed
    pub fn insert_vec_in_post<I, U>(&mut self, post_id: u32, doc: S, terms: I) -> Option<u32>
    where
        I: IntoIterator<Item = U>,
        U: Into<DictTerm>,
    {
        let mut ids = self.builder.terms_to_ids(terms);
        ids.sort_unstable();
        if ids.is_empty() {
            return None;
        }

        for id in &ids {
            *self.term_freqs.entry(*id).or_default() += 1;
        }

        let sparse: Vec<_> = ids.iter().map(|i| (*i, 1.0)).collect();
        let vec = Vector::create_new_raw(sparse);
        let doc_vec = DocVector::new(doc, vec);
        Some(self.builder.index_new(post_id, doc_vec, &ids))
    }

    /// Build the index
    pub fn build<M>(self) -> VSMIndexGen<B, S, M> {
        self.build_raw(None)
    }

    /// Build the index with custom metadata
    pub fn build_with_metadata<M>(self, metadata: M) -> VSMIndexGen<B, S, M> {
        self.build_raw(Some(metadata))
    }

    #[inline]
    fn build_raw<M>(mut self, metadata: Option<M>) -> VSMIndexGen<B, S, M> {
        self.process_terms();
        self.process_vecs();
        let index = self.builder.build();
        VSMIndexGen::new(index, metadata)
    }

    fn process_vecs(&mut self) {
        // TODO: add vector weight calc here
    }

    fn process_terms(&mut self) {
        for (id, freq) in &self.term_freqs {
            let mut term = self.builder.dict().get(*id).unwrap();
            term.frequency = *freq as f32;
            self.builder.dict_mut().set_item(*id, term);
        }
    }
}
