use std::iter::Peekable;

/**
Enum allowing to choose the type of argument.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgType {
    Flag,
    Value,
    ValueList,
}

/**
ArgResult enum is similar to ArgType enum but contains data generated through parsing
*/
#[derive(Debug, PartialEq)]
pub enum ArgResult {
    Flag,
    Value(String),
    ValueList(Vec<String>),
}

///
/// Argument struct allows to specify type of expected argument, its names and after parsing contains results.
/// This is the legacy method of defining arguments. Currently using ParsableValueArgument is preffered.
///
/// # Examples
/// ```
/// use trivial_argument_parser::*;
/// let mut example_argument = Argument::new(Some('l'), Some("an-list"), ArgType::ValueList).unwrap();
/// ```

#[derive(Debug)]
pub struct Argument {
    short: Option<char>,
    long: Option<String>,
    arg_type: ArgType,
    pub arg_result: Option<ArgResult>,
}

impl Argument {
    /**
    Create new Argument. You need to specify at least one name (short or long) or you can specify both. Parameter arg_type changes how the parsing will treat the argument.
    */
    pub fn new(
        short: Option<char>,
        long: Option<&str>,
        arg_type: ArgType,
    ) -> Result<Argument, String> {
        // Check if at least 1 name is specified
        if let (Option::None, Option::None) = (short, long) {
            return Err(String::from(
                "At least one name of argument must be specified (short or long or both)",
            ));
        }

        // Check if long name is defined, if so use it
        let long_owned: Option<String> = if let Some(text) = long {
            Option::Some(String::from(text))
        } else {
            None
        };

        Ok(Argument {
            short,
            long: long_owned,
            arg_type,
            arg_result: None,
        })
    }

    pub fn new_short(name: char, arg_type: ArgType) -> Argument {
        Argument::new(Option::Some(name), Option::None, arg_type).unwrap()
    }

    pub fn new_long(name: &str, arg_type: ArgType) -> Argument {
        Argument::new(Option::None, Option::Some(name), arg_type).unwrap()
    }

    ///
    /// Method allowing to simplify reading values of a single value type arguments.
    ///
    ///# Examples
    ///```
    /// use trivial_argument_parser::*;
    /// let mut args_list = ArgumentList::new();
    /// args_list.append_arg(Argument::new(Some('v'), None, ArgType::Value).unwrap());
    /// args_list.parse_args(vec![String::from("-v"), String::from("VALUE")]).unwrap();
    /// let value = args_list.search_by_short_name('v').unwrap().get_value().unwrap();
    /// println!("Value: {}", value);
    ///```

    pub fn get_value(&self) -> Result<&str, &'static str> {
        if let ArgType::Value = self.arg_type {
            if let Some(result) = &self.arg_result {
                if let ArgResult::Value(ref value) = result {
                    return Ok(value);
                } else {
                    return Err("Wrong type of result. Something really bad has happened");
                }
            } else {
                return Err("No value assigned to result");
            }
        } else {
            return Err("This argument is not an value");
        }
    }
    ///
    /// Method allowing to simplify reading values of a value list type argument.
    ///
    ///# Examples
    ///```
    /// use trivial_argument_parser::*;
    /// let mut args_list = ArgumentList::new();
    /// args_list.append_arg(Argument::new(Some('l'), None, ArgType::ValueList).unwrap());
    /// args_list.parse_args(vec![String::from("-l"), String::from("cos")]).unwrap();
    /// let list = args_list.search_by_short_name('l').unwrap().get_values().unwrap();
    /// for e in list
    /// {
    ///     println!("Value: {}", e);
    /// }
    ///```

    pub fn get_values(&self) -> Result<&Vec<String>, &'static str> {
        if let ArgType::ValueList = self.arg_type {
            if let Some(result) = &self.arg_result {
                if let ArgResult::ValueList(ref list) = result {
                    return Ok(list);
                } else {
                    return Err("Wrong type of result. Something really bad happened");
                }
            } else {
                return Err("No result specified");
            }
        } else {
            return Err("This argument is not an value list");
        }
    }

    ///
    /// Method allowing to simplify reading values of a flag type argument.
    ///
    ///# Examples
    ///```
    /// use trivial_argument_parser::*;
    /// let mut args_list = ArgumentList::new();
    /// args_list.append_arg(Argument::new(Some('d'), None, ArgType::Flag).unwrap());
    /// args_list.parse_args(args_to_string_vector(std::env::args())).unwrap();
    /// if(args_list.search_by_short_name('d').unwrap().get_flag().unwrap())
    /// {
    ///     println!("Flag was set");
    /// }
    ///```

    pub fn get_flag(&self) -> Result<bool, &'static str> {
        if let ArgType::Flag = self.arg_type {
            return Ok(if let Some(_) = self.arg_result {
                true
            } else {
                false
            });
        } else {
            return Err("Argument is not an flag type");
        }
    }

    pub fn add_value(
        &mut self,
        input_iter: &mut Peekable<&mut std::slice::Iter<'_, String>>,
    ) -> Result<(), String> {
        match self.arg_type {
            ArgType::Flag => {
                match self.arg_result {
                    Some(_) => return Err(String::from("Flag already set")),
                    _ => (),
                }
                self.arg_result = Some(ArgResult::Flag);
            }
            ArgType::Value => {
                match self.arg_result {
                    Some(_) => return Err(String::from("Value already assigned")),
                    _ => (),
                }
                match input_iter.next() {
                    Some(word) => self.arg_result = Some(ArgResult::Value(String::from(word))),
                    None => return Err(String::from("Expected value")),
                }
            }
            ArgType::ValueList => {
                let mut new_result = false;
                match self.arg_result {
                    Some(_) => (),
                    None => new_result = true,
                }

                if new_result {
                    self.arg_result = Some(ArgResult::ValueList(Vec::new()));
                }

                match input_iter.next() {
                    Some(word) => match self.arg_result.as_mut().expect("as mut") {
                        ArgResult::ValueList(ref mut values) => values.push(String::from(word)),
                        _ => return Err(String::from("WTF")),
                    },
                    None => return Err(String::from("Expected value")),
                }
            }
        }

        Ok(())
    }

    pub fn short(&self) -> &Option<char> {
        &self.short
    }

    pub fn long(&self) -> &Option<String> {
        &self.long
    }

    pub fn arg_type(&self) -> &ArgType {
        &self.arg_type
    }
}

#[cfg(test)]
mod test {
    use crate::argument::legacy_argument::{ArgType, Argument};

    #[test]
    fn new_works() {
        assert!(Argument::new(Option::None, Option::Some("parameter"), ArgType::Flag).is_ok());
        assert!(Argument::new(Option::Some('x'), Option::None, ArgType::Flag).is_ok());
        assert!(Argument::new(Option::Some('x'), Option::Some("parameter"), ArgType::Flag).is_ok());
    }

    #[test]
    fn new_fails() {
        assert!(Argument::new(Option::None, Option::None, ArgType::Flag).is_err())
    }
}
