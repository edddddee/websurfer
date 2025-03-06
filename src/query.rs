use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::str::FromStr;

use crate::utils::{self, ALLOWED_QUERY_BYTES};

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
    EmptyField,
    BadPercentEncoding,
}

impl FromStr for QueryItem {
    type Err = QueryParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('=').collect::<Vec<_>>()[..] {
            [""] => Err(QueryParseError::EmptyInput),
            [f] => {
                if !QueryItem::is_valid_field(f) {
                    Err(QueryParseError::InvalidCharacter)
                } else {
                    Ok(QueryItem {
                        field: f.into(),
                        value: None,
                    })
                }
            }
            ["", _] => Err(QueryParseError::EmptyField),
            [f, ""] => {
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
    map: HashMap<String, Vec<String>>,
}

impl Query {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl FromStr for Query {
    type Err = QueryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut query = Query::new();
        if s.is_empty() {
            return Ok(query);
        }
        if !utils::is_properly_percent_encoded(s.as_bytes()) {
            return Err(QueryParseError::BadPercentEncoding);
        }
        for query_item in s.split('&').map(str::parse::<QueryItem>) {
            match query_item {
                Err(e) => return Err(e),
                Ok(QueryItem { field, value }) => {
                    match query.map.entry(field) {
                        Occupied(mut entry) => {
                            if let Some(value) = value {
                                let vec = entry.get_mut();
                                vec.push(value);
                            };
                        }
                        Vacant(entry) => {
                            entry.insert(if let Some(value) = value {
                                vec![value]
                            } else {
                                vec![]
                            });
                        }
                    };
                }
            }
        }
        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    // This function is implemented purely for writing less syntax in the
    // tests below.
    // Doesn't validate input so shouldn't be exposed and used anywhere else
    // internally.
    fn create_query(field_value_pairs: &[(&str, &str)]) -> Query {
        let mut query = Query::new();
        field_value_pairs.iter().for_each(|&(f, v)| {
            let field = f.to_string();
            let value = v.to_string();
            match query.map.entry(field) {
                Occupied(mut entry) => {
                    if value != "None" {
                        entry.get_mut().push(value);
                    }
                }
                Vacant(entry) => {
                    entry.insert(if value != "None" {
                        vec![value]
                    } else {
                        vec![]
                    });
                }
            }
        });
        query
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
        assert_eq!("key=".parse::<Query>(), Ok(create_query(&[("key", "")])));
        assert_eq!(
            "name=John&age=".parse::<Query>(),
            Ok(create_query(&[("name", "John"), ("age", "")]))
        );
        assert_eq!(
            "name=John&age".parse::<Query>(),
            Ok(create_query(&[("name", "John"), ("age", "None")]))
        );
        // Empty query (valid)
        assert_eq!(
            "".parse::<Query>(),
            Ok(Query {
                map: HashMap::new()
            })
        );

        assert_eq!(
            "q=apples&oranges&category=fruit".parse::<Query>(),
            Ok(create_query(&[
                ("q", "apples"),
                ("oranges", "None"),
                ("category", "fruit")
            ]))
        );
        // Duplicate query keys without proper handling
        assert_eq!(
            "q=apple&q=banana".parse::<Query>(),
            Ok(create_query(&[("q", "apple"), ("q", "banana")]))
        );

        // Unencoded special characters (space, !)
        assert_matches!("q=hello world!".parse::<Query>(), Err(_));
        // Missing key before equals sign
        assert_matches!("=value".parse::<Query>(), Err(_));
        // Malformed percent encoding (%2G is not valid)
        assert_matches!("q=hello%2Gworld".parse::<Query>(), Err(_));
        // Using reserved characters in query parameter names
        assert_matches!("hello@=world".parse::<Query>(), Err(_));
    }
}
