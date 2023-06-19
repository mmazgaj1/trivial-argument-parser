pub mod builder;
/// Legacy API argument module. Should not be use since parsable_argument is now the preffered way of defining arguments.
/// Most likely will be removed in future.
pub mod legacy_argument;
pub mod parsable_argument;

/// Defines how arguments can be identified.
#[derive(Debug)]
pub enum ArgumentIdentification {
    Short(char),
    Long(String),
    Both(char, String),
}
