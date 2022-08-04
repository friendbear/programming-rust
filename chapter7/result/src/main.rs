use std::fs;
use std::io;
use std::path::Path;

/*
fn print_error(err: &dyn Error) {
    writeln!(stderr(), "error {}", err);
    while let Some(source) = Error::source(err) {

        writeln!(stderr(), "caused by: {:?}", Error::source(err));
        err = Error::source(err);
    }
}
*/

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }

    //    let src = fs::read_dir(".")?.last()?;
    let src = Path::new(".");
    let dst = Path::new("/tmp");
    move_all(src, dst)?;

    Ok(())
}
fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir().expect("read dir call failed.") {
        if let Ok(entry) = entry_result {
            let dst_file = dst.join(entry.file_name());
            fs::copy(entry.path(), dst_file)?;
        }
    }
    Ok(())
}
