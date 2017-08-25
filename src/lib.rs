use std::ascii::AsciiExt;

pub fn normalise(stream: &[u8]) -> Vec<u8> {
    let mut stripped: Vec<u8> = Vec::new();
    for &ch in stream.iter() {
        // Strip whitespace
        if ch == b' ' {
            continue;
        }
        stripped.push(ch);
    }
    stripped
}

pub fn parse(stream: &[u8]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut z = stream.iter().peekable();
    while z.peek().is_some() {
        let chunk: Vec<&u8> = z.by_ref().take(2).collect();
        // TODO Check size
        let high = convert_hexdigit_to_value(chunk[0]);
        let low = convert_hexdigit_to_value(chunk[1]);
        let value = (high << 4) + low;
        bytes.push(value);
    }
    bytes
}

/// Given as byte containing an ASCII-encoded hexadecimal digit,
/// return the value of that digit.
/// ```
/// convert_hexdigit_to_value(b'C') -> 0xC
/// ```
fn convert_hexdigit_to_value(digit: &u8) -> u8 {
    let mut val = digit.to_ascii_uppercase();
    if val >= 0x30 && val <= 0x39 {
        val -= 0x30;
    } else if val >= 0x41 && val <= 0x46 {
        val -= 0x41;
        val += 0xA;
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_elm() {
        assert_eq!(parse(&vec![b'1', b'6']), vec![0x16]);
        assert_eq!(parse(&vec![b'C', b'4']), vec![0xC4]);
        assert_eq!(parse(&vec![b'C', b'4', b'5', b'F']), vec![0xC4, 0x5F]);
        assert_eq!(parse(&vec![b'a', b'f']), vec![0xaf]);
    }
}
