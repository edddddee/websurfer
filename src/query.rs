#[derive(Debug, Eq, PartialEq)]
struct QueryItem {
    field: String,
    value: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Query {
    items: Vec<QueryItem>,
}
