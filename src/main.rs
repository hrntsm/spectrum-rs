use std::fs::File;
use std::io::BufReader;

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
    let beta = 0.25;
    
    let mut period = Vec::new();
    let mut max_acc_vector = Vec::new();
    let mut max_vel_vector = Vec::new();
    let mut max_dis_vector = Vec::new();

    for acc in acceleration {
        let a0 = acc * eq;
        acc0.push(a0);
        f.push(-m * a0);
    }

    let mut i = 0 as f32;

    while i <= t_total / dt_total {
        let mut acc1 = 0 as f32;
        let mut vel1 = 0 as f32;
        let mut dis1 = 0 as f32;
        let mut max_acc = 0 as f32;
        let mut max_vel = 0 as f32;
        let mut max_dis = 0 as f32;

        let acc = Vec::new();
        let vel = Vec::new();
        let dis = Vec::new();

        let t = 0.01 * i;
        let k = 4.0 * std::f32::consts::PI.powi(2) * m / t.powi(2);
        let c = 2.0 * h * (k * m).sqrt();

        let mut j = 0;
        while j < acc0.len() {
            let acc2 = (f[j]
                - c * (vel1 + 0.5 * dt * acc1)
                - k * (dis1 + dt * vel1 + (0.5 - beta) * dt * dt * acc1))
                / (m + c * 0.5 * dt + k * beta * dt * dt);
            let vel2 = vel1 + 0.5 * dt * (acc1 + acc2);
            let dis2 = dis1 + dt * vel1 + (0.5 - beta) * dt * dt * acc1 + beta * dt * dt * acc2;

            acc.push(acc2 + acc0[j]);
            vel.push(vel2);
            dis.push(dis2);

            acc1 = acc2;
            vel1 = vel2;
            dis1 = dis2;

            j += 1;
        }

        j = 0;
        while j < acc.len() {
            max_acc = max_acc.max(acc[j].abs());
            max_vel = max_vel.max(vel[j].abs());
            max_dis = max_dis.max(dis[j].abs());

            j += 1;
        }

        period.push(t);
        max_acc_vector.push(max_acc);
        max_vel_vector.push(max_vel);
        max_dis_vector.push(max_dis);

        i += 1.0;
    }

    let mut results = Vec::new();
    results.push(period);
    results.push(max_acc_vector);
    results.push(max_vel_vector);
    results.push(max_dis_vector);

    return results;
}

fn main() {
    let h = 0.05;
    let dt = 0.02;
    let input_file = String::from("./test.txt");
    let output_file = String::from("./result.txt");
    
    let acceleration = read_file(input_file);
    let result = spectrum(acceleration, dt, h);

    let i = 0;
    for r in result {

    }
}
