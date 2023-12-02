use day_02::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let _ = part1::process(divan::black_box(include_str!("../data.txt")));
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../data.txt",))).unwrap();
}
