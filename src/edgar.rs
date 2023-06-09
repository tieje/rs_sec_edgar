use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, HOST, USER_AGENT},
    Client, Url,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};
const TICKER_URL: &str = "https://www.sec.gov/include/ticker.txt";
const HEADER_ACCEPT_ENCODING: HeaderValue = HeaderValue::from_static("gzip, deflate");
const HEADER_HOST: HeaderValue = HeaderValue::from_static("www.sec.gov");

#[derive(Debug, PartialEq)]
enum EdgarLocation {
    FilePath(PathBuf),
    Url(Url),
}

#[derive(Debug, PartialEq)]
pub struct Edgar {
    location: EdgarLocation,
}
impl Edgar {
    pub fn new(file_path: Option<&str>) -> Edgar {
        let path_or_default = file_path.unwrap_or("./ticker.txt");
        if Path::new(path_or_default).is_file() {
            ticker_file_location(path_or_default)
        } else {
            default_ticker_url_location()
        }
    }
    pub async fn get_cik(&self, ticker: &str) -> Option<String> {
        match &self.location {
            EdgarLocation::Url(location) => get_cik_from_web(location, ticker).await,
            EdgarLocation::FilePath(location) => get_cik_from_file(location, ticker),
        }
    }
}

pub async fn get_cik_from_web(location: &Url, ticker: &str) -> Option<String> {
    let response = sec_client()
        .get(location.as_str())
        .send()
        .await
        .expect("SEC EDGAR request failed");
    let body = response
        .text()
        .await
        .expect("no text available from response");
    find_cik_from_html(body.as_str(), ticker)
}

pub fn get_cik_from_file(location: &PathBuf, ticker: &str) -> Option<String> {
    let file = File::open(location.as_path()).expect("failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("failed to read line");
        let res = find_cik_from_line(&line, ticker);
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
        let res = find_cik_from_line(ticker_cik_match.as_str(), ticker);
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

pub fn sec_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_ENCODING, HEADER_ACCEPT_ENCODING);
    headers.insert(HOST, HEADER_HOST);
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            option_env!("USER_AGENT").expect("USER_AGENT environment variable not set"),
        ),
    );
    Client::builder()
        .default_headers(headers)
        .deflate(true)
        .gzip(true)
        .build()
        .expect("Building client failed")
}

pub fn ticker_file_location(path: &str) -> Edgar {
    let path = Path::new(path);
    Edgar {
        location: EdgarLocation::FilePath(path.to_path_buf()),
    }
}
pub fn default_ticker_url_location() -> Edgar {
    let url = Url::parse(TICKER_URL).expect("SEC ticker URL is incorrect");
    Edgar {
        location: EdgarLocation::Url(url),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "The file could be placed anywhere. By default it will check ./ticker.txt"]
    fn edgar_valid_file_path() {
        let path: &str = "./ignore/ticker.txt";
        assert_eq!(Edgar::new(Some(path)), ticker_file_location(path))
    }
    #[test]
    fn edgar_invalid_file_path() {
        let file = "not/even/real";
        assert_eq!(Edgar::new(Some(file)), default_ticker_url_location())
    }
    #[test]
    fn edgar_no_file_path() {
        assert_eq!(Edgar::new(None), default_ticker_url_location())
    }
    #[test]
    fn test_parse_string_for_cik() {
        let body = r"rtntf	887028
        bac	70858
        ci	1739940
        c	831001";
        let ticker = "c";
        let res = find_cik_from_html(body, ticker);
        match res {
            Some(r) => assert_eq!(r.as_str(), "831001"),
            _ => assert!(false),
        }
    }
    #[tokio::test]
    async fn test_edgar_get_cik_from_file() {
        let answer = "831001";
        let ticker = "c";
        let path: &str = "./ignore/ticker.txt";
        let edgar = Edgar::new(Some(path));
        let res = edgar.get_cik(ticker).await;
        match res {
            Some(r) => assert_eq!(r.as_str(), answer),
            _ => assert!(false),
        }
    }
    #[tokio::test]
    // #[ignore = "Expensive test and must be connected to the internet"]
    async fn test_edgar_get_cik_from_web() {
        let answer = "831001";
        let ticker = "c";
        let edgar = Edgar::new(None);
        let res = edgar.get_cik(ticker).await;
        match res {
            Some(r) => assert_eq!(r.as_str(), answer),
            _ => assert!(false),
        }
    }
}
