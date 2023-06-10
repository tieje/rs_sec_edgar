use log::info;

// pub fn get_10qs(cik: &str, quantity: u8) {
// }
// pub fn get_10ks(cik: &str, quantity: u8) {}


pub fn info_multiples_of_10(quantity: u8) {
    let multiple_of_10 = quantity % 10;
    if multiple_of_10 > 0 {
        info!("The SEC only provides filings in multiples of 10 up to 100 filings. Any other number results in the nearest multiple of 10 rounded down.")
    }
}

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

    #[test]
    fn adding_leading_zeros_to_cik() {
        let answer = "0000000123".to_string();
        assert_eq!(add_leading_zeros_to_cik("123"), answer)
    }
    #[test]
    fn test_info_multiples_of_10() {
        info_multiples_of_10(19);
        assert!(true)
    }
}