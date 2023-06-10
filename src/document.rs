

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

    use std::fs::File;
    use std::io::BufReader;
    use atom_syndication::Feed;

    #[test]
    // #[ignore = "the test file can be anywhere. Checks ./ignore/atom_test.xml"]
    fn test_reading_atom() {
        let file = File::open("./ignore/atom_test.xml").unwrap();
        let feed = Feed::read_from(BufReader::new(file)).unwrap();
        let first_entry_content = &feed.entries.first().unwrap().content;
        let first_entry_value = first_entry_content.clone().unwrap().value.unwrap();
        dbg!(first_entry_value);
        assert!(true)
    }
    #[test]
    fn adding_leading_zeros_to_cik() {
        let answer = "0000000123".to_string();
        assert_eq!(add_leading_zeros_to_cik("123"), answer)
    }
}
