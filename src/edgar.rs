//! This module holds functions that can be used to query the SEC's EDGAR.
//!
//! Usage:
//! ```
//! use sec_edgar::edgar::{edgar_client, get_feed_entries, get_feed_entry_content};
//! use sec_edgar::edgar_query::cik_query::CIKQuery;
//! use sec_edgar::edgar_query::filing_types::FilingTypeOption::_10Q;
//! use sec_edgar::edgar_query::edgar_query_builder::{FilingInput, EdgarQueryBuilder};
//!
//! async fn some_func() {
//!     let ticker = "c";
//!     // To save yourself a trip, you can store the file locally and query it instead.
//!     // The file can be downloaded from [here](https://www.sec.gov/include/ticker.txt).
//!     // let cik_query = CIKQuery::new(Some("./ignore/ticker.txt"))
//!     let cik_query = CIKQuery::new(None)
//!         .unwrap()
//!         .get_cik(ticker)
//!         .await
//!         .unwrap();
//!     let query = EdgarQueryBuilder::new(&cik_query)
//!         .set_filing_type(FilingInput::TypeFiling(_10Q))
//!         .unwrap()
//!         .build()
//!         .unwrap();
//!     let entries = get_feed_entries(edgar_client().unwrap(), query).await.unwrap();
//!     let filing_type = get_feed_entry_content(entries.first().unwrap())
//!         .await
//!         .unwrap()
//!         .filing_type
//!         .value;
//! }
//! ```

use crate::edgar_query::filing_content_value::FilingContentValue;
use crate::error::EDGARError;
use atom_syndication::{Entry, Feed};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, HOST, USER_AGENT},
    Client,
};
use url::Url;

/// There is additional information in the atom formatted feed that can be extracted if desired.
/// An example of such info prior to an entry is shown below.
///
/// From `https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=10-K&count=10&output=atom`
/// ```xml
/// <!-- snip -->
/// <company-info>
/// <!-- snip -->
/// <conformed-name>ADVANCED MICRO DEVICES INC</conformed-name>
/// <fiscal-year-end>1230</fiscal-year-end>
/// <office>Office of Manufacturing</office>
/// <state-location>CA</state-location>
/// <!-- snip -->
/// </company-info>
/// <entry>
///     <category label="form type" scheme="https://www.sec.gov/" term="10-Q" />
///     <content type="text/xml">
///         <accession-number>0000002488-23-000076</accession-number>
///         <act>34</act>
/// <!-- snip -->
/// </entry>
/// <!-- snip -->
/// ```
///
/// Usage:
/// ```
/// use sec_edgar::edgar::{edgar_client, get_feed};
/// use url::Url;
/// let some_url = Url::parse("https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=10-K&count=10&output=atom").unwrap();
/// let client = edgar_client().unwrap();
/// let feed = get_feed(client, some_url);
/// ```
pub async fn get_feed(client: Client, query_url: Url) -> Result<Feed, EDGARError> {
    let res = client.get(query_url.as_str()).send().await?;
    Ok(res.text().await?.parse::<Feed>()?)
    // match client.get(query_url.as_str()).send().await {
    //     Err(_) => Err(EDGARError::GettingFeedFailed),
    //     Ok(f) => match f.text().await {
    //         Err(e) => Err(EDGARError::ReqwestError(e)),
    //         Ok(t) => t.parse::<Feed>()?
    //     },
    // }
}

