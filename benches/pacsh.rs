use std::io;

fn main() {
    divan::main();
}

#[divan::bench]
fn run() {
    pacsh::run().unwrap()
}

#[divan::bench]
fn run_to_sink() {
    let sink = io::sink();
    pacsh::run_to(sink).unwrap()
}
