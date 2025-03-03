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

    #[test]
    fn parsing() {
        assert_eq!(
            "?name=John".parse::<Query>(),
            Ok(Query::new().with_item("name", "John"))
        );
        assert_eq!(
            "?name=John&age=30&city=Stockholm".parse::<Query>(),
            Ok(Query::new()
                .with_item("name", "John")
                .with_item("age", "30")
                .with_item("city", "Stockholm"))
        );
        assert_eq!(
            "?file=report-v1.2~final".parse::<Query>(),
            Ok(Query::new().with_item("file", "report-v1.2~final"))
        );
        assert_eq!(
            "?query=hello+world".parse::<Query>(),
            Ok(Query::new().with_item("query", "hello+world"))
        );
        assert_eq!(
            "?search=C%2B%2B+programming".parse::<Query>(),
            Ok(Query::new().with_item("search", "C%2B%2B+programming"))
        );
        assert_eq!(
            "?key=".parse::<Query>(),
            Ok(Query::new().with_item("key", ""))
        );
        assert_eq!(
            "?name=John&age=".parse::<Query>(),
            Ok(Query::new().with_item("name", "John").with_item("age", ""))
        );
        assert_eq!(
            "?debug".parse::<Query>(),
            Ok(Query::new().with_item("debug", ""))
        );
        assert_eq!(
            "?debug=true".parse::<Query>(),
            Ok(Query::new().with_item("debug", "true"))
        );
    }
}
