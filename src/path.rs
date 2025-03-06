use std::str::FromStr;

use crate::utils::{self, ALLOWED_PATH_BYTES};

#[derive(Debug, PartialEq, Eq)]
struct Path {
    pathstr: String,
}

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
        let s = s.as_bytes();

        // Make sure the path is non-empty and contains only valid characters.
        if !s.iter().all(Path::is_valid_path_byte) || s.is_empty() {
            return Err(PathParseError);
        }

        // TODO: Special case. Technically only valid if method is OPTIONS.
        if s.contains(&b'*') {
            if s.len() == 1 {
                unimplemented!("star path not implemented")
            } else {
                return Err(PathParseError);
            }
        }

        // First character MUST be '/' in a path
        if s[0] != b'/' {
            return Err(PathParseError);
        }

        // No double slashes
        if utils::contains_subslice(s, b"//") {
            return Err(PathParseError);
        }

        // Disallow upward traversal in the file hierarchy
        if utils::contains_subslice(s, b"..") {
            return Err(PathParseError);
        }

        // If a '%' occures, validate the percent-encoding.
        if !utils::is_properly_percent_encoded(s) {
            return Err(PathParseError);
        }
        // If a single '.' occurs, it shall not be used to access hidden folders
        // or files.
        if !s
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c == b'.')
            .all(|(idx, _)| {
                idx == s.len() - 1
                    || !(idx > 0
                        && s[idx + 1].is_ascii_alphanumeric()
                        && s[idx - 1] == b'/')
            })
        {
            return Err(PathParseError);
        }
        // If a '~' occures,
        if !s
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c == b'~')
            .all(|(idx, _)| {
                0 < idx
                    && idx < s.len() - 1
                    && s[idx - 1] == b'/'
                    && s[idx + 1].is_ascii_alphanumeric()
            })
        {
            return Err(PathParseError);
        }

        Ok(Path::new(&String::from_utf8_lossy(s)))
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
        assert_eq!(
            r"/path\with\backslash".parse::<Path>(),
            Err(PathParseError)
        );
        assert_eq!("/path!@#^".parse::<Path>(), Err(PathParseError));
        assert_eq!("/path~".parse::<Path>(), Err(PathParseError));
        assert_eq!("/path&other".parse::<Path>(), Err(PathParseError));
        assert_eq!("/.. ".parse::<Path>(), Err(PathParseError));
        assert_eq!("/path/..".parse::<Path>(), Err(PathParseError));
        assert_eq!(" / ".parse::<Path>(), Err(PathParseError));
        assert_eq!("//".parse::<Path>(), Err(PathParseError));
    }
}
