///{ REQUIREMENTS (from tests):
/// Encoding
/// - digits do not transform
/// - drop punctuation
/// - drop whitespace
/// - output is only lower case
/// - output is groups of 5 letters (last group is <=5)
/// Decoding
/// - output has no whitespace
/// Assumptions
/// - ascii strings only
/// }

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const ENCODE_GROUP_LENGTH: usize = 5;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    if !plain.is_ascii() {
        panic!("only ascii strings are encodable")
    }
    plain.make_ascii_lowercase(); // inplace
    let mut encoded = String::with_capacity(plain.len() * (1 + 1 / ENCODE_GROUP_LENGTH)); // this includes punctuation room -- keep?
    for c in plain.chars() { // since only ascii, no surprises (runes unnecessary)
    }
    encoded.shrink_to_fit();
    encoded
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    if !plain.is_ascii() {
        panic!("only ascii strings are decodable")
    }

    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
