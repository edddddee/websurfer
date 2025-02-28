use crate::utils::{ALLOWED_PATH_BYTES, ASCII_HEX};

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Path {
    pathstr: String,
}

// TODO: Block "../" (directory traversal) and ".filename" (hidden folder/file)
//       but consider it a "valid path" and catch it later for more better
//       error messages.
impl Path {
    fn new(s: &str) -> Self {
        Self { pathstr: s.into() }
    }

    fn is_valid_path_byte(b: &u8) -> bool {
        ALLOWED_PATH_BYTES.contains(b)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PathParseError;

impl FromStr for Path {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.as_bytes().iter().all(Path::is_valid_path_byte) || s.len() == 0 {
            return Err(PathParseError);
        }
        if s.contains('*') {
            if s.len() == 1 {
                return Err(PathParseError);
            } else {
                return Err(PathParseError);
            }
        }
        if s.chars().nth(0).unwrap() != '/' {
            return Err(PathParseError);
        }
        if s.contains("//") {
            return Err(PathParseError);
        }
        if s.contains("..") {
            return Err(PathParseError);
        }
        if !s.match_indices('%').all(|(idx, _)| {
            idx + 2 < s.len() - 1
                && ASCII_HEX.contains(&s.chars().nth(idx + 1).unwrap())
                && ASCII_HEX.contains(&s.chars().nth(idx + 2).unwrap())
        }) {
            return Err(PathParseError);
        }
        // If a dot occurs,
        if !s.match_indices('.').all(|(idx, _)| {
            idx == s.len() - 1
                || !(idx > 0
                    && s.chars().nth(idx + 1).unwrap().is_ascii_alphanumeric()
                    && s.chars().nth(idx - 1).unwrap() == '/')
        }) {
            return Err(PathParseError);
        }
        if !s.match_indices('~').all(|(idx, _)| {
            0 < idx
                && idx < s.len() - 1
                && s.chars().nth(idx - 1).unwrap() == '/'
                && s.chars().nth(idx + 1).unwrap().is_ascii_alphanumeric()
        }) {
            return Err(PathParseError);
        }
        return Ok(Path { pathstr: s.into() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok_paths() {
        assert_eq!("/".parse(), Ok(Path::new("/".into())));
        assert_eq!("/home".parse(), Ok(Path::new("/home".into())));
        assert_eq!(
            "/products/123".parse(),
            Ok(Path::new("/products/123".into()))
        );
        assert_eq!(
            "/api/v1/users".parse(),
            Ok(Path::new("/api/v1/users".into()))
        );
        assert_eq!(
            "/images/profile.jpg".parse(),
            Ok(Path::new("/images/profile.jpg".into()))
        );
        assert_eq!(
            "/articles/tech/how-to".parse(),
            Ok(Path::new("/articles/tech/how-to".into()))
        );
        assert_eq!(
            "/docs/user-guide/installation".parse(),
            Ok(Path::new("/docs/user-guide/installation".into()))
        );
    }

    #[test]
    fn parse_invalid_paths() {
        assert_eq!(
            "/path//with//double-slashes".parse::<Path>(),
            Err(PathParseError)
        );
        assert_eq!("/path with space".parse::<Path>(), Err(PathParseError));
        assert_eq!("/somepath//extra".parse::<Path>(), Err(PathParseError));
        assert_eq!("/.../dots".parse::<Path>(), Err(PathParseError));
        assert_eq!(
            "/path?query=1&filter=abc".parse::<Path>(),
            Err(PathParseError)
        );
        assert_eq!("/path%invalid".parse::<Path>(), Err(PathParseError));
        assert_eq!("/%/invalid-character".parse::<Path>(), Err(PathParseError));
        assert_eq!("/##".parse::<Path>(), Err(PathParseError));
        assert_eq!(r"/path\with\backslash".parse::<Path>(), Err(PathParseError));
        assert_eq!("/path!@#^".parse::<Path>(), Err(PathParseError));
        assert_eq!("/path~".parse::<Path>(), Err(PathParseError));
        assert_eq!("/path&other".parse::<Path>(), Err(PathParseError));
        assert_eq!("/.. ".parse::<Path>(), Err(PathParseError));
        assert_eq!("/path/..".parse::<Path>(), Err(PathParseError));
        assert_eq!(" / ".parse::<Path>(), Err(PathParseError));
        assert_eq!("//".parse::<Path>(), Err(PathParseError));
    }
}
