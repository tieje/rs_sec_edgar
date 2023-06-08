use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, HOST, USER_AGENT},
    Client, Url,
};
use std::path::{Path, PathBuf};
const TICKER_URL: &str = "https://www.sec.gov/include/ticker.txt";
const HEADER_ACCEPT_ENCODING: HeaderValue = HeaderValue::from_static("gzip, deflate");
const HEADER_HOST: HeaderValue = HeaderValue::from_static("www.sec.gov");
// const HEADER_USER_AGENT: HeaderValue = HeaderValue::from_static(
//     option_env!("USER_AGENT").expect("USER_AGENT environment variable not set"),
// );

#[derive(Debug, PartialEq)]
enum EdgarLocation {
    FilePath(PathBuf),
    Url(Url),
}

#[derive(Debug, PartialEq)]
pub struct Edgar {
    location: EdgarLocation,
}
///https://www.sec.gov/include/ticker.txt
impl Edgar {
    fn new(file_path: Option<&str>) -> Edgar {
        let path_or_default = file_path.unwrap_or("./ticker.txt");
        if Path::new(path_or_default).is_file() {
            ticker_file_location(path_or_default)
        } else {
            default_ticker_url_location()
        }
    }
    // async fn get_cik(&self, ticker: &str) -> &str {
    //     match self.location {
    //         EdgarLocation::FilePath(location) => "dbg!(location)",
    //         EdgarLocation::Url(location) => {
    //             let response = sec_client().get(location.as_str()).send().await.expect("No response from SEC");
    //             let body = response.text().await.unwrap();
    //             &body
    //             // parse body to cik later
    //         }
    //     }
    // }
}

// pub fn parse_string_for_cik(body: String, ticker: &str) -> &str {
pub fn parse_string_for_cik(body: String, ticker: &str) {
    let pattern = format!(r"(?<=(?i){})\s\d+", ticker);
    let regex_pattern = Regex::new(&pattern).expect("Regex pattern is incorrect");
    let regex_match = regex_pattern.find(&body).expect("Ticker symbol not found");
    dbg!(regex_match.as_str());
}

pub fn sec_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_ENCODING, HEADER_ACCEPT_ENCODING);
    headers.insert(HOST, HEADER_HOST);
    headers.insert(USER_AGENT, HeaderValue::from_static(option_env!("USER_AGENT").expect("USER_AGENT environment variable not set")));
    Client::builder()
        .default_headers(headers)
        .deflate(true)
        .gzip(true)
        .build()
        .expect("client build unsuccessful")
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
    fn test_parse_string_for_cik() {
        let body = "AMD 12345".to_string();
        let ticker = "amd";
        parse_string_for_cik(body, ticker);
        assert!(true)
    }
}
