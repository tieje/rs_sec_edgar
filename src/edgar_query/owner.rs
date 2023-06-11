#[derive(Debug, PartialEq)]
pub enum OwnerOptions {
    INCLUDE,
    EXCLUDE,
    ONLY,
}
pub struct Owner;
impl Owner {
    pub fn from_str(owner_option: &str) -> OwnerOptions {
        match owner_option {
            "include" => OwnerOptions::INCLUDE,
            "exclude" => OwnerOptions::EXCLUDE,
            "only" => OwnerOptions::ONLY,
            _ => panic!("owner option does not exist"),
        }
    }
    pub fn to_str(owner_option: OwnerOptions) -> String {
        match owner_option {
            OwnerOptions::INCLUDE => "include".to_string(),
            OwnerOptions::EXCLUDE => "exclude".to_string(),
            OwnerOptions::ONLY => "only".to_string(),
        }
    }
}
