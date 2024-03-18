use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_give_file_mot_that_exists_when_read_lines_then_expects_error_result() {
        let file_name = "wrong_path.txt";
        let result = read_lines(&file_name);
        assert!(result.is_err());
    }
}