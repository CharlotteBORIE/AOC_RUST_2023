use criterion::{Criterion, criterion_group, criterion_main};
use MyAOC::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("complete AOC", |b| b.iter(|| Day14::exercice::bench()));
}

fn bench(c: &mut Criterion) {

    let mut group = c.benchmark_group("All");
    group.bench_function("Day1 p1", |b| b.iter(|| { Day1::exercice::bench1(); }));
    group.bench_function("Day1 p2", |b| b.iter(|| { Day1::exercice::bench2(); }));
    group.bench_function("Day2 p1", |b| b.iter(|| { Day2::exercice::bench1(); }));
    group.bench_function("Day2 p2", |b| b.iter(|| { Day2::exercice::bench2(); }));
    group.bench_function("Day3 p1", |b| b.iter(|| { Day3::exercice::bench1(); }));
    group.bench_function("Day3 p2", |b| b.iter(|| { Day3::exercice::bench2(); }));
    group.bench_function("Day4 p1", |b| b.iter(|| { Day4::exercice::bench1(); }));
    group.bench_function("Day4 p2", |b| b.iter(|| { Day4::exercice::bench2(); }));
    group.bench_function("Day5 p1", |b| b.iter(|| { Day5::exercice::bench1(); }));
    group.bench_function("Day5 p2", |b| b.iter(|| { Day5::exercice::bench2(); }));
    group.bench_function("Day6 p1", |b| b.iter(|| { Day6::exercice::bench1(); }));
    group.bench_function("Day6 p2", |b| b.iter(|| { Day6::exercice::bench2(); }));
    group.bench_function("Day7 p1", |b| b.iter(|| { Day7::exercice::bench1(); }));
    group.bench_function("Day7 p2", |b| b.iter(|| { Day7::exercice::bench2(); }));
    group.bench_function("Day8 p1", |b| b.iter(|| { Day8::exercice::bench1(); }));
    group.bench_function("Day8 p2", |b| b.iter(|| { Day8::exercice::bench2(); }));
    group.bench_function("Day9 p1", |b| b.iter(|| { Day9::exercice::bench1(); }));
    group.bench_function("Day9 p2", |b| b.iter(|| { Day9::exercice::bench2(); }));
    group.bench_function("Day10 p1", |b| b.iter(|| { Day10::exercice::bench1(); }));
    group.bench_function("Day10 p2", |b| b.iter(|| { Day10::exercice::bench2(); }));
    group.bench_function("Day11 p1", |b| b.iter(|| { Day11::exercice::bench1(); }));
    group.bench_function("Day11 p2", |b| b.iter(|| { Day11::exercice::bench2(); }));
    group.bench_function("Day12 p1", |b| b.iter(|| { Day12::exercice::bench1(); }));
    //group.bench_function("Day12 p2", |b| b.iter(|| { Day12::exercice::bench2(); }));
    group.bench_function("Day13 p1", |b| b.iter(|| { Day13::exercice::bench1(); }));
    group.bench_function("Day13 p2", |b| b.iter(|| { Day13::exercice::bench2(); }));
    group.bench_function("Day14 p1", |b| b.iter(|| { Day14::exercice::bench1(); }));
    group.bench_function("Day14 p2", |b| b.iter(|| { Day14::exercice::bench2(); }));
}

criterion_group!(benches, bench);
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
    Day10::exercice::bench();
    Day11::exercice::bench();
}
