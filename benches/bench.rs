use criterion::{Criterion, criterion_group, criterion_main};
use MyAOC::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("complete AOC", |b| b.iter(|| test_aoc()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[inline]
fn test_aoc() {
    Day1::exercice::bench();
    Day2::exercice::bench();
    Day3::exercice::bench();
    Day4::exercice::bench();
    Day5::exercice::bench();
    Day6::exercice::bench();
    Day7::exercice::bench();
    Day8::exercice::bench();
    Day9::exercice::bench();
}
