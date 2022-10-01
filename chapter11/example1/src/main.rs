use std::io::Write;
use std::fmt::Debug;
use std::hash::Hash;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}
use std::fs::File;
use std::str::Bytes;
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

// reference of trait
fn trait_object() -> std::io::Result<()> {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;

    writer.write_all(b"hello world\n")?;
    writer.flush()?;
    println!("{:?}", buf);
    Ok(())
}

// genelics function
fn say_hi<W: Write>(out: &mut W) -> std::io::Result<()>{
    out.write_all(b"hi\n")?;
    out.flush()
}

fn applender<W: Write>(out: &mut W, s: &str) -> std::io::Result<()> {
    out.write_all(s.as_bytes())?;
    out.flush()
}

// where type
fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {
    todo!()
}

fn top_twenty<T: Debug + Hash + Eq>(values: &Vec<T>)
    where T: Debug + Hash + Eq
{
    todo!()
}