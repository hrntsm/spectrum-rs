mod spectrum;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

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

    let path = Path::new(&output_file);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create file."),
    };

    for i in 0..result[0].len() {
        let period = result[0][i].to_string();
        let acc = result[1][i].to_string();
        let vel = result[2][i].to_string();
        let dis = result[3][i].to_string();
        let line = format!( "{} {} {} {}\n", period, acc, vel, dis);

        file.write_all( line.as_bytes());
    }
}
