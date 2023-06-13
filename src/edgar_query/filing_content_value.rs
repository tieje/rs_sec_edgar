use atom_syndication::Content;
use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AccessionNumber {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Act {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FileNumber {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FileNumberHref {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilingDate {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilingHref {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilingType {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilmNumber {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FormName {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Size {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct XbrlHref {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilingContentValue {
    #[serde(rename = "accession-number")]
    pub accession_number: AccessionNumber,
    pub act: Act,
    #[serde(rename = "file-number")]
    pub file_number: FileNumber,
    #[serde(rename = "filing-date")]
    pub filing_date: FilingDate,
    #[serde(rename = "filing-type")]
    pub filing_type: FilingType,
    #[serde(rename = "film-number")]
    pub film_number: FilmNumber,
    #[serde(rename = "form-name")]
    pub form_name: FormName,
    pub size: Size,
    // Might be able to parse someday.
    // The serde_xml_rs::from_str function fails for deserializing values that contain "=" sign.
    // This might an easy fix later.
    // #[serde(rename = "file-name-href")]
    // pub file_number_href: FileNumberHref,
    // #[serde(rename = "filing-href")]
    // pub filing_href: FilingHref,
    // #[serde(rename = "xbrl-href")]
    // pub xbrl_href: XbrlHref,
}

impl FilingContentValue {
    pub fn new(content: Content) -> Self {
        let value = content.value.unwrap();
        let mut processed_values = value
            .split("\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|&line| !line.contains("href"))
            .collect::<Vec<&str>>()
            .join("");
        processed_values.insert_str(0, "<ContentValue>");
        processed_values.push_str("</ContentValue>");
        let filing: Self = from_str(processed_values.as_str()).unwrap();
        filing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use atom_syndication::Feed;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    #[ignore = r"The test file could be placed anywhere. This test checks ./ignore/atom_test.xml. This file comes from:
    https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=10-K&count=10&output=atom"]
    fn test_reading_atom() {
        let file = File::open("./ignore/atom_test.xml").unwrap();
        let feed = Feed::read_from(BufReader::new(file)).unwrap();
        let first_entry_content = feed.entries
            .first()
            .unwrap()
            .content
            .clone()
            .unwrap();
        let content = FilingContentValue::new(first_entry_content);
        dbg!(content.filing_date.value);
        assert!(true)
    }
}
