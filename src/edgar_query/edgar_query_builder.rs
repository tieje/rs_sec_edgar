//! This module provides a way to build the URL query that will be used to query EDGAR.

use crate::error::EDGARError;

use super::{
    filing::{self, validate_filing_type_string, FilingTypeOption},
    owner::{self, validate_owner_string, OwnerOptions},
};
use reqwest::Url;

#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum BuilderInput<'a, T> {
    TypeStr(&'a str),
    TypeTInput(T),
}
/// Build a URL HTTPS query that will be used to query EDGAR
/// ```
/// use sec_edgar::edgar_query::edgar_query_builder::{EdgarQueryBuilder, BuilderInput};
/// use reqwest::Url;
/// let query_url: Url = EdgarQueryBuilder::new("78003")
///     .set_filing_type(BuilderInput::TypeStr("10-K"))
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, PartialEq)]
pub struct EdgarQueryBuilder {
    #[allow(missing_docs)]
    pub base: String,
    #[allow(missing_docs)]
    pub cik: String,
    #[allow(missing_docs)]
    pub filing_type: String,
    #[allow(missing_docs)]
    pub dateb: String,
    #[allow(missing_docs)]
    pub owner: String,
    #[allow(missing_docs)]
    pub count: String,
    #[allow(missing_docs)]
    pub search_text: String,
}
impl EdgarQueryBuilder {
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
    pub fn new(short_cik: &str) -> Self {
        let base = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&".to_string();
        let cik = add_leading_zeros_to_cik(short_cik);
        Self {
            base,
            cik,
            filing_type: Default::default(),
            dateb: Default::default(),
            owner: "include".to_string(),
            count: "10".to_string(),
            search_text: Default::default(),
        }
    }
    /// Builds and returns the raw HTTPS query that can be used to query EDGAR.
    pub fn build(&self) -> Result<Url, EDGARError> {
        let url_res = format!("{base}CIK={cik}&type={filing_type}&dateb={dateb}&owner={owner}&count={count}&search_text={search_text}&output=atom",
            base = self.base,
            cik = self.cik,
            filing_type = self.filing_type,
            dateb = self.dateb,
            owner = self.owner,
            count = self.count,
            search_text = self.search_text
        );
        let query = Url::parse(&url_res)?;
        Ok(query)
    }
    /// If no filing type is set, the default is an empty String, in which case, all types of filings will be queried.
    pub fn set_filing_type(mut self, filing_type: BuilderInput<FilingTypeOption>) -> Self {
        self.filing_type = match filing_type {
            BuilderInput::TypeStr(f) => validate_filing_type_string(f).unwrap_or_default(),
            BuilderInput::TypeTInput(f) => filing::to_string(f),
        };
        self
    }
    /// The date must be a string in the form of YYYYMMDD.
    ///
    /// For example, for January 5th, 2023:
    /// ```rs
    /// let example_query = EdgarQueryBuilder::new("78003");
    /// query.set_dateb("20230105")
    /// ```
    /// If no date is set, the default will be an empty String, which is interpreted as the latest date by EDGAR by default.
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
    pub fn set_owner(mut self, owner: BuilderInput<OwnerOptions>) -> Self {
        self.owner = match owner {
            BuilderInput::TypeStr(ow) => validate_owner_string(ow).unwrap_or("include".to_string()),
            BuilderInput::TypeTInput(ow) => owner::to_string(ow),
        };
        self
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
    pub fn set_count(mut self, count: &str) -> Self {
        self.count = count.to_string();
        self
    }
    /// If search text is not set, the default is an empty string.
    pub fn set_search_text(mut self, search_text: &str) -> Self {
        self.search_text = search_text.to_string();
        self
    }
}

/// EDGAR queries require a CIK with ten digits, however, most CIKs have less than ten digits.
/// Leading zeros must be added to the CIK to reach this ten digit requirement.
pub fn add_leading_zeros_to_cik(cik: &str) -> String {
    let mut result = cik.to_owned();
    while result.len() < 10 {
        result.insert(0, '0');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edgar_query::filing::FilingTypeOption::_10K;

    fn sample() -> EdgarQueryBuilder {
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
        let query = sample().set_filing_type(BuilderInput::TypeTInput(_10K));
        assert_eq!(query.filing_type, answer)
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
        let query = sample().set_owner(BuilderInput::TypeStr(answer));
        assert_eq!(query.owner.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_count() {
        let answer = "10";
        let query = sample().set_count(answer);
        assert_eq!(query.count.as_str(), "10")
    }
    #[test]
    fn edgar_query_builder_build() {
        let answer = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000078003&type=10-k&dateb=&owner=include&count=20&search_text=&output=atom".to_lowercase();
        let query = sample()
            .set_count("20")
            .set_filing_type(BuilderInput::TypeTInput(_10K))
            .build()
            .unwrap()
            .as_str()
            .to_lowercase();
        assert_eq!(query, answer)
    }
}
