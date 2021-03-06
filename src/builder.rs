use crate::*;

pub struct ArgBuilder
{
    arg_type: ArgType,
    short_name: Option<char>,
    long_name: Option<String>,
}

/// Builder needs
impl ArgBuilder
{
    pub fn new(arg_type: ArgType) -> ArgBuilder
    {
        return ArgBuilder{arg_type, short_name: None, long_name: None}
    }

    pub fn set_short_name(mut self, short_name: char) -> ArgBuilder
    {
        self.short_name = Some(short_name);
        return self
    }

    pub fn set_long_name(mut self, long_name: & str) -> ArgBuilder
    {
        self.long_name = Some(String::from(long_name));
        return self
    }

    pub fn change_type(mut self, new_type: ArgType) -> ArgBuilder
    {
        self.arg_type = new_type;
        return self
    }
}

mod tests
{
    use super::*;

    #[test]
    fn it_works()
    {
        let mut arg_builder = ArgBuilder::new(ArgType::Value);
    }
}
