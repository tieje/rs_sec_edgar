# SEC EDGAR

This crate provides tools to query the SEC's EDGAR API.

## Usage

Define USER_AGENT in your environment variables. [According to the SEC](https://www.sec.gov/os/webmaster-faq#developers), it must be in the format:

```
USER_AGENT="Sample Company Name AdminContact@<sample company domain>.com"
```

Sample Query:

```rust
use sec_edgar::{
    edgar::{edgar_client, get_feed_entries, get_feed_entry_content},
    edgar_query::{
        cik_query::CIKQuery,
        filing::FilingTypeOption::_10Q,
        edgar_query_builder::{BuilderInput, EdgarQueryBuilder}
   },
};

async fn some_func() {
    let ticker = "c";
    // To save yourself a trip, you can store the file locally and query it instead.
    // The file can be downloaded from [here](https://www.sec.gov/include/ticker.txt).
    // let cik_query = CIKQuery::new(Some("./ignore/ticker.txt"))
    let cik_query = CIKQuery::new(None)
        .unwrap()
        .get_cik(ticker)
        .await
        .unwrap();
    let query = EdgarQueryBuilder::new(&cik_query)
        .set_filing_type(BuilderInput::TypeTInput(_10Q))
        .build()
        .unwrap();
    let entries = get_feed_entries(edgar_client().unwrap(), query).await.unwrap();
    let filing_type = get_feed_entry_content(entries.first().unwrap())
        .unwrap()
        .filing_type
        .value;
}
```
