use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mega_store_search::index::InvertedIndex;
use mega_store_search::tokenizer::tokenize;

fn generate_names(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("produto {}", i)).collect()
}

fn bench_index(c: &mut Criterion) {
    let names = generate_names(10_000);
    c.bench_function("index 10k products", |b| {
        b.iter(|| {
            let mut idx = InvertedIndex::new();
            for (i, name) in names.iter().enumerate() {
                idx.insert_text(black_box(name), i as u64);
            }
        })
    });
}

fn bench_query_without_cache(c: &mut Criterion) {
    let mut idx = InvertedIndex::new();
    for i in 0..10_000 {
        idx.insert_text(&format!("produto {} mouse", i), i as u64);
    }
    c.bench_function("query 'mouse' (sem cache)", |b| {
        b.iter(|| {
            let toks = tokenize(black_box("mouse"));
            let _ = idx.query(&toks);
        })
    });
}
