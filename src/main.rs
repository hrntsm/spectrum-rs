mod spectrum;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

fn read_file(file_name: String) -> Vec<f32> {
    let mut data = Vec::new();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        data.push(line.parse().unwrap());
    }
    data
}

fn main() {
    let h = 0.05;
    let dt = 0.02;
    let input_file = String::from("./test.txt");
    let output_file = String::from("./result.txt");

    let acceleration = read_file(input_file);
    let result = spectrum::calc(acceleration, dt, h);

    let mut writer = BufWriter::new(File::create(output_file)?);
    for r in result {
        let byte = result?;
        writer.write_all(&[byte])?;
    }
}
