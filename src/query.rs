use std::str::FromStr;

use crate::utils::ALLOWED_QUERY_BYTES;

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
                    Ok(QueryItem {
                        field: f.into(),
                        value: None,
                    })
                }
            }
            [f, v] => {
                if !f.as_bytes().iter().all(QueryItem::is_valid_byte)
                    || !v.as_bytes().iter().all(QueryItem::is_valid_byte)
                {
                    Err(QueryItemParseError)
                } else {
                    Ok(QueryItem {
                        field: f.into(),
                        value: Some(v.into()),
                    })
                }
            }
            _ => Err(QueryItemParseError),
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
        if s.is_empty() || s.chars().nth(0).unwrap() != '?' {
            return Err(QueryParseError);
        }
        // remove the '?' and parse QueryItems separated by '&'
        match s[1..]
            .split('&')
            .map(str::parse::<QueryItem>)
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(items) => Ok(Query { items }),
            Err(_) => Err(QueryParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add actual tests. Compare using json files? The Query structs become
    //       quite verbose otherwise.
    #[test]
    fn parsing() {
        let _ = dbg!("?name=John".parse::<Query>());
        let _ = dbg!("?name=John&age=30&city=Stockholm".parse::<Query>());
        let _ = dbg!("?file=report-v1.2~final".parse::<Query>());
        let _ = dbg!("?query=hello+world".parse::<Query>());
        let _ = dbg!("?search=C%2B%2B+programming".parse::<Query>());
        let _ = dbg!("?key=".parse::<Query>());
        let _ = dbg!("?name=John&age=".parse::<Query>());
        let _ = dbg!("?debug".parse::<Query>());
        let _ = dbg!("?debug=true".parse::<Query>());
    }
}
