use crate::utils::{get_cik_from_file, get_cik_from_web};
use reqwest::Url;
use std::path::{Path, PathBuf};

const TICKER_URL: &str = "https://www.sec.gov/include/ticker.txt";

#[derive(Debug, PartialEq)]
pub enum EdgarLocation {
    FilePath(PathBuf),
    Url(Url),
}

#[derive(Debug, PartialEq)]
pub struct Edgar {
    pub(crate) location: EdgarLocation,
}
impl Edgar {
    pub fn new(file_path: Option<&str>) -> Edgar {
        let path_or_default = file_path.unwrap_or("./ticker.txt");
        if Path::new(path_or_default).is_file() {
            Edgar::ticker_file_location(path_or_default)
        } else {
            Edgar::default_ticker_url_location()
        }
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
    pub async fn get_cik(&self, ticker: &str) -> Option<String> {
        match &self.location {
            EdgarLocation::Url(location) => get_cik_from_web(location, ticker).await,
            EdgarLocation::FilePath(location) => get_cik_from_file(location, ticker),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::find_cik_from_html;

    use super::*;
    #[test]
    #[ignore = r"The file could be placed anywhere. By default it will check ./ticker.txt. The file comes from:
    https://www.sec.gov/include/ticker.txt"]
    fn edgar_valid_file_path() {
        let path: &str = "./ignore/ticker.txt";
        assert_eq!(Edgar::new(Some(path)), Edgar::ticker_file_location(path))
    }
    #[test]
    fn edgar_invalid_file_path() {
        let file = "not/even/real";
        assert_eq!(Edgar::new(Some(file)), Edgar::default_ticker_url_location())
    }
    #[test]
    fn edgar_no_file_path() {
        assert_eq!(Edgar::new(None), Edgar::default_ticker_url_location())
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
