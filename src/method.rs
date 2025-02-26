use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidHttpMethod;

impl FromStr for Method {
    type Err = InvalidHttpMethod;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(InvalidHttpMethod),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!("GET".parse(), Ok(Method::GET));
        assert_eq!("HEAD".parse(), Ok(Method::HEAD));
        assert_eq!("POST".parse(), Ok(Method::POST));
        assert_eq!("PUT".parse(), Ok(Method::PUT));
        assert_eq!("DELETE".parse(), Ok(Method::DELETE));
        assert_eq!("CONNECT".parse(), Ok(Method::CONNECT));
        assert_eq!("OPTIONS".parse(), Ok(Method::OPTIONS));
        assert_eq!("TRACE".parse(), Ok(Method::TRACE));
        assert_eq!("PATCH".parse(), Ok(Method::PATCH));

        // Case sensitive
        assert_eq!("get".parse::<Method>(), Err(InvalidHttpMethod));

        // Empty string is invalid
        assert_eq!("".parse::<Method>(), Err(InvalidHttpMethod));
    }
}
