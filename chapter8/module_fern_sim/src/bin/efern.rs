use fern_sim::{run_simulation, Fern};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.01,
    };
    run_simulation(&mut fern, 365);
    println!("final fern size {}", fern.size)
}
