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

impl ArgumentIdentification {
    // Check if this identification can be identified by specified single character.
    pub fn is_by_short(&self, name: char) -> bool {
        if let ArgumentIdentification::Short(c) = self {
            return c == &name;
        }
        if let ArgumentIdentification::Both(c, _) = self {
            return c == &name;
        }
        false
    }

    // Check if this identification can be identified by specified string value.
    pub fn is_by_long(&self, name: &str) -> bool {
        if let ArgumentIdentification::Long(s) = &self {
            return s == name;
        }
        if let ArgumentIdentification::Both(_, s) = &self {
            return s == name;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::ArgumentIdentification;

    #[test]
    fn is_by_short_works() {
        let short_id = ArgumentIdentification::Short('x');
        assert!(short_id.is_by_short('x'));
        assert!(!short_id.is_by_short('c'));
        let both_id = ArgumentIdentification::Both('z', String::from("directory"));
        assert!(both_id.is_by_short('z'));
        assert!(!both_id.is_by_short('c'));
    }

    #[test]
    fn is_by_long_works() {
        let short_id = ArgumentIdentification::Long(String::from("path"));
        assert!(short_id.is_by_long("path"));
        assert!(!short_id.is_by_long("name"));
        let both_id = ArgumentIdentification::Both('z', String::from("file"));
        assert!(both_id.is_by_long("file"));
        assert!(!both_id.is_by_long("bar"));
    }
}
