#[macro_use]
extern crate criterion;

use criterion::Criterion;

pub fn spectrum_result() {
    let h = 0.05;
    let dt = 0.02;
    let input_file = String::from("./files/input.txt");
    let output_file = String::from("./files/result.txt");

    let acceleration = spectrum::io::read_file(input_file);
    let result = spectrum::solver::calc(acceleration, dt, h);

    spectrum::io::write_file(output_file, result);
}

fn criteria_spectrum(c: &mut Criterion) {
    c.bench_function("spectrum", |b| b.iter(spectrum_result));
}

criterion_group!(benches, criteria_spectrum);
criterion_main!(benches);
