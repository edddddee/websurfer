use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    Http0_9,
    Http1_0,
    Http1_1,
    Http2,
    Http3,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidHttpVersion;

impl FromStr for Version {
    type Err = InvalidHttpVersion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            r"HTTP/0.9" => Ok(Self::Http0_9),
            r"HTTP/1.0" => Ok(Self::Http1_0),
            r"HTTP/1.1" => Ok(Self::Http1_1),
            r"HTTP/2" => Ok(Self::Http2),
            r"HTTP/3" => Ok(Self::Http3),
            _ => Err(InvalidHttpVersion),

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!("HTTP/0.9".parse(), Ok(Version::Http0_9));
        assert_eq!("HTTP/1.0".parse(), Ok(Version::Http1_0));
        assert_eq!("HTTP/1.1".parse(), Ok(Version::Http1_1));
        assert_eq!("HTTP/2".parse(), Ok(Version::Http2));
        assert_eq!("HTTP/3".parse(), Ok(Version::Http3));

        // Case sensitive!
        assert_eq!("http/0.9".parse::<Version>(), Err(InvalidHttpVersion));

        assert_eq!("".parse::<Version>(), Err(InvalidHttpVersion));
        assert_eq!("blablabla".parse::<Version>(), Err(InvalidHttpVersion));
    }
}
