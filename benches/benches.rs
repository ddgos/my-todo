use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mtd::{Document, Iteration, Preamble, Task, TaskStatus};

fn bench_parse_doc_1(c: &mut Criterion) {
    c.bench_function("parse_doc_1", |b| {
        b.iter(|| black_box(include_str!(r#"../test_docs/doc_1.mtd"#)).parse::<Document>())
    });
}

fn bench_parse_doc_2(c: &mut Criterion) {
    c.bench_function("parse_doc_2", |b| {
        b.iter(|| black_box(include_str!(r#"../test_docs/doc_2.mtd"#)).parse::<Document>())
    });
}

fn bench_display_doc_1(c: &mut Criterion) {
    let doc_1: Document = Document {
        preamble: Some(Preamble {
            content: "# Preamble\n\nI am the preamble!".to_string(),
        }),
        iterations: vec![
            (
                0,
                Iteration {
                    tasks: vec![
                        (
                            1,
                            Task {
                                status: TaskStatus::Incomplete,
                                description: "unstarted".to_string(),
                            },
                        ),
                        (
                            2,
                            Task {
                                status: TaskStatus::Complete,
                                description: "complete".to_string(),
                            },
                        ),
                        (
                            3,
                            Task {
                                status: TaskStatus::Cancelled,
                                description: "cancelled".to_string(),
                            },
                        ),
                    ],
                },
            ),
            (
                1,
                Iteration {
                    tasks: vec![(
                        1,
                        Task {
                            status: TaskStatus::Incomplete,
                            description: "next iteration".to_string(),
                        },
                    )],
                },
            ),
        ],
    };
    c.bench_function("display_doc_1", |b| {
        b.iter(|| format!("{}", black_box(doc_1.clone())))
    });
}

fn bench_display_doc_2(c: &mut Criterion) {
    let doc_2: Document = Document {
        preamble: None,
        iterations: vec![
            (
                0,
                Iteration {
                    tasks: vec![
                        (
                            1,
                            Task {
                                status: TaskStatus::Incomplete,
                                description: "unstarted".to_string(),
                            },
                        ),
                        (
                            2,
                            Task {
                                status: TaskStatus::Complete,
                                description: "complete".to_string(),
                            },
                        ),
                        (
                            3,
                            Task {
                                status: TaskStatus::Cancelled,
                                description: "cancelled".to_string(),
                            },
                        ),
                    ],
                },
            ),
            (1, Iteration { tasks: Vec::new() }),
        ],
    };
    c.bench_function("display_doc_2", |b| {
        b.iter(|| format!("{}", black_box(doc_2.clone())))
    });
}

criterion_group! {parse_benches, bench_parse_doc_1, bench_parse_doc_2}
criterion_group! {display_benches, bench_display_doc_1, bench_display_doc_2 }
criterion_main!(parse_benches, display_benches);
