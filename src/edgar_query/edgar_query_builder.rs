use crate::utils;

#[derive(Debug, PartialEq)]

struct EdgarQuery {
    query: String,
}
#[derive(Debug, PartialEq)]
struct EdgarQueryBuilder {
    base: String,
    cik: String,
    filing_type: String,
    dateb: String,
    owner: String,
    count: String,
    search_text: String,
}

impl EdgarQueryBuilder {
    pub fn new(short_cik: &str) -> EdgarQueryBuilder {
        let base = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany".to_string();
        let cik = utils::add_leading_zeros_to_cik(short_cik);
        let default = "".to_string();
        EdgarQueryBuilder {
            base,
            cik,
            filing_type: default.clone(),
            dateb: default.clone(),
            owner: default.clone(),
            count: default.clone(),
            search_text: default,
        }
    }
}
