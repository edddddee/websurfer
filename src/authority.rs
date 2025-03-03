use std::fmt;
use std::str::FromStr;

use crate::utils::ALLOWED_HOSTNAME_BYTES;

#[derive(Debug, PartialEq, Eq)]
struct Host {
    inner: String,
}

impl Host {
    fn is_valid_host_byte(b: &u8) -> bool {
        ALLOWED_HOSTNAME_BYTES.contains(b)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HostParseError;

impl FromStr for Host {
    type Err = HostParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check that s contains only valid hostname characters
        if !s.as_bytes().iter().all(Host::is_valid_host_byte) {
            return Err(HostParseError);
        }
        // s needs to start with an alphanumeric character
        if !s.starts_with(|c: char| c.is_ascii_alphanumeric()) {
            return Err(HostParseError);
        }
        // s cannot be purely numerical. E.g. '424242' is not a hostname.
        if s.chars().all(|c: char| c.is_ascii_digit()) {
            return Err(HostParseError);
        }
        // No consecutive dots
        if s.contains("..") {
            return Err(HostParseError);
        }
        // If s contains a '-', it must be preceed and be preceeded by
        // an alphanumeric value.
        //
        // s cannot end with a '-' or a '.'
        let mut chars = s.chars();
        while let Some(this_char) = chars.next() {
            match (&this_char, &chars.next()) {
                ('-', Some('.'))
                | ('.', Some('-'))
                | ('-', None)
                | ('.', None) => {
                    return Err(HostParseError);
                }
                _ => {}
            }
        }
        Ok(Self {
            inner: s.to_ascii_lowercase(),
        })
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
            host: Host {
                inner: hostname.into(),
            },
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
                    Ok(Authority { host, port: None })
                } else {
                    Err(AuthorityParseError)
                }
            }
            [h, p] => match (h.parse::<Host>(), p.parse::<Port>()) {
                (Ok(host), Ok(port)) => Ok(Authority {
                    host,
                    port: Some(port),
                }),
                (Ok(host), Err(_)) => Ok(Authority { host, port: None }),
                _ => Err(AuthorityParseError),
            },
            _ => Err(AuthorityParseError),
        }
    }
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(p) = &self.port {
            write!(f, "{}:{}", self.host.inner, p.inner)
        } else {
            write!(f, "{}", self.host.inner)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add more examples (especially invalid ones)
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

    #[test]
    fn formatting() {
        assert_eq!(
            "www.example.com".parse::<Authority>().unwrap().to_string(),
            "www.example.com"
        );
        assert_eq!(
            "www.example.com:80"
                .parse::<Authority>()
                .unwrap()
                .to_string(),
            "www.example.com:80"
        );
        assert_eq!(
            "www.example-2.com:443"
                .parse::<Authority>()
                .unwrap()
                .to_string(),
            "www.example-2.com:443"
        );
        // Case insensitive
        assert_eq!(
            "WWW.EXAMPLE.COM".parse::<Authority>().unwrap().to_string(),
            "www.example.com".to_ascii_lowercase()
        );
    }
}
