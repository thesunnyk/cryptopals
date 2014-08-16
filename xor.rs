
extern crate core;
mod base64;

pub mod xor {
    use base64::base64;
    use std::iter::Repeat;
    use std::iter::range_inclusive;
    use std::io::File;

    pub fn xor(x: &[u8], y: &[u8]) -> Vec<u8> {
        x.iter().zip(y.iter()).map(|(i, j)| { *i ^ *j }).collect()
    }

    pub fn xor_byte(x: &[u8], a: u8) -> Vec<u8> {
        let rep : Vec<u8> = Repeat::new(a).take(x.len()).collect();
        xor(x, rep.as_slice())
    }

    pub fn get_score(x: u8) -> uint {
        if !x.is_ascii() {
            0
        } else {
            match x.to_ascii().to_lowercase().to_char() {
                ' ' => 10,
                'a' => 8,
                'b' => 1,
                'c' => 3,
                'd' => 4,
                'e' => 13,
                'f' => 2,
                'g' => 2,
                'h' => 6,
                'i' => 7,
                'j' => 0,
                'k' => 1,
                'l' => 4,
                'm' => 2,
                'n' => 7,
                'o' => 8,
                'p' => 2,
                'q' => 0,
                'r' => 6,
                's' => 6,
                't' => 9,
                'u' => 3,
                'v' => 1,
                'w' => 2,
                'x' => 0,
                'y' => 2,
                'z' => 0,
                _ => 0
            }
        }
    }

    pub fn get_score_for(x: &[u8]) -> uint {
        x.iter().map(|x| get_score(*x)).fold(0, |a, b| a + b)
    }

    pub fn get_xor_score(x: &[u8], a: u8) -> uint {
        get_score_for(xor_byte(x, a).as_slice())
    }

    pub fn find_max_xor(x: &[u8]) -> Option<u8> {
        range_inclusive(0 as u8, 255 as u8).max_by(|&a| get_xor_score(x, a))
    }

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

    pub fn to_string(a: &[u8]) -> String {
        let chars = a.iter().map(|x| x.to_ascii().to_char());
        FromIterator::from_iter(chars)
    }

    #[test]
    fn test_max_score_file() {
        let lines_str = read_file("challenge_4.txt".to_string());
        let lines : Vec<Vec<u8>> = lines_str.iter().map(|x| match base64::to_hex(x.as_slice()) {
            Ok(x) => x,
            Err(x) => fail!(x)
        }).collect();
        let max_xors = lines.iter().map(|x| match find_max_xor(x.as_slice()) {
            Some(x) => x,
            None => fail!("Could not find max xor")
        });
        let mut xored = lines.iter().zip(max_xors).map(|(l, s)| xor_byte(l.as_slice(), s));
        let max = match xored.max_by(|x| get_score_for(x.as_slice())) {
            Some(x) => x,
            None => fail!("Could not get max score")
        };

        let result = to_string(max.as_slice());
        println!("val: {}", result);
        assert!(result == "Now that the party is jumping\n".to_string());
    }

    #[test]
    fn test_max_score() {
        let xstr = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let x = match base64::to_hex(xstr) {
            Ok(x) => x,
            Err(x) => fail!(x)
        };
        let val = match find_max_xor(x.as_slice()) {
            Some(x) => x,
            None => fail!("no max xor found")
        };
        let rep : Vec<u8> = Repeat::new(val).take(x.len()).collect();
        let xored = xor(x.as_slice(), rep.as_slice());
        println!("val: {} -> {}", val, to_string(xored.as_slice()));
        assert!(val == 88);
    }

    #[test]
    fn test_score() {
        assert!(get_score('e'.to_ascii().to_byte()) == 13);
        assert!(get_score('s'.to_ascii().to_byte()) == 6);
        let helloscore = 6 + 13 + 4 + 4 + 8 + 0;
        let helloascii = "helloq".to_string().into_ascii();
        let hellostr : Vec<u8> = helloascii.iter().map(|x| x.to_byte()).collect();
        assert!(get_score_for(hellostr.as_slice()) == helloscore);
    }

    #[test]
    fn test_xor() {
        let xstr = "1c0111001f010100061a024b53535009181c";
        let ystr = "686974207468652062756c6c277320657965";
        let expectedstr = "746865206b696420646f6e277420706c6179";

        let x = match base64::to_hex(xstr) {
            Ok(x) => x,
            Err(x) => fail!(x)
        };
        let y = match base64::to_hex(ystr) {
            Ok(x) => x,
            Err(x) => fail!(x)
        };
        let expected = match base64::to_hex(expectedstr) {
            Ok(x) => x,
            Err(x) => fail!(x)
        };
        assert!(xor(x.as_slice(), y.as_slice()) == expected);
    }
}

