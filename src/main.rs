mod io;
mod solver;
mod check_result;

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

fn main() {
    let settings = Settings::from_args();
    let h = settings.h.unwrap_or(0.05);
    let dt = settings.dt.unwrap_or(0.02);
    let input_file = match settings.input {
        Some(i) => i,
        None => String::from("./files/input.txt"),
    };
    let output_file = match settings.output {
        Some(o) => o,
        None => String::from("./files/result.txt"),
    };

    let acceleration = io::read_file(input_file);
    let result = solver::calc(acceleration, dt, h);

    io::write_file(output_file, result);
}
