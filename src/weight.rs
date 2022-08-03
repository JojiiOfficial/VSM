pub trait TermWeight {
    /// Calculates the weight of a term.
    /// tf - Term frequency (frequency in the given document)
    /// df - Document frequency (document count with this term)
    fn weight(&self, current: f32, tf: usize, df: usize, total_docs: usize) -> f32;
}

/// Normal TF.IDF (normaized)
pub struct TFIDF;
impl TermWeight for TFIDF {
    #[inline]
    fn weight(&self, _current: f32, tf: usize, df: usize, total_docs: usize) -> f32 {
        let idf = (total_docs as f32 / df as f32).log10();
        ((tf as f32).log10() + 1.0) * idf
    }
}

/// Normal TF.IDF (normaized)
pub struct SmoothTFIDF;
impl TermWeight for SmoothTFIDF {
    #[inline]
    fn weight(&self, _current: f32, tf: usize, df: usize, total_docs: usize) -> f32 {
        let idf = (total_docs as f32 / (df as f32 + 1.0)).log10();
        ((tf as f32).log10() + 1.0) * idf
    }
}

/// Normalized term frequency
pub struct NormalizedTF;
impl TermWeight for NormalizedTF {
    #[inline]
    fn weight(&self, _current: f32, tf: usize, _df: usize, _total_docs: usize) -> f32 {
        (tf as f32).log10() + 1.0
    }
}

/// Normalized term frequency
pub struct ProbalisticTFIDF;
impl TermWeight for ProbalisticTFIDF {
    #[inline]
    fn weight(&self, _current: f32, tf: usize, _df: usize, total_docs: usize) -> f32 {
        let idf = (total_docs as f32 - tf as f32) / total_docs as f32;
        ((tf as f32).log10() + 1.0) * idf
    }
}
