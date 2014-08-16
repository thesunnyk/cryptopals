
use std::io::File;

pub fn read_file(s : String) -> Vec<String> {
    let p = Path::new(s);
    let mut file = match File::open(&p) {
        Ok(f) => f,
        Err(e) => fail!(e)
    };
    let result = match file.read_to_str() {
        Ok(r) => r,
        Err(e) => fail!(e)
    };

    result.as_slice().split('\n').map(|x| x.to_string()).collect()
}
