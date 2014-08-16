
mod base64;
mod strutils;

fn main() {
    let data_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    match strutils::to_hex(data_str) {
        Ok(x) => {
            let data_b64 = base64::to_base64_unpacked(x);
            let data_str = base64::to_base64_string(data_b64);
            assert!(data_str == expected.to_string());
        },
        Err(y) => {
            fail!(y);
        }
    }
}
