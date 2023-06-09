6/7/2023

I should create the functions that I need for now.
Other functions can be built later.
It is significantly more human-readable to query by ticker rather than CIK, but the SEC uses the CIK for their API. Therefore, I need to make it easy to grab the most updated CIK from the SEC with a company's ticker symbol.

6/9/2023

0. For online, do a global match on the text to get a list of lines
1. Iterate over ticker cik lines.
   1. Separate the ticker and the cik with regex.
   2. If the ticker is the same as parameter ticker then return cik.
   3. If the ticker is not the same, then continue iterating

Only write documentation when you think you've reached a stopping point.
I think this point will be when I create the function `fn get_latest_10Q`

- [example query](https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=10-K&count=10&output=atom)

- [x] figure out how to parse the string
- [x] learn [how to write documentation](https://doc.rust-lang.org/rustdoc/write-documentation/what-to-include.html)
- [x] use crate [atom_syndication](https://crates.io/crates/atom_syndication) to utilize atom type

6/10/2023

```xml
<?xml version="1.0" encoding="ISO-8859-1" ?>
<accession-number>0000002488-23-000076</accession-number>
<act>34</act>
<file-number>001-07882</file-number>
<file-number-href>https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&filenum=001-07882&owner=include&count=10</file-number-href>
<filing-date>2023-05-03</filing-date>
<filing-href>https://www.sec.gov/Archives/edgar/data/2488/000000248823000076/0000002488-23-000076-index.htm</filing-href>
<filing-type>10-Q</filing-type>
<film-number>23884480</film-number>
<form-name>Quarterly report [Sections 13 or 15(d)]</form-name>
<size>7 MB</size>
<xbrl_href>https://www.sec.gov/cgi-bin/viewer?action=view&cik=2488&accession_number=0000002488-23-000076&xbrl_type=v</xbrl_href>
```

```css
#DataTables_Table_0
  > tbody
  > tr
  > td.release-number-content.views-field.views-field-field-release-number.is-active.sorting_1 {
  color: red;
}
```

```js
let a = document.querySelector(
  "#DataTables_Table_0 > tbody > tr > td.release-number-content.views-field.views-field-field-release-number.is-active.sorting_1"
);
```

- [x] use serde to deserialize xml or create a struct for the xml
- [x] split by \n
- [x] remove lines that contain href sign
- [x] prepend and append xml string with Filing

6/11/2023

- [x] add match arms to filing_types

6/12/2023

`https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=&dateb=&owner=include&count=10&search_text=s`

- missing dateb and type
  `https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=10Q&dateb=20230101&owner=include&count=10&search_text=s`

- https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&

  - CIK=0000002488
    - many possibilities in the thousands
  - type=10Q
    - many filing types but only in the hundreds
  - dateb=20230101
    - optional, but can easily be defined by user
  - owner=include
    - only three options
  - count=10
  - search_text=s

- [x] create edgar_query_builder
- [x] create an option to insert a date
- [x] create documentation to show how to format date
- [x] sort functions in utils.rs file into their submodules

6/13/2023

- [x] create convenience functions
- [x] finish edgar module

- [x] write documentation for everything. Include examples where necessary. [Tests in examples might be necessary](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)
- [x] figure how to publish a crate
  - https://doc.rust-lang.org/cargo/reference/publishing.html

6/14/2023

- [x] ~~set up rust doc~~
  - https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
- [x] learn about versioning
  - https://doc.rust-lang.org/cargo/reference/semver.html

6/16/2023

- [x] propagate errors for cik_query
- [x] propagate errors for filing_content_value
- [x] propagate errors with Result<> instead of panicking. Use an enum of errors to allow this
- [x] bump up version to 1.0.0
- [x] update changelog
- [x] push to main
- [x] publish to crates.io

7/9/2023

- try is deprecated. Handle errors generically https://dev.to/nathan20/how-to-handle-errors-in-rust-a-comprehensive-guide-1cco
- thiserror library is very useful for cutting down on repetition

- [x] change mod.rs system to modern file system
- [x] fix clippy errors
- [x] implement [thisError](https://crates.io/crates/thiserror)

7/10/2023

- [x] rather than returning a new Self, simply edit the data of the original Self struct

7/11/2023

- [x] apply knowledge about lifetimes, Box, Rc, Arc
- [x] push version 1.0.2

7/12/2023

- [x] mutate self instead of creating new self
- [x] deploy 1.0.3

## Commands

cargo t -- --nocapture

## References

- [py-edgar](https://github.com/joeyism/py-edgar/tree/master)

## Known Bugs

- [ ] The serde_xml_rs::from_str function fails for deserializing values that contain "=" sign.
