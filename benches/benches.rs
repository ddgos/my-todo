use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mtd::Document;

fn bench_doc_1(c: &mut Criterion) {
    c.bench_function("doc_1", |b| {
        b.iter(|| black_box(include_str!(r#"../test_docs/doc_1.mtd"#)).parse::<Document>())
    });
}

fn bench_doc_2(c: &mut Criterion) {
    c.bench_function("doc_2", |b| {
        b.iter(|| black_box(include_str!(r#"../test_docs/doc_2.mtd"#)).parse::<Document>())
    });
}

criterion_group! {parse_benches, bench_doc_1, bench_doc_2}
criterion_main!(parse_benches);
