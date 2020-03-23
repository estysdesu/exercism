const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const ENCODE_GROUP_LENGTH: usize = 5;

// encode translates `plain` strings according to the atbash cipher
pub fn encode(plain: &str) -> String {
    let mut encoded = decode(plain);

    let space_count = if encoded.len() % ENCODE_GROUP_LENGTH != 0 {
        encoded.len() / ENCODE_GROUP_LENGTH + 1
    } else {
        encoded.len() / ENCODE_GROUP_LENGTH
    };
    for i in 1..space_count {
        let ii = (i * 5) + (i - 1);
        encoded.insert(ii, ' ');
    }
    encoded
}

// decode translates encoded cipher strings (resultants from `encode`) into their original `plain` strings
pub fn decode(cipher: &str) -> String {
    cipher
        .to_ascii_lowercase()
        .chars()
        .filter_map(|c| atbash(c))
        .collect::<String>()
}

// atbash is a helper to get the mirrored letter from the alphabet
fn atbash(c: char) -> Option<char> {
    if c.is_ascii_digit() {
        Some(c)
    } else {
        let i = ALPHABET.find(c)?;
        ALPHABET.chars().nth(ALPHABET.len() - i - 1)
    }
}
