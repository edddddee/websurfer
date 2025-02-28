use crate::utils::ALLOWED_QUERY_BYTES;

use std::str::FromStr;

// TODO: percent-encoding currently not handled. Maybe should be
//       the responsibility of a validation earlier in the chain though.
//       (for example if only uri exposes some API and query is left private).

#[derive(Debug, Eq, PartialEq)]
struct QueryItem {
    field: String,
    value: Option<String>,
}

impl QueryItem {
    fn is_valid_byte(b: &u8) -> bool {
        ALLOWED_QUERY_BYTES.contains(b)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct QueryItemParseError;

impl FromStr for QueryItem {
    type Err = QueryItemParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('=').collect::<Vec<_>>()[..] {
            [f, ""] | [f] => {
                if !f.as_bytes().iter().all(QueryItem::is_valid_byte) {
                    Err(QueryItemParseError)
                } else {
                    Ok(QueryItem {field: f.into(), value: None})
                }
            }
            [f, v] => {
                if !f.as_bytes().iter().all(QueryItem::is_valid_byte) ||
                        !v.as_bytes().iter().all(QueryItem::is_valid_byte) {
                   Err(QueryItemParseError)
                } else {
                    Ok(QueryItem{ field: f.into(), value: Some(v.into()) })
                }
            }
            _ => {
                Err(QueryItemParseError)
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Query {
    items: Vec<QueryItem>,
}

#[derive(Debug, Eq, PartialEq)]
struct QueryParseError;

impl FromStr for Query {
    type Err = QueryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 || s.chars().nth(0).unwrap() != '?' {
            return Err(QueryParseError);
        }
        // remove the '?', now parsing field-value pairs
        match s[1..].split('&').map(|fv| fv.parse::<QueryItem>()).collect::<Result<Vec<_>,_>>() {
            Ok(items) => Ok(Query { items }),
            Err(_) => Err(QueryParseError),
        }
    }
}

// TODO: Add actual tests. Compare using json files? The Query structs become
//       quite verbose otherwise.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        dbg!("?name=John".parse::<Query>());
        dbg!("?name=John&age=30&city=Stockholm".parse::<Query>());
        dbg!("?file=report-v1.2~final".parse::<Query>());
        dbg!("?query=hello+world".parse::<Query>());
        dbg!("?search=C%2B%2B+programming".parse::<Query>());
        dbg!("?key=".parse::<Query>());
        dbg!("?name=John&age=".parse::<Query>());
        dbg!("?debug".parse::<Query>());
        dbg!("?debug=true".parse::<Query>());
    }
}
