//! This module is used for extracting the content type of a filing.
use crate::error::EDGARError;
use atom_syndication::Content;
use serde::Deserialize;
use serde_xml_rs::from_str;

/// The Unique identifier assigned to each filing.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AccessionNumber {
    /// value of AccessionNumber
    #[serde(rename = "$value")]
    pub value: String,
}
/// The specific legislation or law under which a filing is made.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Act {
    /// value of Act
    #[serde(rename = "$value")]
    pub value: String,
}
/// A unique number assigned to each filing and helps in tracking and referencing specific submissions.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FileNumber {
    /// value of FileNumber
    #[serde(rename = "$value")]
    pub value: String,
}
/// The link to the File Number reference.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FileNumberHref {
    /// value of FileNumberHref
    #[serde(rename = "$value")]
    pub value: String,
}
/// The date at which the filing was made.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilingDate {
    /// value of FilingDate
    #[serde(rename = "$value")]
    pub value: String,
}
/// The filing link
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilingHref {
    /// value of FilingHref
    #[serde(rename = "$value")]
    pub value: String,
}
/// The Filing Type.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilingType {
    /// value of FilingType
    #[serde(rename = "$value")]
    pub value: String,
}
/// A unique identifier assigned to the microfilm version of a filing.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilmNumber {
    /// value of FilmNumber
    #[serde(rename = "$value")]
    pub value: String,
}
/// The full name of the abbreviated form type.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FormName {
    /// value of FormName
    #[serde(rename = "$value")]
    pub value: String,
}
/// Human-readable file size.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Size {
    /// value of Size
    #[serde(rename = "$value")]
    pub value: String,
}
/// An ancient alien language.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct XbrlHref {
    /// value of XbrlHref
    #[serde(rename = "$value")]
    pub value: String,
}
/// Provides structure for the feed's entry's content, which is in the form of Some("string") in Rust.
/// Href values are not provided because the serde_xml_rs crate cannot deserialize values that contain the `=` symbol.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FilingContentValue {
    /// See [AccessionNumber]
    #[serde(rename = "accession-number")]
    pub accession_number: AccessionNumber,
    /// See [Act]
    pub act: Act,
    /// See [FileNumber]
    #[serde(rename = "file-number")]
    pub file_number: FileNumber,
    /// See [FilingDate]
    #[serde(rename = "filing-date")]
    pub filing_date: FilingDate,
    #[serde(rename = "filing-type")]
    /// See [FilingType]
    pub filing_type: FilingType,
    #[serde(rename = "film-number")]
    /// See [FilmNumber]
    pub film_number: FilmNumber,
    /// See [FormName]
    #[serde(rename = "form-name")]
    pub form_name: FormName,
    /// See [Size]
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
    /// Instantiates the [FilingContentValue] to deserialize the content of an entry from a feed with atom format.
    pub fn new(content: Content) -> Result<Self, EDGARError> {
        let value = content
            .value
            .ok_or(EDGARError::FilingContentValueNotFound)?;
        let mut processed_values = value
            .split('\n')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|&line| !line.contains("href"))
            .collect::<Vec<&str>>()
            .join("");
        processed_values.insert_str(0, "<ContentValue>");
        processed_values.push_str("</ContentValue>");
        let filing: Self = from_str(processed_values.as_str())?;
        Ok(filing)
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
        let first_entry_content = feed.entries.first().unwrap().content.clone().unwrap();
        let content = FilingContentValue::new(first_entry_content);
        assert!(content.is_ok())
    }
}
