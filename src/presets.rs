use crate::{build::Builder, dict_term::DictTerm, doc_vec::DocVector, VSMIndexGen};
use index_framework::backend::memory::{
    dict::default::Dictionary, postings::compressed::Postings, storage::default::Storage,
    MemBackend,
};

pub type VSMIndexSimple<D> = VSMIndexGen<
    MemBackend<DictTerm, DocVector<D>, Dictionary<DictTerm>, Storage<DocVector<D>>, Postings>,
    D,
    (),
>;

pub type VSMIndexSimpleBuilder<D> = Builder<
    MemBackend<DictTerm, DocVector<D>, Dictionary<DictTerm>, Storage<DocVector<D>>, Postings>,
    D,
    Dictionary<DictTerm>,
    Storage<DocVector<D>>,
    Postings,
>;
