use criterion::{black_box, criterion_group, criterion_main, Criterion};

const REGION: &[u8] = include_bytes!("r.0.0.mca");

fn mca() {
    let parsed = mca::RegionReader::new(REGION).unwrap();
    parsed.get_chunk(0, 0).unwrap();
}

fn parser() {
    let parsed = mca_parser::Region::from_slice(&REGION).unwrap();
    parsed.get_chunk(0, 0).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mca_0_0", |b| b.iter(|| black_box(mca())));
    c.bench_function("parser_0_0", |b| b.iter(|| black_box(parser())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
