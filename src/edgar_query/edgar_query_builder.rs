use super::{
    filing_types::{Filing, FilingType},
    owner::{Owner, OwnerOptions},
};
use reqwest::Url;
use url::ParseError;

#[derive(Debug, PartialEq)]
pub enum FilingInput {
    TypeStr(String),
    TypeFiling(FilingType),
}
#[derive(Debug, PartialEq)]
pub enum OwnerInput {
    TypeStr(String),
    TypeOwner(OwnerOptions),
}
#[derive(Debug, PartialEq)]
pub enum CountInput {
    TypeStr(String),
    TypeU8(u8),
}
#[derive(Debug, PartialEq)]
pub struct EdgarQueryBuilder {
    base: String,
    cik: String,
    filing_type: String,
    dateb: String,
    owner: String,
    count: String,
    search_text: String,
}
/// Build a raw HTTPS query to be used for EDGAR
/// ```rs
///
///
/// ```
impl EdgarQueryBuilder {
    /// Instantiating a query builder with the following defaults:
    /// ```rs
    /// let base = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&".to_string();
    /// let cik = utils::add_leading_zeros_to_cik(short_cik);
    /// let default = "".to_string();
    /// EdgarQueryBuilder {
    /// base,
    /// cik,
    /// filing_type: default.clone(),
    /// dateb: default.clone(),
    /// owner: "include".to_string(),
    /// count: "10".to_string(),
    /// search_text: default,
    /// ```
    /// It is assumed that the CIK is valid.
    pub fn new(short_cik: &str) -> Self {
        let base = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&".to_string();
        let cik = Self::add_leading_zeros_to_cik(short_cik);
        let default = "".to_string();
        Self {
            base,
            cik,
            filing_type: default.clone(),
            dateb: default.clone(),
            owner: "include".to_string(),
            count: "10".to_string(),
            search_text: default,
        }
    }
    /// EDGAR queries require a CIK with ten digits, however, most CIKs have less than ten digits.
    /// Leading zeros must be added to the CIK to reach this ten digit requirement.
    pub fn add_leading_zeros_to_cik(cik: &str) -> String {
        let mut result = cik.to_owned();
        while result.len() < 10 {
            result.insert_str(0, "0");
        }
        result
    }
    /// Builds and returns the raw HTTPS query that can be used to query EDGAR.
    pub fn build(&self) -> Result<Url, ParseError> {
        let query = Url::parse(format!("{base}CIK={cik}&type={filing_type}&dateb={dateb}&owner={owner}&count={count}&search_text={search_text}&output=atom",
            base = self.base,
            cik = self.cik,
            filing_type = self.filing_type,
            dateb = self.dateb,
            owner = self.owner,
            count = self.count,
            search_text = self.search_text
        ).as_str());
        query
    }
    /// If no filing type is set, the default is an empty String, in which case, all types of filings will be queried.
    pub fn set_filing_type(mut self, filing_type: FilingInput) -> Self {
        match filing_type {
            FilingInput::TypeStr(f) => {
                self.filing_type = Filing::validate_filing_type_string(f.as_str());
                self
            }
            FilingInput::TypeFiling(f) => {
                self.filing_type = Filing::to_string(f);
                self
            }
        }
    }
    /// The date must be a string in the form of YYYYMMDD.
    ///
    /// For example, for January 5th, 2023:
    /// ```rs
    /// let example_query = EdgarQueryBuilder::new("78003");
    /// query.set_dateb("20230105")
    /// ```
    /// If no date is set, the default will be an empty string, which is interpreted as the latest date by EDGAR by default.
    pub fn set_dateb(mut self, yyyymmdd: &str) -> Self {
        self.dateb = yyyymmdd.to_string();
        self
    }
    /// There are three options: "include", "exclude", and "only".
    ///
    /// Owner refers to individuals who own significant amounts of the company's stock.
    /// - "include" means include all documents regardless of the source.
    /// - "exclude" means exclude documents related to the company's director or officer ownership.
    /// - "only" means only show documents related to the company's director or officer ownership.
    /// If owner is not set, the default is "include".
    pub fn set_owner(mut self, owner: OwnerInput) -> Self {
        match owner {
            OwnerInput::TypeStr(ow) => {
                self.owner = Owner::validate_owner_string(ow.as_str());
                self
            }
            OwnerInput::TypeOwner(ow) => {
                self.owner = Owner::to_string(ow);
                self
            }
        }
    }
    /// The SEC's EDGAR apparently provides filings from 10 to 100 with the following options:
    ///
    /// `10, 20 , 40, 80, 100`
    ///
    /// Whatever number is used will be rounded down to the greatest valued option, up to 100.
    ///
    /// For example, a string value of "200" will be rounded down to 100.
    ///
    /// 19 gets rounded down to 10.
    ///
    /// If count is not set, default is 10.
    pub fn set_count(mut self, count: CountInput) -> Self {
        match count {
            CountInput::TypeStr(c) => {
                self.count = c;
                self
            }
            CountInput::TypeU8(c) => {
                self.count = c.to_string();
                self
            }
        }
    }
    /// If search text is not set, the default is an empty string.
    pub fn set_search_text(mut self, search_text: &str) -> Self {
        self.search_text = search_text.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edgar_query::filing_types::FilingType::_10K;
    use CountInput::TypeU8;
    use FilingInput::TypeFiling;

    fn sample() -> EdgarQueryBuilder {
        EdgarQueryBuilder::new("78003")
    }
    #[test]
    fn edgar_query_builder_adding_leading_zeros_to_cik() {
        let answer = "0000000123".to_string();
        assert_eq!(EdgarQueryBuilder::add_leading_zeros_to_cik("123"), answer)
    }
    #[test]
    fn edgar_query_builder_new() {
        let answer = "0000078003";
        assert_eq!(sample().cik.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_filing_type() {
        let answer = "10-K";
        let query = sample().set_filing_type(TypeFiling(_10K));
        assert_eq!(query.filing_type.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_dateb() {
        let answer = "20230105";
        let query = sample().set_dateb(&answer);
        assert_eq!(query.dateb.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_owner() {
        let answer = "only";
        let query = sample().set_owner(OwnerInput::TypeStr(answer.to_string()));
        assert_eq!(query.owner.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_count() {
        let answer = 10;
        let query = sample().set_count(CountInput::TypeU8(answer));
        assert_eq!(query.count.as_str(), "10")
    }
    #[test]
    fn edgar_query_builder_build() {
        let answer = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000078003&type=10-k&dateb=&owner=include&count=20&search_text=&output=atom".to_lowercase();
        let query = sample()
            .set_count(TypeU8(20))
            .set_filing_type(TypeFiling(_10K))
            .build()
            .unwrap()
            .as_str()
            .to_lowercase();
        assert_eq!(query, answer)
    }
}
