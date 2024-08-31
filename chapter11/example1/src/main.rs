use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}
use std::fs::File;
fn main() {
    let mut local_file = File::create("hello.txt").unwrap();

    local_file.write_all(b"hello world\n").unwrap();
    //    say_hello(&local_file).unwrap();

    let mut bytes = vec![];
    say_hello(&mut bytes).unwrap();
    assert_eq!(bytes, b"hello world\n");

    trait_object().unwrap();

    let mut bytes = vec![];
    say_hi::<Vec<u8>>(&mut bytes).unwrap();
    assert_eq!(bytes, b"hi\n");

    let mut v1 = (0..255).collect::<Vec<u8>>();
    //    let mut v1 = vec![];
    applender::<Vec<u8>>(&mut v1, "hi writer").unwrap();
    println!("writer: {:?}", v1)
}

/// reference of trait
fn trait_object() -> std::io::Result<()> {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;

    writer.write_all(b"hello world\n")?;
    writer.flush()?;
    println!("{:?}", buf);
    Ok(())
}

// genelics function
fn say_hi<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hi\n")?;
    out.flush()
}

fn applender<W: Write>(out: &mut W, s: &str) -> std::io::Result<()> {
    out.write_all(s.as_bytes())?;
    out.flush()
}

/// Prints the top ten items in a list.
pub fn top_ten<T>(list: &Vec<T>)
where
    T: Eq + Hash + Debug,
{
    let mut map = HashMap::new();
    for item in list {
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }
    let mut pairs: Vec<_> = map.into_iter().collect();
    pairs.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    for (item, count) in pairs.into_iter().take(10) {
        println!("{:?}: {}", item, count);
    }
}
fn _top_twenty<T: Debug + Hash + Eq>(_values: &Vec<T>)
where
    T: Debug + Hash + Eq,
{
    todo!()
}

/// default method
/// A writer that ignores whatever data yoou write to it.
pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    // write_allはデフォルト実装がか書かれているのでそれを利用する。
}

/// seadeライブラリ
use serde::{Deserialize, Serialize};
use serde_json::json;
pub fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
    // Create a JSON serializer to write the data to a file.
    let writer = File::create("configuration.txt")?;
    let mut serializer = serde_json::Serializer::new(writer);
    config.serialize(&mut serializer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_ten() {
        let list = vec![
            1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ];
        top_ten(&list);
    }

    #[test]
    fn test_save_configuration() {
        let mut config = HashMap::new();
        config.insert("name".to_string(), "Alice".to_string());
        config.insert("age".to_string(), "20".to_string());
        save_configuration(&config).unwrap();
    }
}
