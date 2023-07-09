//! This module exists to aid users with setting the Owner.
//! Owner refers to individuals who own significant amounts of the company's stock.

use crate::error::EDGARError;

/// These are the options for owner allowed by EDGAR.
#[derive(Debug, PartialEq)]
pub enum OwnerOptions {
    /// "include" means include all documents regardless of the source.
    INCLUDE,
    /// "exclude" means exclude documents related to the company's director or officer ownership.
    EXCLUDE,
    /// "only" means only show documents related to the company's director or officer ownership.
    ONLY,
}
/// Owner refers to individuals who own significant amounts of the company's stock.
pub struct Owner;
impl Owner {
    /// Converts a string to an OwnerOption.
    /// Panics for strings that are not a string representation of an owner option.
    /// See [OwnerOptions] for a list of owner options.
    /// String input is **case-insensitive**
    pub fn owner_from_str(owner_option: &str) -> Result<OwnerOptions, EDGARError> {
        match owner_option.to_lowercase().as_str() {
            "include" => Ok(OwnerOptions::INCLUDE),
            "exclude" => Ok(OwnerOptions::EXCLUDE),
            "only" => Ok(OwnerOptions::ONLY),
            _ => Err(EDGARError::OwnerOptionNotFound),
        }
    }
    /// Converts an [OwnerOptions] to a lowercase string representation of that option.
    pub fn to_string(owner_option: OwnerOptions) -> String {
        match owner_option {
            OwnerOptions::INCLUDE => "include".to_string(),
            OwnerOptions::EXCLUDE => "exclude".to_string(),
            OwnerOptions::ONLY => "only".to_string(),
        }
    }
    /// Validates by converting string to an [OwnerOptions] and back.
    /// Panics if the string is not a real option.
    pub fn validate_owner_string(owner: &str) -> Result<String, EDGARError> {
        let owner = Owner::owner_from_str(owner)?;
        Ok(Owner::to_string(owner))
    }
}
