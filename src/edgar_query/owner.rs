#[derive(Debug, PartialEq)]
pub enum OwnerOptions {
    INCLUDE,
    EXCLUDE,
    ONLY,
}
pub struct Owner;
impl Owner {
    pub fn from_str(owner_option: &str) {
        match owner_option {
            "include" => OwnerOptions::INCLUDE,
            "exclude" => OwnerOptions::EXCLUDE,
            "only" => OwnerOptions::ONLY,
            _ => panic!("")
        }
    }
    pub fn to_str() {}
}
