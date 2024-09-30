pub struct City {
    pub name: String,
    pub population: i64,
    pub country: String,
}

impl City {
    pub fn new(name: String, population: i64, country: String) -> Self {
        City {
            name,
            population,
            country,
        }
    }
}

fn start_sorting_thread(mut cities: Vec<City>) {
    std::thread::spawn(move || {
        cities.sort_by_key(|city| std::cmp::Reverse(city.population));
    });
}


#[cfg(test)]
mod tests {
    use super::*;

}
