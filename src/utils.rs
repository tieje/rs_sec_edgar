use log::info;
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, HOST, USER_AGENT},
    Client, Url,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

const HEADER_ACCEPT_ENCODING: HeaderValue = HeaderValue::from_static("gzip, deflate");
const HEADER_HOST: HeaderValue = HeaderValue::from_static("www.sec.gov");

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

pub fn info_multiples_of_10(quantity: u8) {
    let multiple_of_10 = quantity % 10;
    if multiple_of_10 > 0 {
        info!("The SEC only provides filings in multiples of 10 up to 100 filings. Any other number results in the nearest multiple of 10 rounded down.")
    }
}

pub fn add_leading_zeros_to_cik(cik: &str) -> String {
    let mut result = cik.to_owned();
    while result.len() < 10 {
        result.insert_str(0, "0");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_leading_zeros_to_cik() {
        let answer = "0000000123".to_string();
        assert_eq!(add_leading_zeros_to_cik("123"), answer)
    }
    #[test]
    fn test_info_multiples_of_10() {
        info_multiples_of_10(19);
        assert!(true)
    }
}
