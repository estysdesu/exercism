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
    // panic if invalid input
    if !plain.is_ascii() {
        panic!("only ascii strings are encodable")
    }
    // make String variable with max possible capacity needed (but, possibly more than needed b/c special characters and whitespaces)
    let mut encoded = String::with_capacity(plain.len() * (1 + 1 / ENCODE_GROUP_LENGTH));
    // iterate over chars; since only ascii, no surprises (rune conversion not necessary)
    let mut char_len: u128 = 0;
    for c in plain.to_ascii_lowercase().chars() {
        // handle all ascii not desired
        if !c.is_ascii_digit() && !c.is_ascii_alphabetic() {
            continue;
        }
        // handle if group of 5 exists (push space)
        if char_len != 0 && char_len % 5 == 0 {
            encoded.push(' ');
        }
        // handle numbers
        if c.is_ascii_digit() {
            encoded.push(c);
            char_len += 1;
        }
        // handle aphabetic
        if c.is_ascii_alphabetic() {
            let i = ALPHABET.find(c).unwrap();
            encoded.push(ALPHABET.chars().nth(ALPHABET.len() - i - 1).unwrap());
            char_len += 1;
        }
    }
    encoded.shrink_to_fit();
    encoded
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    if !cipher.is_ascii() {
        panic!("only ascii strings are decodable")
    }

    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
