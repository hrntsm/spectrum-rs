mod spectrum;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Calculate the response spectrum for the input waveform.")]
struct Settings {
    #[structopt(short = "h", help = "damping ratio")]
    pub h: Option<f32>,

    #[structopt(short = "dt", help = "Input file time increment")]
    pub dt: Option<f32>,

    #[structopt(short = "i", help = "input file path")]
    pub input: Option<String>,

    #[structopt(short = "o", help = "output file path")]
    pub output: Option<String>,
}

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

fn write_file(file_name: String, data: Vec<Vec<f32>>) {
    let path = Path::new(&file_name);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => panic!("couldn't create file."),
    };

    for i in 1..data[0].len() {
        let period = data[0][i].to_string();
        let acc = data[1][i].to_string();
        let vel = data[2][i].to_string();
        let dis = data[3][i].to_string();
        let line = format!("{} {} {} {}\n", period, acc, vel, dis);

        file.write_all(line.as_bytes()).unwrap();
    }
}

fn main() {
    let settings = Settings::from_args();
    let h = settings.h.unwrap_or(0.05);
    let dt = settings.dt.unwrap_or(0.02);
    let input_file = match settings.input {
        Some(i) => i,
        None => String::from("./input.txt"),
    };
    let output_file = match settings.output {
        Some(o) => o,
        None => String::from("./result.txt"),
    };

    let acceleration = read_file(input_file);
    let result = spectrum::calc(acceleration, dt, h);

    write_file(output_file, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spectrum_result() {
        let h = 0.05;
        let dt = 0.02;
        let input_file = String::from("./test.txt");

        let acceleration = read_file(input_file);
        let result = spectrum::calc(acceleration, dt, h);

        assert!(result[0][10] < 0.100 && result[0][10] > 0.098);
        assert!(result[1][10] < 5.160 && result[1][10] > 5.150);
        assert!(result[2][10] < 0.062 && result[2][10] > 0.061);
        assert!(result[3][10] < 0.002 && result[3][10] > 0.001);
    }
}
