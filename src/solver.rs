pub fn calc(acceleration: Vec<f32>, dt: f32, h: f32) -> Vec<Vec<f32>> {
    let mass = 100_f32;
    let eq = 0.01;
    let t_total = 5_f32;
    let dt_total = 0.01;
    let beta = 0.25;

    let mut acc0 = Vec::new();
    let mut force = Vec::new();

    let mut period = Vec::new();
    let mut max_acc_vector = Vec::new();
    let mut max_vel_vector = Vec::new();
    let mut max_dis_vector = Vec::new();

    for acc in acceleration {
        let a0 = acc * eq;
        acc0.push(a0);
        force.push(-mass * a0);
    }

    let mut i = 0 as f32;
    while i <= t_total / dt_total {
        let mut acc1 = 0 as f32;
        let mut vel1 = 0 as f32;
        let mut dis1 = 0 as f32;
        let mut max_acc = 0 as f32;
        let mut max_vel = 0 as f32;
        let mut max_dis = 0 as f32;

        let mut acc = Vec::new();
        let mut vel = Vec::new();
        let mut dis = Vec::new();

        let t = 0.01 * i;
        let k = 4.0 * std::f32::consts::PI.powi(2) * mass / t.powi(2);
        let c = 2.0 * h * (k * mass).sqrt();

        for j in 0..acc0.len() - 1 {
            let acc2 = (force[j]
                - c * (vel1 + 0.5 * dt * acc1)
                - k * (dis1 + dt * vel1 + (0.5 - beta) * dt * dt * acc1))
                / (mass + c * 0.5 * dt + k * beta * dt * dt);
            let vel2 = vel1 + 0.5 * dt * (acc1 + acc2);
            let dis2 = dis1 + dt * vel1 + (0.5 - beta) * dt * dt * acc1 + beta * dt * dt * acc2;

            acc.push(acc2 + acc0[j]);
            vel.push(vel2);
            dis.push(dis2);

            acc1 = acc2;
            vel1 = vel2;
            dis1 = dis2;
        }

        for j in 0..acc.len() - 1 {
            max_acc = max_acc.max(acc[j].abs());
            max_vel = max_vel.max(vel[j].abs());
            max_dis = max_dis.max(dis[j].abs());
        }

        period.push(t);
        max_acc_vector.push(max_acc);
        max_vel_vector.push(max_vel);
        max_dis_vector.push(max_dis);

        i += 1.0;
    }

    let results = vec![period, max_acc_vector, max_vel_vector, max_dis_vector];
    results
}
