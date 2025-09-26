use super::Query;

pub struct QueryParseError {
    pos: usize,
    string: String
}

impl TryFrom<String> for Query {
    type Error = QueryParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        
    }
}