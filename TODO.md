6/7/2023

I should create the functions that I need for now.
Other functions can be built later.
It is significantly more human-readable to query by ticker rather than CIK, but the SEC uses the CIK for their API. Therefore, I need to make it easy to grab the most updated CIK from the SEC with a company's ticker symbol.

- [ ] look ahead and look behind not supported. create a workaround. Perhaps double regex patterns.
- [ ] create a function that queries the SEC's cik txt file for a company by either ticker or cik
  - [ ] query the SEC's ticker file
    - https://www.sec.gov/include/ticker.txt
  - [ ] the new implementation can take an optional file path
- [ ] write documentation for this function using chatgpt
- [ ] read references
- [ ] figure how to publish a crate

## Design

- [ ] have USER_AGENT be an environment variable
- [ ] 

## Commands

USER_AGENT="Company Not Available toj320@gmail.com" cargo t

## References
- [py-edgar](https://github.com/joeyism/py-edgar/tree/master)