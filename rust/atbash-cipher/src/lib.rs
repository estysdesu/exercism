const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const ENCODE_GROUP_LENGTH: usize = 5;

// encode translates `plain` strings according to the atbash cipher
pub fn encode(plain: &str) -> String {
    let atbashed = decode(plain);

    let mut encoded = String::with_capacity(atbashed.len() + atbashed.len() / ENCODE_GROUP_LENGTH);
    for (i, c) in atbashed.char_indices() {
        if i % 5 == 0 && i != 0 {
            encoded.push(' ');
            encoded.push(c);
        } else {
            encoded.push(c);
        }
    }

    encoded
}

// decode translates encoded cipher strings (resultants from `encode`) into their original `plain` strings
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(atbash).collect::<String>()
}

// atbash is a helper to get the mirrored letter from the alphabet
fn atbash(c: char) -> Option<char> {
    if c.is_ascii_digit() {
        Some(c)
    } else {
        let i = ALPHABET.find(c.to_ascii_lowercase())?;
        ALPHABET.chars().nth(ALPHABET.len() - i - 1)
    }
}
