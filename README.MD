# Commands

    cargo clippy --all-features --all --tests -- -W clippy::all -W clippy::style -W clippy::complexity -W clippy::suspicious -W clippy::pedantic -W clippy::nursery -W clippy::perf -D warnings
    
    cargo bench --bench day-02 >> day-02/bench.txt

    cargo r --package day-02 --bin part1 && cargo r --package day-02 --bin part2 && cargo t --package day-02 && cargo t
