use crate::{dict_term::DictTerm, doc_vec::DocVector, weight::TermWeight, VSMIndexGen};
use index_framework::{
    backend::memory::build::{options::PostingsMod, MemIndexBuilder},
    traits::{
        backend::{Backend, NewBackend},
        build::{IndexBuilder, ItemMod},
        deser::DeSer,
        dictionary::BuildIndexDictionary,
        postings::BuildPostings,
        storage::BuildIndexStorage,
    },
};
use sparse_vec::{SpVec32, VecExt};
use std::collections::HashMap;

type MemBuilderType<B, S, DD, SS, PP> = MemIndexBuilder<B, DictTerm, DocVector<S>, DD, SS, PP>;

/// Builder for VSM Indexes
pub struct Builder<B, S, DD, SS, PP> {
    builder: MemBuilderType<B, S, DD, SS, PP>,

    // Maps term-id to occurrence count
    term_freqs_total: HashMap<u32, u32>,

    // Maps term frequencies for documents
    tf: HashMap<u32, HashMap<u32, u32>>,

    // Weighting variant
    weight_fn: Option<Box<dyn TermWeight>>,

    // Max length of a postings entry
    max_postings_len: usize,
}

impl<B, S, DD, SS, PP> Builder<B, S, DD, SS, PP>
where
    B: Backend<DictTerm, DocVector<S>> + NewBackend<DictTerm, DocVector<S>>,
    S: DeSer,
    SS: BuildIndexStorage<DocVector<S>, Output = B::Storage> + ItemMod<DocVector<S>>,
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
        let builder = MemIndexBuilder::with_postings_len(postings_len);
        Self {
            builder,
            term_freqs_total: HashMap::new(),
            tf: HashMap::new(),
            weight_fn: None,
            max_postings_len: 1000,
        }
    }

    #[inline]
    pub fn set_weight<W: TermWeight + 'static>(&mut self, w: W) {
        self.weight_fn = Some(Box::new(w));
    }

    #[inline]
    pub fn with_max_postings_len(mut self, max_postings_len: usize) -> Self {
        self.max_postings_len = max_postings_len;
        self
    }

    /// Inserts a new document into the index builder.
    /// Returns `None` if no terms were passed
    #[inline]
    pub fn insert_vec<I, U>(&mut self, doc: S, terms: I) -> Option<u32>
    where
        I: IntoIterator<Item = U>,
        U: Into<DictTerm>,
    {
        self.insert_vec_in_post(0, doc, terms)
    }

    /// Inserts a new document into the index builder with a given postings ID.
    /// Returns `None` if no terms were passed
    pub fn insert_vec_in_posts<I, U>(&mut self, post_ids: &[u32], doc: S, terms: I) -> Option<u32>
    where
        I: IntoIterator<Item = U>,
        U: Into<DictTerm>,
    {
        let mut ids = self.builder.terms_to_ids(terms);
        if ids.is_empty() {
            return None;
        }

        let mut term_freqs: HashMap<u32, u32> = HashMap::with_capacity(ids.len());

        for id in &ids {
            *self.term_freqs_total.entry(*id).or_default() += 1;
            *term_freqs.entry(*id).or_default() += 1;
        }

        ids.sort_unstable();
        ids.dedup();

        let vec = SpVec32::create_new_raw(ids.iter().map(|i| (*i, 1.0)));
        let doc_vec = DocVector::new(doc, vec);

        let item_id = self.builder.insert_item(doc_vec);

        for post_id in post_ids {
            self.builder.map(*post_id, item_id, &ids);
        }

        Some(item_id)
    }

    /// Inserts a new document into the index builder with a given postings ID.
    /// Returns `None` if no terms were passed
    #[inline]
    pub fn insert_vec_in_post<I, U>(&mut self, post_id: u32, doc: S, terms: I) -> Option<u32>
    where
        I: IntoIterator<Item = U>,
        U: Into<DictTerm>,
    {
        self.insert_vec_in_posts(&[post_id], doc, terms)
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
        self.process_postings();
        let index = self.builder.build();
        VSMIndexGen::new(index, metadata)
    }

    /// Sorts and compresses postings
    fn process_postings(&mut self) {
        let max_postings_len = self.max_postings_len;

        let pmd = PostingsMod::new(
            move |_pst_id, t_id, posts, builder: &MemBuilderType<B, S, DD, SS, PP>| {
                // Sort by vector weight for the given term. This makes retrieval faster
                // and allows removing unlikely vectors from the results
                let storage = &builder.storage;
                posts.sort_by(|a, b| {
                    let a = storage.get(*a).and_then(|a| a.get_dim(t_id as usize));
                    let b = storage.get(*b).and_then(|b| b.get_dim(t_id as usize));
                    a.unwrap().total_cmp(&b.unwrap()).reverse()
                });

                // Trim posts to improve query processing time
                if max_postings_len > 0 {
                    posts.truncate(max_postings_len);
                }
            },
        );

        self.builder.set_postings_mod(pmd);
    }

    fn process_vecs(&mut self) {
        let len = self.builder.storage().len() as u32;
        for i in 0..len {
            let mut vec = self.builder.storage().get(i).unwrap();
            self.calc_weight(i, vec.vec_mut());
            self.builder.storage.set_item(i, vec);
        }
    }

    #[inline]
    fn calc_weight(&self, id: u32, vec: &mut SpVec32) {
        let weight = match self.weight_fn.as_ref() {
            Some(w) => w,
            None => return,
        };

        let len = self.builder.storage.len();
        let tf_map = self.tf.get(&id).unwrap();

        for (d, w) in vec.iter_mut() {
            let df = self.df(*d);
            let tf = *tf_map.get(&d).expect("Term not found in frequencies") as usize;
            *w = (weight).weight(*w, tf, df, len);

            if *w == 0.0 {
                // TODO maybe un-index here and remove the value from the vec
                panic!("Weight reached zero!");
            }
        }
    }

    #[inline]
    fn df(&self, pos: u32) -> usize {
        let mut sum = 0;
        for pi in 0..self.builder.postings_count() {
            sum += self
                .builder
                .postings(pi)
                .unwrap()
                .get(&pos)
                .map(|i| i.len())
                .unwrap_or(0);
        }
        sum
    }

    fn process_terms(&mut self) {
        for (id, freq) in &self.term_freqs_total {
            let mut term = self.builder.dict().get(*id).unwrap();
            term.frequency = *freq as f32;
            self.builder.dict_mut().set_item(*id, term);
        }
    }
}
