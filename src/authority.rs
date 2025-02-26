use crate::utils;

use std::str::FromStr;

#[rustfmt::skip]
const ALLOWED_HOSTNAME_BYTES: [u8; 64] = [
    b'-', b'.',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K',
    b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z',
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k',
    b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z',  
];

#[derive(Debug, PartialEq, Eq)]
struct Host {
    inner: String,
}

impl Host {
    fn is_valid_host_byte(b: &u8) -> bool {
        utils::binary_search(*b, &ALLOWED_HOSTNAME_BYTES)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HostParseError;

impl FromStr for Host {
    type Err = HostParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes().iter().all(Host::is_valid_host_byte) {
            Ok(Self { inner: s.to_ascii_lowercase() })
        } else {
            Err(HostParseError)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Port {
    inner: u16,
}

#[derive(Debug, PartialEq, Eq)]
struct PortParseError;

impl FromStr for Port {
    type Err = PortParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(n) => Ok(Port { inner: n }),
            Err(_) => Err(PortParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Authority {
    host: Host,
    port: Option<Port>,
}

impl Authority {
    fn new() -> Self {
        Self {
            host: Host { inner: "".into() },
            port: None,
        }
    }

    fn host(self, hostname: &str) -> Self {
        Self {
            host: Host { inner: hostname.into() },
            port: self.port,
        }
    }

    fn port(self, port: u16) -> Self {
        Self {
            host: self.host,
            port: Some(Port { inner: port }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AuthorityParseError;

impl FromStr for Authority {
    type Err = AuthorityParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(":").collect::<Vec<_>>()[..] {
            [h] => {
                if let Ok(host) = h.parse::<Host>() {
                    Ok(Authority {
                        host,
                        port: None,
                    })
                } else {
                    Err(AuthorityParseError)
                }
            }
            [h, p] => {
                match (h.parse::<Host>(), p.parse::<Port>()) {
                    (Ok(host), Ok(port)) => Ok(Authority {host, port: Some(port)}),
                    (Ok(host), Err(_)) => Ok(Authority {host, port: None}),
                    _ => Err(AuthorityParseError),
                }
            }
            _ => Err(AuthorityParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            "www.example.com".parse(),
            Ok(Authority::new().host("www.example.com"))
        );
        assert_eq!(
            "www.example.com:443".parse(),
            Ok(Authority::new().host("www.example.com").port(443))
        );
        assert_eq!(
            "www.example-2.com:80".parse(),
            Ok(Authority::new().host("www.example-2.com").port(80))
        );
        // Case insensitive
        assert_eq!(
            "WWW.EXAMPLE.COM".parse(),
            Ok(Authority::new().host("www.example.com"))
        );

        // Too many ':'-separators (can only have one port number)
        assert_eq!(
            "www.example.com:80:443".parse::<Authority>(),
            Err(AuthorityParseError)
        );
        // Invalid character
        assert_eq!(
            "*www.example.com".parse::<Authority>(),
            Err(AuthorityParseError)
        );
        // Whitespace disallowed
        assert_eq!(
            " www.example.com".parse::<Authority>(),
            Err(AuthorityParseError)
        );
    }
}
