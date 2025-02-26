use crate::utils;

use std::str::FromStr;

#[rustfmt::skip]
const ALLOWED_SCHEME_BYTES: [u8; 65] = [
    b'+', b'-', b'.',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K',
    b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z',
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k',
    b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z',  
];

#[derive(Debug, PartialEq, Eq)]
pub struct SchemeParseError;

#[derive(Debug, PartialEq, Eq)]
enum CommonSchemes {
    HTTP,
    HTTPS,
}

impl FromStr for CommonSchemes {
    type Err = SchemeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            r"http" => Ok(Self::HTTP),
            r"https" => Ok(Self::HTTPS),
            _ => Err(SchemeParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Scheme {
    inner: Repr,
}

#[derive(Debug, PartialEq, Eq)]
enum Repr {
    Standard(CommonSchemes),
    Custom(String),
    Empty,
}

impl Scheme {
    pub const HTTP: Scheme = Scheme {
        inner: Repr::Standard(CommonSchemes::HTTP),
    };
    pub const HTTPS: Scheme = Scheme {
        inner: Repr::Standard(CommonSchemes::HTTPS),
    };
    pub const EMPTY: Scheme = Scheme { inner: Repr::Empty };

    fn parse_custom_scheme(s: &str) -> Result<Self, SchemeParseError> {
        if !s.as_bytes().iter().all(Self::is_valid_scheme_byte) {
            Err(SchemeParseError)
        } else {
            match s {
                "" => Ok(Scheme { inner: Repr::Empty }),
                s => {
                    if s.chars().take(1).all(char::is_alphabetic) {
                        Ok(Scheme {
                            inner: Repr::Custom(String::from(s)),
                        })
                    } else {
                        Err(SchemeParseError)
                    }
                }
            }
        }
    }

    fn is_valid_scheme_byte(b: &u8) -> bool {
        utils::binary_search(*b, &ALLOWED_SCHEME_BYTES)
    }
}

impl FromStr for Scheme {
    type Err = SchemeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // scheme is case insensitive
        let s = &s.to_ascii_lowercase()[..];
        if let Ok(scheme) = s.parse::<CommonSchemes>() {
            Ok(Scheme {
                inner: Repr::Standard(scheme),
            })
        } else {
            match s {
                "" => Ok(Scheme { inner: Repr::Empty }),
                s => Scheme::parse_custom_scheme(s),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!("http".parse(), Ok(Scheme::HTTP));
        assert_eq!("HTTP".parse(), Ok(Scheme::HTTP));
        assert_eq!("https".parse(), Ok(Scheme::HTTPS));
        assert_eq!("HTTPS".parse(), Ok(Scheme::HTTPS));
        assert_eq!("".parse(), Ok(Scheme::EMPTY));
        assert_eq!(
            "blablabla".parse(),
            Ok(Scheme {
                inner: Repr::Custom("blablabla".into())
            })
        );
        assert_eq!(
            "BLABLABLA".parse(),
            Ok(Scheme {
                inner: Repr::Custom("blablabla".into())
            })
        );

        // whitespace not allowed
        assert_eq!(" http".parse::<Scheme>(), Err(SchemeParseError));
        // invalid character
        assert_eq!("http@".parse::<Scheme>(), Err(SchemeParseError));
        // does not start with an alphabetic character
        assert_eq!("1http".parse::<Scheme>(), Err(SchemeParseError));
    }
}
