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
    assert_eq!(bytes, b"hello world\n")
}