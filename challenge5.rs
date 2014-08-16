
mod strutils;
mod xor;

fn main() {
    let cleartext = "Burning 'em, if you ain't quick and nimble\n".to_string()
        + "I go crazy when I hear a cymbal".to_string();
    let cipher = "ICE".to_ascii().iter().map(|x| x.to_byte()).collect();
    let ciphertext_expected_str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202".to_string()
    + "d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b20".to_string()
    + "27630c692b20283165286326302e27282f".to_string();
    let ciphertext_expected = match strutils::to_hex(ciphertext_expected_str.as_slice()) {
        Err(x) => fail!(x),
        Ok(x) => x
    };
    let cleartext_u8 : Vec<u8> = cleartext.as_slice().to_ascii().iter().map(|x| x.to_byte()).collect();
    let ciphertext = xor::xor_repeat(cleartext_u8.as_slice(), cipher);
    assert_eq!(ciphertext, ciphertext_expected);
}
