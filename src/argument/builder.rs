use crate::argument::legacy_argument::{ArgType, Argument};

pub struct ArgBuilder {
    arg_type: ArgType,
    short_name: Option<char>,
    long_name: Option<String>,
}

/// Builder needs
impl ArgBuilder {
    pub fn new(arg_type: ArgType) -> ArgBuilder {
        return ArgBuilder {
            arg_type,
            short_name: None,
            long_name: None,
        };
    }

    pub fn set_short_name(mut self, short_name: char) -> ArgBuilder {
        self.short_name = Some(short_name);
        return self;
    }

    pub fn set_long_name(mut self, long_name: &str) -> ArgBuilder {
        self.long_name = Some(String::from(long_name));
        return self;
    }

    pub fn set_type(mut self, new_type: ArgType) -> ArgBuilder {
        self.arg_type = new_type;
        return self;
    }

    pub fn build(&self) -> Result<Argument, String> {
        let long = if let Some(ref l) = self.long_name {
            Option::Some(l.as_str())
        } else {
            Option::None
        };
        Argument::new(self.short_name, long, self.arg_type)
    }
}

#[cfg(test)]
mod tests {
    use super::{ArgBuilder, ArgType};

    #[test]
    fn new_works() {
        let arg_builder = ArgBuilder::new(ArgType::Value);
        assert_eq!(arg_builder.long_name, Option::None);
        assert_eq!(arg_builder.short_name, Option::None);
        assert_eq!(arg_builder.arg_type, ArgType::Value);
    }

    #[test]
    fn build_short_works() {
        let arg = ArgBuilder::new(ArgType::Value)
            .set_short_name('x')
            .build()
            .unwrap();
        assert_eq!(arg.short(), &Option::Some('x'));
        assert_eq!(arg.long(), &Option::None);
        assert_eq!(arg.arg_type(), &ArgType::Value);
    }

    #[test]
    fn build_long_works() {
        let arg = ArgBuilder::new(ArgType::Value)
            .set_long_name("my_arg")
            .build()
            .unwrap();
        assert_eq!(arg.long(), &Option::Some(String::from("my_arg")));
        assert_eq!(arg.short(), &Option::None);
        assert_eq!(arg.arg_type(), &ArgType::Value);
    }

    #[test]
    fn build_both_works() {
        let arg = ArgBuilder::new(ArgType::Value)
            .set_long_name("my_arg")
            .set_short_name('x')
            .build()
            .unwrap();
        assert_eq!(arg.long(), &Option::Some(String::from("my_arg")));
        assert_eq!(arg.short(), &Option::Some('x'));
        assert_eq!(arg.arg_type(), &ArgType::Value);
    }

    #[test]
    fn set_type_works() {
        let arg = ArgBuilder::new(ArgType::Value)
            .set_long_name("my_arg")
            .set_type(ArgType::Flag)
            .build()
            .unwrap();
        assert_eq!(arg.long(), &Option::Some(String::from("my_arg")));
        assert_eq!(arg.short(), &Option::None);
        assert_eq!(arg.arg_type(), &ArgType::Flag);
    }
}
