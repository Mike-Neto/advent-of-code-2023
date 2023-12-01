use day_01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let _ = part1::process(divan::black_box(include_str!("../data.txt",)));
}

#[divan::bench]
fn part2() {
    let _ = part2::process(divan::black_box(include_str!("../data.txt",)));
}
