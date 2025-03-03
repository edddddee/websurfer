use std::str::FromStr;

use crate::utils::ALLOWED_QUERY_BYTES;

// TODO: percent-encoding currently not handled. Maybe should be
//       the responsibility of a validation earlier in the chain though.
//       (for example if only uri exposes some API and query is left private).

#[derive(Debug, Eq, PartialEq)]
struct QueryItem {
    field: String,
    value: String,
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
                        value: "".into(),
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
                        value: v.into(),
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

impl Query {
    fn new() -> Self {
        Self { items: vec![] }
    }

    fn with_item(mut self, field: &str, value: &str) -> Self {
        let value = value.to_string();
        let field = field.to_string();
        self.items.push(QueryItem { field, value });
        Self { items: self.items }
    }
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

    // local util function for quickly creating Query structs
    // doesn't validate input so shouldn't be exposed.
    // Similar constructor functions might be needed as methods in the future
    // but may be implemented differently. This function is purely implemented
    // to be able to write less verbose syntax in the tests below
    fn create_query(field_value_pairs: &[(&str, &str)]) -> Query {
        let items: Vec<_> = field_value_pairs
            .iter()
            .map(|&(f, v)| QueryItem {
                field: f.into(),
                value: v.into(),
            })
            .collect();
        Query { items }
    }

    #[test]
    fn parsing() {
        assert_eq!(
            "?name=John".parse::<Query>(),
            Ok(create_query(&[("name", "John")]))
        );
        assert_eq!(
            "?name=John&age=30&city=Stockholm".parse::<Query>(),
            Ok(create_query(&[
                ("name", "John"),
                ("age", "30"),
                ("city", "Stockholm")
            ]))
        );
        assert_eq!(
            "?file=report-v1.2~final".parse::<Query>(),
            Ok(create_query(&[("file", "report-v1.2~final")]))
        );
        assert_eq!(
            "?query=hello+world".parse::<Query>(),
            Ok(create_query(&[("query", "hello+world")]))
        );
        assert_eq!(
            "?search=C%2B%2B+programming".parse::<Query>(),
            Ok(create_query(&[("search", "C%2B%2B+programming")]))
        );
        assert_eq!("?key=".parse::<Query>(), Ok(create_query(&[("key", "")])));
        assert_eq!(
            "?name=John&age=".parse::<Query>(),
            Ok(create_query(&[("name", "John"), ("age", "")]))
        );
        assert_eq!(
            "?debug".parse::<Query>(),
            Ok(create_query(&[("debug", "")]))
        );
        assert_eq!(
            "?debug=true".parse::<Query>(),
            Ok(create_query(&[("debug", "true")]))
        );
    }
}
