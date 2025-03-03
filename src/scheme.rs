use std::str::FromStr;

use crate::utils::ALLOWED_SCHEME_BYTES;

#[derive(Debug, PartialEq, Eq)]
pub struct SchemeParseError;

#[derive(Debug, PartialEq, Eq)]
enum CommonSchemes {
    Http,
    Https,
}

impl FromStr for CommonSchemes {
    type Err = SchemeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            r"http" => Ok(Self::Http),
            r"https" => Ok(Self::Https),
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
        inner: Repr::Standard(CommonSchemes::Http),
    };
    pub const HTTPS: Scheme = Scheme {
        inner: Repr::Standard(CommonSchemes::Https),
    };
    pub const EMPTY: Scheme = Scheme { inner: Repr::Empty };

    fn parse_custom_scheme(s: &str) -> Result<Self, SchemeParseError> {
        if !s.as_bytes().iter().all(Self::is_valid_scheme_byte) {
            Err(SchemeParseError)
        } else {
            match s {
                "" => Ok(Scheme { inner: Repr::Empty }),
                s => {
                    // First letter must be alphabetic
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
        ALLOWED_SCHEME_BYTES.contains(b)
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
        assert_eq!("Http".parse(), Ok(Scheme::HTTP));
        assert_eq!("https".parse(), Ok(Scheme::HTTPS));
        assert_eq!("Https".parse(), Ok(Scheme::HTTPS));
        assert_eq!("".parse(), Ok(Scheme::EMPTY));
        assert_eq!(
            "blablabla".parse(),
            Ok(Scheme {
                inner: Repr::Custom("blablabla".into())
            })
        );
        // case insensitive
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
