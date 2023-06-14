use log::info;

pub fn info_multiples_of_10(quantity: u8) {
    let multiple_of_10 = quantity % 10;
    if multiple_of_10 > 0 {
        info!("The SEC only provides filings in multiples of 10 up to 100 filings. Any other number results in the nearest multiple of 10 rounded down.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_info_multiples_of_10() {
        info_multiples_of_10(19);
        assert!(true)
    }
}
