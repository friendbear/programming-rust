
#[cfg(test)]
mod error_test {

    use std::{fs::File, io::{self, stderr, BufRead,BufReader, Write}};

    /// define custom error type
    
    #[test]
    /// 複数エラーの対応
    fn many_error() {
        type GenError = Box<dyn std::error::Error>;
        type GenResult = Result<T, GenError>;
        fn read_numbers(file: &mut BufRead) -> GenResult<Vec<i64>> {
            let mut numbers = vec![];
            for line_result in file.lines() {
                let line = line_result.map_err(|e| GenError::from(e)).unwrap();
                numbers.push((line.parse::<i64>()).map_err(|e| GenError::from(e)));
            }
            Ok(numbers)
        }

        fn compile_project() {
            let mut buf_src = BufReader::new(File::open("src/main.rs").unwrap());
            let value = read_numbers(&mut buf_src);
        }

        assert!(true)
    }

    #[test]
    fn unexcepted_error() {
        "999999999999999999999999999".parse::<u64>().expect_err("Testing expected error");
    }

    #[test]
    /// Result を握り潰す。
    fn ignore_error() {

        let out = stderr();
        let mut locked = out.lock();

        let _ = locked.write_all(format!("error: {}", "some error.").as_bytes());

        // Return Resultを無視する
        // let _ = writeln!(&locked, "error: {}", "some error.");
        
        assert!(true)
    }



}