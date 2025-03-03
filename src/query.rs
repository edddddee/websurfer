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
    fn is_valid_field(field: &str) -> bool {
        field.as_bytes().iter().all(Self::is_valid_byte)
    }
    fn is_valid_value(value: &str) -> bool {
        value.as_bytes().iter().all(Self::is_valid_byte)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum QueryParseError {
    InvalidCharacter,
    BadFieldValue,
    EmptyInput,
}

impl FromStr for QueryItem {
    type Err = QueryParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('=').collect::<Vec<_>>()[..] {
            [""] => Err(QueryParseError::EmptyInput),
            [f] | [f, ""] => {
                if !QueryItem::is_valid_field(f) {
                    Err(QueryParseError::InvalidCharacter)
                } else {
                    Ok(QueryItem {
                        field: f.into(),
                        value: Some("".into()),
                    })
                }
            }
            [f, v] => {
                if !QueryItem::is_valid_field(f)
                    || !QueryItem::is_valid_value(v)
                {
                    Err(QueryParseError::InvalidCharacter)
                } else {
                    Ok(QueryItem {
                        field: f.into(),
                        value: Some(v.into()),
                    })
                }
            }
            _ => Err(QueryParseError::BadFieldValue),
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
}

impl FromStr for Query {
    type Err = QueryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split('&')
            .map(str::parse::<QueryItem>)
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(items) => Ok(Query { items }),
            Err(QueryParseError::EmptyInput) => Ok(Query { items: vec![] }),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This function is implemented purely for writing less syntax in the
    // tests below.
    // Doesn't validate input so shouldn't be exposed and used anywhere else
    // internally.
    fn create_query(field_value_pairs: &[(&str, &str)]) -> Query {
        let items: Vec<_> = field_value_pairs
            .iter()
            .map(|&(f, v)| QueryItem {
                field: f.into(),
                value: if v == "" { None } else { Some(v.into()) },
            })
            .collect();
        Query { items }
    }

    #[test]
    fn parsing() {
        assert_eq!(
            "name=John".parse::<Query>(),
            Ok(create_query(&[("name", "John")]))
        );
        assert_eq!(
            "name=John&age=30&city=Stockholm".parse::<Query>(),
            Ok(create_query(&[
                ("name", "John"),
                ("age", "30"),
                ("city", "Stockholm")
            ]))
        );
        assert_eq!(
            "file=report-v1.2~final".parse::<Query>(),
            Ok(create_query(&[("file", "report-v1.2~final")]))
        );
        assert_eq!(
            "query=hello%20world".parse::<Query>(),
            Ok(create_query(&[("query", "hello%20world")]))
        );
        assert_eq!(
            "search=C%2B%2B%20programming".parse::<Query>(),
            Ok(create_query(&[("search", "C%2B%2B%20programming")]))
        );
        assert_eq!(
            "key=".parse::<Query>(),
            Ok(Query {
                items: vec![QueryItem {
                    field: "key".into(),
                    value: Some("".into()),
                }]
            })
        );
        assert_eq!(
            "name=John&age=".parse::<Query>(),
            Ok(Query {
                items: vec![
                    QueryItem {
                        field: "name".into(),
                        value: Some("John".into()),
                    },
                    QueryItem {
                        field: "age".into(),
                        value: Some("".into()),
                    }
                ]
            })
        );
        assert_eq!(
            "debug=true".parse::<Query>(),
            Ok(create_query(&[("debug", "true")]))
        );
        // Empty query (valid)
        assert_eq!("".parse::<Query>(), Ok(Query { items: vec![] }))
    }
}
