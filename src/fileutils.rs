
use std::fs::File;
use std::io::Read;

pub fn read_file(s : String) -> Vec<String> {
    let mut file = File::open(s).unwrap();
    let mut result = String::new();
    let size = file.read_to_string(&mut result).unwrap();

    result.split('\n').map(|x| x.to_string()).collect()
}
