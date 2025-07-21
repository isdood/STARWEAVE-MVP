// #FF69B4 Similarity Performance Test
#![feature(test)]
extern crate test;

use starweave_mvp::concepts::{SimilarityEngine};
use ndarray::Array1;
use test::Bencher;

#[bench]
fn bench_similarity_search(b: &mut Bencher) {
    let engine = SimilarityEngine::new();
    let test_vector = Array1::from_vec(vec![0.85, -0.15, 0.45]);

    b.iter(|| {
        engine.find_best_match(&test_vector);
    });
}
