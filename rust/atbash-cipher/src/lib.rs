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
    // make String variable with max possible capacity needed (but, possibly more than needed b/c punctuation is dropped)
    let mut encoded = String::with_capacity(plain.len() * (1 + 1 / ENCODE_GROUP_LENGTH));
    // iterate over chars; since only ascii, no surprises (rune conversion not necessary)
    for c in plain.to_ascii_lowercase().chars() {
        // handle if group of 5 exists (push space)
        if encoded.len() != 0 && encoded.len() % 5 == 0 {
            encoded.push(' ');
        }

        if c.is_numeric() {
            encoded.push(c)
        }
        // c.alphabetical()
        let i = ALPHABET.find(c);
        // handle non alphabetical chars (punctuation, etc.)
        if i.is_none() {
            continue;
        }
        // push complimentary character
        encoded.push(
            ALPHABET
                .chars()
                .nth(ALPHABET.len() - i.unwrap() - 1)
                .unwrap(),
        );
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
