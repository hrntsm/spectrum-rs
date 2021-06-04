use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
}

fn read_file(file_name: String) -> Vec<f32> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);
    let mut data = Vec::new();
    let col1;
    data.push(col1)
}

fn spectrum(acceleration: Vec<f32>, dt: f32, h: f32) -> Vec<Vec<f32>> {
    let m = 100 as f32;
    let eq = 0.01;
    let acc0 = Vec::new();
    let f = Vec::new();
    let t_total = 5 as f32;
    let dt_total = 0.01;

    for acc in acceleration{
        let a0 = acc * eq;
        acc0.push(a0);
        f.push(-m * a0);
    }
    
    let i = 0 as f32;

    while i <= t_total / dt_total {
        let mut acc1 = 0 as f32;
        let mut vec1 = 0 as f32;
        let mut dis1 = 0 as f32;
        let mut max_acc = 0 as f32;
        let mut max_vel = 0 as f32;
        let mut max_dis = 0 as f32;


        
        i += 1.0;
    }

    let a = Vec<Vec<f32>>::new()
}
