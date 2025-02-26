use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct InvalidScheme;

#[derive(Debug, PartialEq, Eq)]
enum CommonSchemes {
    HTTP,
    HTTPS,
}

impl FromStr for CommonSchemes {
    type Err = InvalidScheme;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            r"http" => Ok(Self::HTTP),
            r"https" => Ok(Self::HTTPS),
            _ => Err(InvalidScheme),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Scheme {
    Standard(CommonSchemes),
    Custom(String),
    Empty,
}

impl Scheme {
    fn parse_custom_scheme(s: &str) -> Result<Self, InvalidScheme> {
        todo!("{s}")
    }
}

impl FromStr for Scheme {
    type Err = InvalidScheme;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(scheme) = s.parse::<CommonSchemes>() {
            Ok(Scheme::Standard(scheme))
        } else {
            match s {
                "" => Ok(Scheme::Empty),
                s => Scheme::parse_custom_scheme(s),
            }
        }
    }
}
