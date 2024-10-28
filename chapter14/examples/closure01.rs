
use chapter14::City;
use std::cmp::Reverse;
use chapter14::start_sorting_thread;
fn main() {


}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_city() {
        let city = City::new(String::from("Tokyo"), 9_273_000, String::from("Japan"));
        assert_eq!(city.name, "Tokyo");
        assert_eq!(city.population, 9_273_000);
        assert_eq!(city.country, "Japan");
    }

    #[test]
    fn test_sort_by_population() {
        let mut cities = vec![
            City::new(String::from("Tokyo"), 9_273_000, String::from("Japan")),
            City::new(String::from("Delhi"), 16_787_941, String::from("India")),
            City::new(String::from("Shanghai"), 24_153_000, String::from("China")),
            City::new(String::from("Sao Paulo"), 21_650_000, String::from("Brazil")),
            City::new(String::from("Mexico City"), 21_581_000, String::from("Mexico")),
            City::new(String::from("Cairo"), 9_500_000, String::from("Egypt")),
            City::new(String::from("Mumbai"), 20_185_064, String::from("India")),
            City::new(String::from("Beijing"), 21_516_000, String::from("China")),
            City::new(String::from("Dhaka"), 14_543_000, String::from("Bangladesh")),
            City::new(String::from("Osaka"), 19_222_665, String::from("Japan")),
        ];
    
        // 解決方法 1: sort_by を使用
        // cities.sort_by(|a, b| b.population.cmp(&a.population));
    
        // 解決方法 2: sort_by_key + Reverse
        cities.sort_by_key(|city| Reverse(city.population));

       // ソート結果のデバッグ表示
        for city in &cities {
            println!("City: {}, Population: {}", city.name, city.population);
        }


        assert_eq!(cities[0].name, "Shanghai");
        assert_eq!(cities[9].name, "Tokyo");

        let join_handle = std::thread::spawn(move || {
            cities.sort_by(|c1, c2| c2.population.cmp(&c1.population));
            cities
        });

        let sorted = join_handle.join().unwrap();

        //let sorted = start_sorting_thread(cities);
        
        assert_eq!(sorted[0].name, "Shanghai");
        assert_eq!(sorted[9].name, "Tokyo");

    }
}
