pub mod legacy_argument;
pub mod parsable_argument;

/// Defines how arguments can be identified.
#[derive(Debug)]
pub enum ArgumentIdentification {
    Short(char),
    Long(String),
    Both(char, String),
}
