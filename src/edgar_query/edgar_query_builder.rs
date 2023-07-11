//! This module provides a way to build the URL query that will be used to query EDGAR.

use super::{
    filing::{self, validate_filing_type_string, FilingTypeOption},
    owner::{self, validate_owner_string, OwnerOptions},
};
use reqwest::Url;
use url::ParseError;

#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum BuilderInput<'a, T> {
    TypeStr(&'a str),
    TypeTInput(T),
}
/// Build a URL HTTPS query that will be used to query EDGAR
/// ```
/// use sec_edgar::edgar_query::edgar_query_builder::EdgarQueryBuilder;
/// use sec_edgar::edgar_query::edgar_query_builder::FilingInput;
/// use reqwest::Url;
/// let query_url: Url = EdgarQueryBuilder::new("78003")
///     .set_filing_type(FilingInput::TypeStr("10-K".to_string()))
///     .unwrap()
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, PartialEq)]
pub struct EdgarQueryBuilder<'a> {
    #[allow(missing_docs)]
    pub base: &'a str,
    #[allow(missing_docs)]
    pub cik: &'a str,
    #[allow(missing_docs)]
    pub filing_type: &'a str,
    #[allow(missing_docs)]
    pub dateb: &'a str,
    #[allow(missing_docs)]
    pub owner: &'a str,
    #[allow(missing_docs)]
    pub count: &'a str,
    #[allow(missing_docs)]
    pub search_text: &'a str,
}
impl<'a> EdgarQueryBuilder<'a> {
    /// Instantiating a query builder with the following defaults:
    /// ```
    /// use sec_edgar::edgar_query::edgar_query_builder::{add_leading_zeros_to_cik, EdgarQueryBuilder};
    ///
    /// let base = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&".to_string();
    /// let short_cik = "78003";
    /// let cik = add_leading_zeros_to_cik(short_cik);
    /// let default = "".to_string();
    /// let default_build = EdgarQueryBuilder {
    ///     base,
    ///     cik,
    ///     filing_type: default.clone(),
    ///     dateb: default.clone(),
    ///     owner: "include".to_string(),
    ///     count: "10".to_string(),
    ///     search_text: default,
    /// };
    /// ```
    /// It is assumed that the CIK is valid.
    pub fn new(short_cik: &'a str) -> Self {
        let base = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&";
        let cik = add_leading_zeros_to_cik(short_cik);
        Self {
            base,
            cik,
            filing_type: Default::default(),
            dateb: Default::default(),
            owner: "include",
            count: "10",
            search_text: Default::default(),
        }
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
    /// If no filing type is set, the default is an empty &str, in which case, all types of filings will be queried.
    pub fn set_filing_type(&mut self, filing_type: BuilderInput<'a, FilingTypeOption>) {
        self.filing_type = match filing_type {
            BuilderInput::TypeStr(f) => validate_filing_type_string(f)
                .map_err(|e| e)
                .unwrap_or_default(),
            BuilderInput::TypeTInput(f) => filing::to_str(f),
        };
    }
    /// The date must be a string in the form of YYYYMMDD.
    ///
    /// For example, for January 5th, 2023:
    /// ```rs
    /// let example_query = EdgarQueryBuilder::new("78003");
    /// query.set_dateb("20230105")
    /// ```
    /// If no date is set, the default will be an empty &str, which is interpreted as the latest date by EDGAR by default.
    pub fn set_dateb(&mut self, yyyymmdd: &'a str) {
        self.dateb = yyyymmdd;
    }
    /// There are three options: "include", "exclude", and "only".
    ///
    /// Owner refers to individuals who own significant amounts of the company's stock.
    /// - "include" means include all documents regardless of the source.
    /// - "exclude" means exclude documents related to the company's director or officer ownership.
    /// - "only" means only show documents related to the company's director or officer ownership.
    /// If owner is not set, the default is "include".
    pub fn set_owner(&mut self, owner: BuilderInput<'a, OwnerOptions>) {
        self.owner = match owner {
            BuilderInput::TypeStr(ow) => validate_owner_string(ow)
                .map_err(|e| e)
                .unwrap_or("include"),
            BuilderInput::TypeTInput(ow) => owner::to_str(ow),
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
    pub fn set_count(&mut self, count: &'a str) {
        self.count = count
    }
    /// If search text is not set, the default is an empty string.
    pub fn set_search_text(&mut self, search_text: &'a str) {
        self.search_text = search_text;
    }
}

/// EDGAR queries require a CIK with ten digits, however, most CIKs have less than ten digits.
/// Leading zeros must be added to the CIK to reach this ten digit requirement.
pub fn add_leading_zeros_to_cik<'a>(cik: &'a str) -> &'a str {
    let mut result = cik.to_owned();
    while result.len() < 10 {
        result.insert(0, '0');
    }
    &result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edgar_query::filing::FilingTypeOption::_10K;

    fn sample<'a>() -> EdgarQueryBuilder<'a> {
        EdgarQueryBuilder::new("78003")
    }
    #[test]
    fn edgar_query_builder_adding_leading_zeros_to_cik() {
        let answer = "0000000123".to_string();
        assert_eq!(add_leading_zeros_to_cik("123"), answer)
    }
    #[test]
    fn edgar_query_builder_new() {
        let answer = "0000078003";
        assert_eq!(sample().cik.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_filing_type() {
        let answer = "10-K";
        let query = sample().set_filing_type(_10K);
        assert_eq!(query.unwrap().filing_type.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_dateb() {
        let answer = "20230105";
        let query = sample().set_dateb(answer);
        assert_eq!(query.dateb.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_owner() {
        let answer = "only";
        let query = sample().set_owner(BuilderInput::TypeStr(answer)).unwrap();
        assert_eq!(query.owner.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_count() {
        let answer = "10";
        let query = sample().set_count(BuilderInput::TypeStr(answer));
        assert_eq!(query.count.as_str(), "10")
    }
    #[test]
    fn edgar_query_builder_build() {
        let answer = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000078003&type=10-k&dateb=&owner=include&count=20&search_text=&output=atom".to_lowercase();
        let query = sample()
            .set_count("20")
            .set_filing_type(_10K)
            .unwrap()
            .build()
            .unwrap()
            .as_str()
            .to_lowercase();
        assert_eq!(query, answer)
    }
}
