use criterion::{black_box, criterion_group, criterion_main, Criterion};
use markdownit::{tokenizer, parser, SecondaryToken}; // Ensure correct import

// Criterion::default().warm_up_time(std::time::Duration::from_secs(5));
fn bench_tokenizer(c: &mut Criterion) {
    let input = std::fs::read_to_string("./sample.md").unwrap();

    c.bench_function("tokenizer", |b| {
        b.iter(|| tokenizer(black_box(&input)))
    });
}

fn bench_parser(c: &mut Criterion) {
    let input = std::fs::read_to_string("./sample.md").unwrap();
    let tokens: Vec<SecondaryToken> = tokenizer(&input);

    c.bench_function("parser", |b| {
        b.iter(|| parser(black_box(&tokens)))
    });
}

criterion_group!(benches, bench_tokenizer, bench_parser);
criterion_main!(benches);
