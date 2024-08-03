pub struct Fern {
    pub size: f64,
    pub growth_rate: f64, // Growth rate for each leaf
}

impl Fern {
    pub fn grow(&mut self) {
        self.size *= self.growth_rate + 1.0;
    }
}

/// Run simuration for some number of days
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
        println!("The fern is now {} day(s) old", fern.size);
    }
}
