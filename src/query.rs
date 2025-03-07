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

// Query struct that stores all fields and their corresponding values
// using parallell vectors. 
// These vectors are expected to be small, which is why something like HashMap
// is not used. In fact, it may in many cases be outright slower than
// contiguous arrays for this particular use case.
#[derive(Debug, Eq, PartialEq)]
struct Query {
    fields: Vec<String>,
    values: Vec<Vec<String>>,
}

impl Query {
    fn new() -> Self {
        Self { 
            fields: vec![],
            values: vec![],
        }
    }

    fn find(&self, field: &str) -> Option<usize> {
        self.fields.iter().position(|f| *f == field)
    }


    fn insert(&mut self, field: &str, value: Option<&str>) {
        // If field already exists
        if let Some(index) = self.find(field) {
            if let Some(value) = value {
                self.values[index].push(value.to_string());
            }
        } else {
            self.fields.push(field.to_string());
            if let Some(value) = value {
                self.values.push(vec![value.to_string()]);
            } else {
                self.values.push(vec![]);
            }
        }
    }

    fn get(&self, field: &str) -> Option<&[String]> {
        if let Some(index) = self.find(field) {
            Some(&self.values[index][..])
        } else {
            None
        }
    }

    fn get_mut(&mut self, field: &str) -> Option<&mut [String]> {
        if let Some(index) = self.find(field) {
            Some(&mut self.values[index][..])
        } else {
            None
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
                Ok(QueryItem { field, value }) => query.insert(&field, value.as_deref()),
                Err(e) => return Err(e),
            };
        };
        Ok(query)
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
        let mut query = Query::new();
        field_value_pairs.iter().for_each(|&(f, v)| {
            let field = f;
            let value = v;
            let value = if value == "None" { None } else { Some(value) };
            query.insert(field, value);
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
            Ok(Query::new())
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
        assert_eq!(
            "q=hello world!".parse::<Query>(),
            Err(QueryParseError::InvalidCharacter)
        );
        // Missing key before equals sign
        assert_eq!("=value".parse::<Query>(), Err(QueryParseError::EmptyField));
        // Malformed percent encoding (%2G is not valid)
        assert_eq!(
            "q=hello%2Gworld".parse::<Query>(),
            Err(QueryParseError::BadPercentEncoding)
        );
        // Using reserved characters in query parameter names
        assert_eq!(
            "hello@=world".parse::<Query>(),
            Err(QueryParseError::InvalidCharacter)
        );
        assert_eq!(
            "hello=this=world".parse::<Query>(),
            Err(QueryParseError::BadFieldValue)
        );
    }
}
