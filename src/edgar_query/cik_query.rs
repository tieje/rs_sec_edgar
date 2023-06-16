//! This module provides functions to get the CIK from a ticker symbol.

use crate::edgar::{self, edgar_client};
use regex::Regex;
use reqwest::Url;
use std::io::BufRead;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

const TICKER_URL: &str = "https://www.sec.gov/include/ticker.txt";
/// The input of `fn new()` for [CIKQuery]
#[derive(Debug, PartialEq)]
enum CIKDictionaryLocation {
    /// The path of the file containing [this info](https://www.sec.gov/include/ticker.txt).
    FilePath(PathBuf),
    /// The default Url of [this](https://www.sec.gov/include/ticker.txt).
    Url(Url),
}
/// Provides a methods to get short CIKs (no leading zeros) from a ticker represented as string.
#[derive(Debug, PartialEq)]
pub struct CIKQuery {
    location: CIKDictionaryLocation,
}
impl CIKQuery {
    /// Instantiates [CIKQuery] with an optionally defined `file_path`.
    /// If the file path does not exist or the `file_path` is `None`, then the default URL is used.
    /// ```
    /// use sec_edgar::edgar_query::cik_query::CIKQuery;
    /// let ticker = Some("amd");
    /// let cik_query = CIKQuery::new(ticker);
    /// ```
    pub fn new(file_path: Option<&str>) -> CIKQuery {
        let path_or_default = file_path.unwrap_or("./ticker.txt");
        if Path::new(path_or_default).is_file() {
            ticker_file_location(path_or_default)
        } else {
            default_ticker_url_location()
        }
    }
    /// Returns the short CIK (no leading zeros) either from the default URL or from the file defined when [CIKQuery] was instantiated.
    /// ```
    /// use sec_edgar::edgar_query::cik_query::CIKQuery;
    /// async fn some_func() {
    ///     let ticker = "AMD";
    ///     let cik_query = CIKQuery::new(None);
    ///     // To save yourself a trip, you can define the path to a local file, which should be the same as what is found [here](https://www.sec.gov/include/ticker.txt).
    ///     // let cik_query = CIKQuery::new(Some("./your/local/file")).await;
    ///     let cik = cik_query.get_cik(ticker).await;
    /// }
    /// ```
    /// The ticker is **case-insensitive**.
    pub async fn get_cik(&self, ticker: &str) -> Result<String, edgar::Error> {
        let ticker_low = ticker.to_lowercase();
        match &self.location {
            CIKDictionaryLocation::Url(location) => {
                get_cik_from_web(location, ticker_low.as_str()).await
            }
            CIKDictionaryLocation::FilePath(location) => {
                get_cik_from_file(location, ticker_low.as_str())
            }
        }
    }
}
fn ticker_file_location(path: &str) -> CIKQuery {
    let path = Path::new(path);
    CIKQuery {
        location: CIKDictionaryLocation::FilePath(path.to_path_buf()),
    }
}
fn default_ticker_url_location() -> CIKQuery {
    let url = Url::parse(TICKER_URL).expect("SEC ticker URL is incorrect");
    CIKQuery {
        location: CIKDictionaryLocation::Url(url),
    }
}
async fn get_cik_from_web(location: &Url, ticker: &str) -> Result<String, edgar::Error> {
    let response = edgar_client().get(location.as_str()).send().await;
    match response {
        Err(e) => Err(edgar::Error::EDGARRequestFailed(e)),
        Ok(r) => {
            let body = r.text().await;
            match body {
                Err(e) => Err(edgar::Error::EDGARNoTextInResponse(e)),
                Ok(b) => find_cik_from_html(b.as_str(), ticker),
            }
        }
    }
}
fn get_cik_from_file(location: &PathBuf, ticker: &str) -> Result<String, edgar::Error> {
    let file = File::open(location.as_path()).expect("failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Err(e) => return Err(edgar::Error::FailedToReadLine(e)),
            Ok(l) => {
                let res = find_cik_from_line(&l, ticker);
                match res {
                    None => continue,
                    Some(r) => return Ok(r),
                };
            }
        };
    }
    Err(edgar::Error::CIKNotFound)
}
fn find_cik_from_html(body: &str, ticker: &str) -> Result<String, edgar::Error> {
    let ticker_cik_regex = match Regex::new(r"(?i)[a-zA-Z]+\s\d+") {
        Ok(n) => n,
        Err(e) => return Err(edgar::Error::RegexErr(e)),
    };
    let ticker_cik_matches = ticker_cik_regex.find_iter(body);
    for ticker_cik_match in ticker_cik_matches {
        let res = find_cik_from_line(ticker_cik_match.as_str(), ticker);
        match res {
            None => continue,
            Some(r) => return Ok(r),
        };
    }
    Err(edgar::Error::CIKNotFound)
}

fn find_cik_from_line(line: &str, ticker: &str) -> Option<String> {
    let mut parts = line.split_whitespace();
    let ticker_line = parts.next()?;
    let cik = parts.next()?;
    if ticker_line == ticker {
        return Some(cik.to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = r"The file could be placed anywhere. By default it will check ./ticker.txt. The file comes from:
    https://www.sec.gov/include/ticker.txt"]
    fn cik_query_valid_file_path() {
        let path: &str = "./ignore/ticker.txt";
        assert_eq!(CIKQuery::new(Some(path)), ticker_file_location(path))
    }
    #[test]
    fn cik_query_invalid_file_path() {
        let file = "not/even/real";
        assert_eq!(CIKQuery::new(Some(file)), default_ticker_url_location())
    }
    #[test]
    fn cik_query_no_file_path() {
        assert_eq!(CIKQuery::new(None), default_ticker_url_location())
    }
    #[test]
    fn cik_query_string_for_cik() {
        let body = r"rtntf	887028
        bac	70858
        ci	1739940
        c	831001";
        let ticker = "c";
        let res = find_cik_from_html(body, ticker);
        match res {
            Ok(r) => assert_eq!(r.as_str(), "831001"),
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
            Ok(r) => assert_eq!(r.as_str(), answer),
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
            Ok(r) => assert_eq!(r.as_str(), answer),
            _ => assert!(false),
        }
    }
}
