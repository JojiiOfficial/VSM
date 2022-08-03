use index_framework::traits::{
    backend::Backend, dictionary::IndexDictionary, postings::IndexPostings, storage::IndexStorage,
};
use vsm::{
    build::Builder,
    presets::{VSMIndexSimple, VSMIndexSimpleBuilder},
    weight::TFIDF,
};

#[test]
fn test_build() {
    let mut builder: VSMIndexSimpleBuilder<u32> = Builder::new();

    builder.set_weight(TFIDF);

    builder.insert_vec(1, &["lol", "rise"]);
    builder.insert_vec(2, &["some", "test", "document"]);

    let index: VSMIndexSimple<u32> = builder.build();

    assert_eq!(index.dict().len(), 5);
    assert_eq!(index.storage().len(), 2);

    for term in ["lol", "rise"] {
        let term = index.dict().get_id(term).unwrap();
        let vecs = index.postings(0).unwrap().get_posting(term);
        assert_eq!(vecs, vec![0]);
    }

    for term in ["some", "test", "document"] {
        let term = index.dict().get_id(term).unwrap();
        let vecs = index.postings(0).unwrap().get_posting(term);
        assert_eq!(vecs, vec![1]);
    }
}
