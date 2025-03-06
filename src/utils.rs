pub const UNRESERVED_BYTES: [u8; 66] = [
    b'-', b'.', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L',
    b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
    b'Y', b'Z', b'_', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i',
    b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
    b'v', b'w', b'x', b'y', b'z', b'~',
];

pub const RESERVED_BYTES: [u8; 19] = [
    b'!', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'/',
    b':', b';', b'=', b'?', b'@', b'[', b']',
];

pub const ALLOWED_SCHEME_BYTES: [u8; 65] = [
    b'+', b'-', b'.', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
    b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K',
    b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i',
    b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
    b'v', b'w', b'x', b'y', b'z',
];

pub const ALLOWED_HOSTNAME_BYTES: [u8; 64] = [
    b'-', b'.', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L',
    b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
    b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
    b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z',
];

pub const ALLOWED_PATH_BYTES: [u8; 70] = [
    b'%', b'*', b'+', b'-', b'.', b'/', b'0', b'1', b'2', b'3', b'4', b'5',
    b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
    b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T',
    b'U', b'V', b'W', b'X', b'Y', b'Z', b'_', b'a', b'b', b'c', b'd', b'e',
    b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q',
    b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'~',
];

pub const ALLOWED_QUERY_BYTES: [u8; 67] = [
    b'%', b'-', b'.', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
    b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K',
    b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'Z', b'_', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
    b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
    b'u', b'v', b'w', b'x', b'y', b'z', b'~',
];

pub const ASCII_HEX: [u8; 22] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b',
    b'c', b'd', b'e', b'f', b'A', b'B', b'C', b'D', b'E', b'F',
];

pub fn is_percent_encoding(hex: [u8; 2]) -> bool {
    matches!(
        (ASCII_HEX.contains(&hex[0]), ASCII_HEX.contains(&hex[1])),
        (true, true)
    )
}

pub fn is_properly_percent_encoded(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'%')
        .all(|(idx, _)| {
            idx + 2 < bytes.len() - 1
                && is_percent_encoding([bytes[idx+1], bytes[idx+2]])
        })
}

// TODO: Optimize with better algorithm? Currently using naive approach.
//       Might not be worth it anyway. Should be cache-friendly and most inputs
//       are expected to be shorter strings.
pub fn contains_subslice(src: &[u8], subslice: &[u8]) -> bool {
    let n = subslice.len();
    if n > src.len() || n == 0 {
        return false;
    }
    src.windows(n).any(|slice| slice == subslice)
}
