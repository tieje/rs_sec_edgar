use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, HOST, USER_AGENT},
    Client,
};

const HEADER_ACCEPT_ENCODING: HeaderValue = HeaderValue::from_static("gzip, deflate");
const HEADER_HOST: HeaderValue = HeaderValue::from_static("www.sec.gov");

pub fn sec_client() -> Result<Client, reqwest::Error> {
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
}

#[cfg(test)]
mod tests {
    use crate::edgar_query::edgar_query_builder::FilingInput;
    use crate::edgar_query::filing_types::FilingType::_10Q;
    use crate::edgar_query::{cik_query::CIKQuery, edgar_query_builder::EdgarQueryBuilder};

    use super::*;
    #[tokio::test]
    // #[ignore = "Tests with local file. The file could be put anywhere."]
    async fn edgar_sample_query() {
        let ticker = "c";
        let cik_query = CIKQuery::new(Some("./ignore/ticker.txt"))
            .get_cik(ticker)
            .await
            .expect("ticker not found");
        let edgar_query = EdgarQueryBuilder::new(&cik_query)
            .set_filing_type(FilingInput::TypeFiling(_10Q))
            .build()
            .unwrap();
        let _response = sec_client()
            .unwrap()
            .get(edgar_query.as_str())
            .send()
            .await
            .unwrap();
        assert!(true)
        // finish example
        // response
    }
}
