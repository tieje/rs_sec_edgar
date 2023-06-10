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



- [ ] use serde to deserialize xml or create a struct for the xml
- [ ] build get_latest_10Qs

- [ ] write documentation for everything. Include examples where necessary. [Tests in examples might be necessary](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)
- [ ] read references
- [ ] figure how to publish a crate
- [ ] learn about versioning

## Design

- [ ] have USER_AGENT be an environment variable
- [ ] 

## Commands

cargo t -- --nocapture

## References
- [py-edgar](https://github.com/joeyism/py-edgar/tree/master)
