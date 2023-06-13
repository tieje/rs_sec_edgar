use atom_syndication::{Entry, Feed};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, HOST, USER_AGENT},
    Client,
};
use url::Url;

use crate::edgar_query::filing_content_value::FilingContentValue;

const HEADER_ACCEPT_ENCODING: HeaderValue = HeaderValue::from_static("gzip, deflate");
const HEADER_HOST: HeaderValue = HeaderValue::from_static("www.sec.gov");

pub async fn get_feed(client: Client, query_url: Url) -> Feed {
    let feed = client
        .get(query_url.as_str())
        .send()
        .await
        .expect("Response error occurred with SEC")
        .text()
        .await
        .expect("Error with querying text")
        .parse::<Feed>()
        .expect("Could not parse response text");
    feed
}

pub async fn get_feed_entries(client: Client, query_url: Url) -> Vec<Entry> {
    let entries = get_feed(client, query_url).await.entries;
    entries
}

pub async fn get_feed_entry_content(entry: &Entry) -> FilingContentValue {
    let entry_content = entry.content.clone().unwrap();
    let content = FilingContentValue::new(entry_content);
    content
}

pub fn edgar_client() -> Client {
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
        .expect("Building client unsuccessful.")
}

#[cfg(test)]
mod tests {
    use crate::edgar_query::edgar_query_builder::FilingInput;
    use crate::edgar_query::filing_types::FilingType::_10Q;
    use crate::edgar_query::{cik_query::CIKQuery, edgar_query_builder::EdgarQueryBuilder};

    use super::*;
    #[tokio::test]
    #[ignore = "Tests with local file. The file could be put anywhere."]
    async fn edgar_sample_query() {
        let answer = "10-Q";
        let ticker = "c";
        let cik_query = CIKQuery::new(Some("./ignore/ticker.txt"))
            .get_cik(ticker)
            .await
            .expect("ticker not found");
        let edgar_query = EdgarQueryBuilder::new(&cik_query)
            .set_filing_type(FilingInput::TypeFiling(_10Q))
            .build()
            .unwrap();
        let entries = get_feed_entries(edgar_client(), edgar_query).await;
        let filing_type = get_feed_entry_content(entries.first().unwrap())
            .await
            .filing_type
            .value;
        assert_eq!(filing_type.as_str(), answer);
    }
}
