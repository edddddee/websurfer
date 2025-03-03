use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidHttpMethod;

impl FromStr for Method {
    type Err = InvalidHttpMethod;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            _ => Err(InvalidHttpMethod),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!("GET".parse(), Ok(Method::Get));
        assert_eq!("HEAD".parse(), Ok(Method::Head));
        assert_eq!("POST".parse(), Ok(Method::Post));
        assert_eq!("PUT".parse(), Ok(Method::Put));
        assert_eq!("DELETE".parse(), Ok(Method::Delete));
        assert_eq!("CONNECT".parse(), Ok(Method::Connect));
        assert_eq!("OPTIONS".parse(), Ok(Method::Options));
        assert_eq!("TRACE".parse(), Ok(Method::Trace));
        assert_eq!("PATCH".parse(), Ok(Method::Patch));

        // Case sensitive
        assert_eq!("get".parse::<Method>(), Err(InvalidHttpMethod));

        // Empty string is invalid
        assert_eq!("".parse::<Method>(), Err(InvalidHttpMethod));
    }
}
