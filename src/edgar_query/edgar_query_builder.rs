use crate::utils;

use super::{
    filing_types::{Filing, FilingType},
    owner::{Owner, OwnerOptions},
};

#[derive(Debug, PartialEq)]
pub enum FilingInput {
    TypeStr(String),
    TypeF(FilingType),
}
#[derive(Debug, PartialEq)]
pub enum OwnerInput {
    TypeStr(String),
    TypeOwner(OwnerOptions),
}
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
        let base = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&".to_string();
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
    pub fn build(&self) -> EdgarQuery {
        let query = format!("{base}CIK={cik}&type={filing_type}&dateb={dateb}&owner={owner}&count={count}&search_text={search_text}",
            base = self.base,
            cik = self.cik,
            filing_type = self.filing_type,
            dateb = self.dateb,
            owner = self.owner,
            count = self.count,
            search_text = self.search_text
        );
        EdgarQuery { query }
    }
    pub fn set_filing_type(&mut self, filing_type: FilingInput) {
        match filing_type {
            FilingInput::TypeStr(f) => {
                self.filing_type = Filing::validate_filing_type_string(f.as_str());
            }
            FilingInput::TypeF(f) => {
                self.filing_type = Filing::to_string(f);
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
    pub fn set_dateb(&mut self, yyyymmdd: &str) {
        self.dateb = yyyymmdd.to_string();
    }
    pub fn set_owner(&mut self, owner: OwnerInput) {
        match owner {
            OwnerInput::TypeStr(ow) => {
                self.filing_type = Owner::validate_owner_string(ow.as_str());
            }
            OwnerInput::TypeOwner(ow) => {
                self.filing_type = Owner::to_string(ow);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edgar_query::filing_types::FilingType::_10K;
    use FilingInput::TypeF;

    fn sample() -> EdgarQueryBuilder {
        EdgarQueryBuilder::new("78003")
    }
    #[test]
    fn edgar_query_builder_new() {
        let answer = "0000078003";
        assert_eq!(sample().cik.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_filing_type() {
        let answer = "10-K";
        let mut query = sample();
        query.set_filing_type(TypeF(_10K));
        assert_eq!(query.filing_type.as_str(), answer)
    }
    #[test]
    fn edgar_query_builder_set_dateb() {
        let answer = "20230105";
        let mut query = sample();
        query.set_dateb(&answer);
        assert_eq!(query.dateb.as_str(), answer)
    }
}
