use std::sync::{Arc, Mutex};
#[derive(Debug, Clone)]
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

// 引数で渡した値をソートする関数
pub fn start_sorting_thread(cities: &mut Vec<City>) {
    // Arc<Mutex<>> で可変な共有データを作成
    let cities_arc = Arc::new(Mutex::new(cities));

    // Arc をクローンして別スレッドに渡す
    let cities_clone = Arc::clone(&cities_arc);
    let join_handle = std::thread::spawn(move || {
        // 別スレッドから cities にアクセスし、ソートを行う
        let mut cities_lock = cities_clone.lock().unwrap();
        cities_lock.sort_by(|c1, c2| c2.population.cmp(&c1.population));
    });

    // スレッドが終了するまで待機
    join_handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

}
