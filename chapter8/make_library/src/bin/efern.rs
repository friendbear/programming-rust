
extern crate fern_sim;
use fern_sim::{Fern, run_simulation};

fn main() {
    let mut fern = Fern {
        size: 0.0,
        growth_rate: -1.001
    };
    run_simulation(&mut fern, 999);
    println!("final fern size: {}", fern.size);
}