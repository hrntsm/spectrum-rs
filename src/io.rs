use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn read_file(file_name: String) -> Vec<f32> {
    let mut data = Vec::new();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        data.push(line.parse().unwrap());
    }
    data
}

pub fn write_file(file_name: String, data: Vec<Vec<f32>>) {
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
