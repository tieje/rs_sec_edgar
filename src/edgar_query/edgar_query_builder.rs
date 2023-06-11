#[derive(Debug, PartialEq)]

struct EdgarQuery {
    query: String,
}
#[derive(Debug, PartialEq)]
struct EdgarQueryBuilder {
    cik: String,
    filing_type: String,
    dateb: String,
    owner: String,
    count: String,
    search_text: String,
}


impl EdgarQueryBuilder {}