/// Get the feed entries, which will be in atom format.
/// A brief example of such an entry can be found below.
///
/// From `https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=10-K&count=10&output=atom`
/// ```xml
/// <!-- snip -->
/// <entry>
///     <category label="form type" scheme="https://www.sec.gov/" term="10-Q" />
///     <content type="text/xml">
///         <accession-number>0000002488-23-000076</accession-number>
///         <act>34</act>
///     <!-- snip -->
///     </content>
/// </entry>
/// <!-- snip -->
/// ```
///
/// Usage:
/// ```
/// use sec_edgar::edgar::{edgar_client, get_feed_entries};
/// use url::Url;
/// async fn some_func() {
///     let some_url = Url::parse("https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=10-K&count=10&output=atom").unwrap();
///     let client = edgar_client().unwrap();
///     let feed_entries = get_feed_entries(client, some_url).await.unwrap();
/// }
/// ```
pub async fn get_feed_entries(client: Client, query_url: Url) -> Result<Vec<Entry>, EDGARError> {
    let entries = get_feed(client, query_url).await?.entries;
    Ok(entries)
}
/// Get the content of a feed entry.
/// Because the serde-xml-rs crate fails at parsing XML values with an `=` symbol, URL links have been removed.
///
/// Usage:
/// ```
/// use sec_edgar::edgar::{edgar_client, get_feed_entry_content, get_feed_entries};
/// use url::Url;
/// async fn some_func() {
///     let some_url = Url::parse("https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=10-K&count=10&output=atom").unwrap();
///     let client = edgar_client().unwrap();
///     let feed_entries = get_feed_entries(client, some_url).await.unwrap();
///     let first_entry = feed_entries.first().unwrap();
///     let first_entry_content = get_feed_entry_content(first_entry);
/// }
/// ```
pub fn get_feed_entry_content<'a>(entry: &Entry) -> Result<FilingContentValue<'a>, EDGARError> {
    let entry_content = match &entry.content {
        None => return Err(EDGARError::FilingContentNotFound),
        Some(a) => a,
    };
    FilingContentValue::new(entry_content.clone())
}
/// Returns a client that can send requests to EDGAR.
/// Please define the `USER_AGENT` in your environment variables.
/// [According to the SEC](https://www.sec.gov/os/webmaster-faq#developers), the `USER_AGENT` must be in the form:
/// ```txt
/// Sample Company Name AdminContact@<sample company domain>.com
/// ```
/// For Rust apps, I recommend defining it in [`/your_project/.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure)
///
/// Usage:
/// ```
/// use sec_edgar::edgar::edgar_client;
/// let client = edgar_client();
/// ```
pub fn edgar_client() -> Result<Client, EDGARError> {
    let mut headers = HeaderMap::new();
    let user_agent = match option_env!("USER_AGENT") {
        Some(u) => u,
        None => return Err(EDGARError::UserAgentEnvVarMissing),
    };
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
    headers.insert(HOST, HeaderValue::from_static("www.sec.gov"));
    headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
    Ok(Client::builder()
        .default_headers(headers)
        .deflate(true)
        .gzip(true)
        .build()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edgar_query::{
        cik_query::CIKQuery, edgar_query_builder::{EdgarQueryBuilder, BuilderInput}, filing::FilingTypeOption::{_10Q, self},
    };

    async fn edgar_sample_query_ending(cik_query: String) {
        let answer = "10-Q";
        let query = EdgarQueryBuilder::new(&cik_query)
            .set_filing_type(BuilderInput::<FilingTypeOption>::TypeTInput(_10Q))
            .build();
        let entries = get_feed_entries(edgar_client().unwrap(), query).await;
        let filing_type = get_feed_entry_content(entries.unwrap().first().unwrap())
            .unwrap()
            .filing_type
            .value;
        assert_eq!(filing_type.as_str(), answer);
    }
    #[tokio::test]
    #[ignore = "Tests with local file. The file could be put anywhere."]
    async fn edgar_sample_query_local_file() {
        let ticker = "c";
        let cik_query = CIKQuery::new(Some("./ignore/ticker.txt"))
            .unwrap()
            .get_cik(ticker)
            .await
            .expect("ticker not found");
        edgar_sample_query_ending(cik_query).await
    }
    #[tokio::test]
    async fn edgar_sample_query_web() {
        let ticker = "c";
        let cik_query = CIKQuery::new(None)
            .unwrap()
            .get_cik(ticker)
            .await
            .expect("ticker not found");
        edgar_sample_query_ending(cik_query).await
    }
}
