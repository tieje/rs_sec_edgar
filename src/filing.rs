use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccessionNumber {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct Act {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct FileNumber {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct FileNumberHref {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct FilingDate {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct FilingHref {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct FilingType {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct FilmNumber {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct FormName {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct Size {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub struct XbrlHref {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Filing {
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
    // keep. Might be able to parse someday.
    // The serde_xml_rs::from_str function fails for deserializing values that contain "=" sign.
    // This might an easy fix later.
    // #[serde(rename = "file-name-href")]
    // pub file_number_href: FileNumberHref,
    // #[serde(rename = "filing-href")]
    // pub filing_href: FilingHref,
    // #[serde(rename = "xbrl-href")]
    // pub xbrl_href: XbrlHref,
}

// pub fn get_latest_10k_entries(cik: &str, entries: u8) {
// }
// pub fn get_latest_10K_date(cik: &str) {}

pub fn add_leading_zeros_to_cik(cik: &str) -> String {
    let mut result = cik.to_owned();
    while result.len() < 10 {
        result.insert_str(0, "0");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use atom_syndication::Feed;
    use serde_xml_rs::from_str;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    #[ignore = r"The test file could be placed anywhere. This test checks ./ignore/atom_test.xml. This file comes from:
    https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000002488&type=10-K&count=10&output=atom"]
    fn test_reading_atom() {
        let file = File::open("./ignore/atom_test.xml").unwrap();
        let feed = Feed::read_from(BufReader::new(file)).unwrap();
        let first_entry_content = &feed.entries.first().unwrap().content;
        let first_entry_value = first_entry_content.clone().unwrap().value.unwrap();
        let mut entry = first_entry_value
            .split("\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|&line| !line.contains("href"))
            .collect::<Vec<&str>>()
            .join("");
        entry.insert_str(0, "<Filing>");
        entry.push_str("</Filing>");
        let filing: Filing = from_str(entry.as_str()).unwrap();
        dbg!(filing.filing_date.value);
        assert!(true)
    }
    #[test]
    fn adding_leading_zeros_to_cik() {
        let answer = "0000000123".to_string();
        assert_eq!(add_leading_zeros_to_cik("123"), answer)
    }
}
