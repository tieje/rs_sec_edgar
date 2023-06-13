use regex::Regex;
use reqwest::Url;
use std::io::BufRead;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::edgar::edgar_client;

const TICKER_URL: &str = "https://www.sec.gov/include/ticker.txt";

#[derive(Debug, PartialEq)]
pub enum CIKDictionaryLocation {
    FilePath(PathBuf),
    Url(Url),
}

#[derive(Debug, PartialEq)]
pub struct CIKQuery {
    location: CIKDictionaryLocation,
}
impl CIKQuery {
    pub fn new(file_path: Option<&str>) -> CIKQuery {
        let path_or_default = file_path.unwrap_or("./ticker.txt");
        if Path::new(path_or_default).is_file() {
            CIKQuery::ticker_file_location(path_or_default)
        } else {
            CIKQuery::default_ticker_url_location()
        }
    }
    pub fn ticker_file_location(path: &str) -> CIKQuery {
        let path = Path::new(path);
        CIKQuery {
            location: CIKDictionaryLocation::FilePath(path.to_path_buf()),
        }
    }
    pub fn default_ticker_url_location() -> CIKQuery {
        let url = Url::parse(TICKER_URL).expect("SEC ticker URL is incorrect");
        CIKQuery {
            location: CIKDictionaryLocation::Url(url),
        }
    }
    pub async fn get_cik(&self, ticker: &str) -> Option<String> {
        match &self.location {
            CIKDictionaryLocation::Url(location) => Self::get_cik_from_web(location, ticker).await,
            CIKDictionaryLocation::FilePath(location) => Self::get_cik_from_file(location, ticker),
        }
    }
    pub async fn get_cik_from_web(location: &Url, ticker: &str) -> Option<String> {
        let response = edgar_client()
            .unwrap()
            .get(location.as_str())
            .send()
            .await
            .expect("SEC EDGAR request failed");
        let body = response
            .text()
            .await
            .expect("no text available from response");
        Self::find_cik_from_html(body.as_str(), ticker)
    }

    pub fn get_cik_from_file(location: &PathBuf, ticker: &str) -> Option<String> {
        let file = File::open(location.as_path()).expect("failed to open file");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("failed to read line");
            let res = Self::find_cik_from_line(&line, ticker);
            if res.is_some() {
                return res;
            }
        }
        None
    }

    pub fn find_cik_from_html(body: &str, ticker: &str) -> Option<String> {
        let ticker_cik_regex =
            Regex::new(r"(?i)[a-zA-Z]+\s\d+").expect("ticker_cik_pattern regex incorrect");
        let ticker_cik_matches = ticker_cik_regex.find_iter(body);
        for ticker_cik_match in ticker_cik_matches {
            let res = Self::find_cik_from_line(ticker_cik_match.as_str(), ticker);
            if res.is_some() {
                return res;
            }
        }
        None
    }

    pub fn find_cik_from_line(line: &str, ticker: &str) -> Option<String> {
        let regex_failure = "regex failure";
        let mut parts = line.split_whitespace();
        let ticker_line = parts.next().expect(&regex_failure);
        let cik = parts.next().expect(&regex_failure);
        if ticker_line == ticker {
            return Some(cik.to_string());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = r"The file could be placed anywhere. By default it will check ./ticker.txt. The file comes from:
    https://www.sec.gov/include/ticker.txt"]
    fn cik_query_valid_file_path() {
        let path: &str = "./ignore/ticker.txt";
        assert_eq!(
            CIKQuery::new(Some(path)),
            CIKQuery::ticker_file_location(path)
        )
    }
    #[test]
    fn cik_query_invalid_file_path() {
        let file = "not/even/real";
        assert_eq!(
            CIKQuery::new(Some(file)),
            CIKQuery::default_ticker_url_location()
        )
    }
    #[test]
    fn cik_query_no_file_path() {
        assert_eq!(CIKQuery::new(None), CIKQuery::default_ticker_url_location())
    }
    #[test]
    fn cik_query_string_for_cik() {
        let body = r"rtntf	887028
        bac	70858
        ci	1739940
        c	831001";
        let ticker = "c";
        let res = CIKQuery::find_cik_from_html(body, ticker);
        match res {
            Some(r) => assert_eq!(r.as_str(), "831001"),
            _ => assert!(false),
        }
    }
    #[tokio::test]
    async fn cik_query_get_cik_from_file() {
        let answer = "831001";
        let ticker = "c";
        let path: &str = "./ignore/ticker.txt";
        let edgar = CIKQuery::new(Some(path));
        let res = edgar.get_cik(ticker).await;
        match res {
            Some(r) => assert_eq!(r.as_str(), answer),
            _ => assert!(false),
        }
    }
    #[tokio::test]
    // #[ignore = "Expensive test and must be connected to the internet"]
    async fn cik_query_get_cik_from_web() {
        let answer = "831001";
        let ticker = "c";
        let edgar = CIKQuery::new(None);
        let res = edgar.get_cik(ticker).await;
        match res {
            Some(r) => assert_eq!(r.as_str(), answer),
            _ => assert!(false),
        }
    }
}
