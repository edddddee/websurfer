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

#[derive(Debug, PartialEq, Eq)]
struct HostParseError;

impl FromStr for Host {
    type Err = HostParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
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
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Authority {
    host: Host,
    port: Option<Port>,
}

#[derive(Debug, PartialEq, Eq)]
struct AuthorityParseError;

impl FromStr for Authority {
    type Err = AuthorityParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
