use crate::{build::Builder, dict_term::DictTerm, doc_vec::DocVector, VSMIndexGen};
use index_framework::backend::memory::{
    dict::default::Dictionary,
    postings::compressed::Postings,
    storage::{c_u32::U32Storage, default::Storage},
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

/// Compressed and U32-optimized VSM index
pub type VSMIndexU32 = VSMIndexGen<
    MemBackend<DictTerm, DocVector<u32>, Dictionary<DictTerm>, U32Storage, Postings>,
    u32,
    (),
>;

pub type VSMIndexU32Builder = Builder<
    MemBackend<DictTerm, DocVector<u32>, Dictionary<DictTerm>, U32Storage, Postings>,
    u32,
    Dictionary<DictTerm>,
    U32Storage,
    Postings,
>;
