use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use chapter13::as_ref::print_as_ref;

// A function that takes any type that implements AsRef<Path>
fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Custom type that implements AsRef<Path>
struct MyPath {
    path: String,
}

impl AsRef<Path> for MyPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

fn main() -> io::Result<()> {
    // Using a string slice
    let contents = read_file("example.txt")?;
    println!("Contents of example.txt:\n{}", contents);

    // Using a custom type that implements AsRef<Path>
    let my_path = MyPath {
        path: String::from("example.txt"),
    };
    let contents = read_file(my_path)?;
    println!("Contents of example.txt:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_as_ref() {
        let my_path = MyPath {
            path: String::from("example.txt"),
        };
        print_as_ref(&my_path.path);
    }
}