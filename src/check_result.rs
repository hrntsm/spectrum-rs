use super::*;

#[test]
fn spectrum_result() {
    let h = 0.05;
    let dt = 0.02;
    let input_file = String::from("./files/input.txt");

    let acceleration = io::read_file(input_file);
    let result = solver::calc(acceleration, dt, h);

    assert!(result[0][10] < 0.100 && result[0][10] > 0.098);
    assert!(result[1][10] < 5.160 && result[1][10] > 5.150);
    assert!(result[2][10] < 0.062 && result[2][10] > 0.061);
    assert!(result[3][10] < 0.002 && result[3][10] > 0.001);
}
