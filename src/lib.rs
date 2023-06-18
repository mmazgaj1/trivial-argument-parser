pub mod builder;
pub mod parsable_argument;
use std::env;

use parsable_argument::HandleableArgument;

/**
Enum allowing to choose the type of argument.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgType {
    Flag,
    Value,
    ValueList,
}

pub trait ArgumentValueValidator {
    fn validate(&self, value: &str) -> bool;
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
        input_iter: &mut std::slice::Iter<'_, String>,
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
}

///
/// Acumulates arguments into list which then can be fed to parse.
///
/// # Examples
/// ```
/// use trivial_argument_parser::*;
/// let mut args_list = ArgumentList::new();
/// args_list.append_arg(Argument::new(Some('d'), None, ArgType::Flag).unwrap());
/// args_list.append_arg(Argument::new(Some('p'), None, ArgType::Value).unwrap());
/// args_list.append_arg(Argument::new(Some('l'), Some("an-list"), ArgType::ValueList).unwrap());
/// ```
pub struct ArgumentList<'a> {
    pub dangling_values: Vec<String>,
    pub arguments: Vec<Argument>,
    pub parsable_arguments: Vec<&'a mut (dyn HandleableArgument<'a> + 'a)>,
}

impl<'a> ArgumentList<'a> {
    pub fn arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }
    /**
    Create ArgumentList with empty vector of arguments.
    */
    pub fn new() -> ArgumentList<'a> {
        ArgumentList {
            dangling_values: Vec::new(),
            arguments: Vec::new(),
            parsable_arguments: Vec::new(),
        }
    }

    /**
    Append argument to the end of the list.
    */
    pub fn append_arg(&mut self, argument: Argument) {
        self.arguments.push(argument);
    }

    /**
    Append dangling values.
    */
    pub fn append_dangling_value(&mut self, value: &str) {
        self.dangling_values.push(String::from(value));
    }

    /**
    Search arguments by short name.
    */
    pub fn search_by_short_name(&mut self, name: char) -> Option<&mut Argument> {
        for x in &mut self.arguments {
            match x.short {
                Some(symbol) => {
                    if symbol == name {
                        return Some(x);
                    }
                }
                None => (),
            };
        }
        Option::None
    }

    // pub fn search_parsable_by_short_name(
    //     &mut self,
    //     name: char,
    // ) -> Option<&mut &'a mut dyn HandleableArgument> {
    //     for x in self.parsable_arguments.as_mut_slice() {
    //         if x.is_by_short(name) {
    //             return Option::Some(x);
    //         }
    //     }
    //     Option::None
    // }

    fn handle_parsable_short_name(
        &mut self,
        name: char,
        input_iter: &mut std::slice::Iter<'_, String>,
    ) -> Result<bool, String> {
        for x in &mut self.parsable_arguments {
            if x.is_by_short(name) {
                x.handle(input_iter)?;
                return Result::Ok(true);
            }
        }
        return Result::Ok(false);
    }

    fn handle_parsable_long_name(
        &mut self,
        name: &str,
        input_iter: &mut std::slice::Iter<'_, String>,
    ) -> Result<bool, String> {
        for x in &mut self.parsable_arguments {
            if x.is_by_long(name) {
                x.handle(input_iter)?;
                return Result::Ok(true);
            }
        }
        return Result::Ok(false);
    }

    /**
    Search arguments by long name.
    */
    pub fn search_by_long_name(&mut self, name: &str) -> Option<&mut Argument> {
        for x in &mut self.arguments {
            match x.long {
                Some(ref long_name) => {
                    if long_name == name {
                        return Option::Some(x);
                    }
                }
                None => (),
            }
        }
        Option::None
    }

    /// Returns vector of all generated dangling values (values not attached to any argument)
    pub fn get_dangling_values(&self) -> &Vec<String> {
        &self.dangling_values
    }

    /// Function that does all the parsing. You need to feed user input as an argument.
    ///
    /// # Examples
    /// ```
    /// use trivial_argument_parser::*;
    ///
    /// let mut args_list = ArgumentList::new();
    /// args_list.append_arg(Argument::new(Some('d'), None, ArgType::Flag).unwrap());
    /// args_list.append_arg(Argument::new(Some('p'), None, ArgType::Value).unwrap());
    /// args_list.append_arg(Argument::new(Some('l'), Some("an-list"), ArgType::ValueList).unwrap());
    /// args_list.parse_args(args_to_string_vector(std::env::args())).unwrap();
    /// ```
    pub fn parse_args(&mut self, input: Vec<String>) -> Result<(), String> {
        let mut input_iter = input.iter();
        while let Some(word) = input_iter.next() {
            // Check if word is a short argument, long argument or dangling value
            let word_length = word.chars().count();
            if word_length == 2 {
                if word.chars().nth(0).expect("first letter") == '-'
                    && word
                        .chars()
                        .nth(1)
                        .expect(&format!("{}", word_length))
                        .is_alphabetic()
                {
                    // Add value to argument identified by short name
                    match self.search_by_short_name(word.chars().nth(1).unwrap()) {
                        Some(argument) => {
                            argument.add_value(&mut input_iter)?;
                        }
                        None => {
                            if !self.handle_parsable_short_name(
                                word.chars().nth(1).unwrap(),
                                &mut input_iter,
                            )? {
                                return Err(format!(
                                    "Could not find argument identified by {}.",
                                    word
                                ));
                            }
                        }
                    };
                } else {
                    // Add as dangling value
                    self.append_dangling_value(word);
                }
            } else if word_length > 2 {
                if word.chars().nth(0).unwrap() == '-'
                    && word.chars().nth(1).unwrap() == '-'
                    && word.chars().nth(2).unwrap().is_alphabetic()
                {
                    // Add value to argument identified by long name
                    match self.search_by_long_name(&word[2..word.len()]) {
                        Some(argument) => {
                            argument.add_value(&mut input_iter)?;
                        }
                        Option::None => {
                            if !self
                                .handle_parsable_long_name(&word[2..word.len()], &mut input_iter)?
                            {
                                return Err(format!(
                                    "Could not find argument identified by {}.",
                                    word
                                ));
                            }
                        }
                    };
                } else {
                    // Add as dangling value
                    self.append_dangling_value(word);
                }
            } else {
                // Add as dangling value
                self.append_dangling_value(word);
            }
        }

        // return arguments list with filled parsed values
        Ok(())
    }

    /**
     * Registers argument reference to be used while parsing.
     */
    pub fn register_parsable(&mut self, arg: &'a mut impl HandleableArgument<'a>) {
        self.parsable_arguments.push(arg);
    }
}

/**
Helper function to transform arguments given by user from Args to vector of String.
*/
pub fn args_to_string_vector(args: env::Args) -> Vec<String> {
    let mut arguments = Vec::new();

    for x in args {
        arguments.push(String::from(x));
    }
    arguments
}

#[cfg(test)]
mod tests {
    use crate::parsable_argument::ParsableValueArgument;

    use super::*;

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

    #[test]
    fn parse_works() {
        let args = vec![
            String::from("-d"),
            String::from("-p"),
            String::from("/file"),
            String::from("--an-list"),
            String::from("Marcin"),
            String::from("-l"),
            String::from("Mazgaj"),
        ];

        let mut args_list = ArgumentList::new();
        args_list.append_arg(Argument::new(Some('d'), None, ArgType::Flag).expect("append 1"));
        args_list.append_arg(Argument::new(Some('p'), None, ArgType::Value).expect("append 2"));
        args_list.append_arg(
            Argument::new(Some('l'), Some("an-list"), ArgType::ValueList).expect("append 3"),
        );
        args_list.parse_args(args).unwrap();
        assert_eq!(args_list.arguments()[0].arg_result, Some(ArgResult::Flag));
        assert_eq!(
            args_list.arguments[1].arg_result,
            Some(ArgResult::Value(String::from("/file")))
        );
        assert_eq!(
            args_list.arguments[2].arg_result,
            Some(ArgResult::ValueList(vec![
                String::from("Marcin"),
                String::from("Mazgaj")
            ]))
        );

        assert_eq!(
            args_list
                .search_by_short_name('d')
                .unwrap()
                .get_flag()
                .unwrap(),
            true
        );
        assert_eq!(
            args_list
                .search_by_long_name("an-list")
                .unwrap()
                .get_values()
                .unwrap(),
            &vec![String::from("Marcin"), String::from("Mazgaj")]
        );
    }

    #[test]
    fn get_dangling_values_works() {
        let args = vec![
            String::from("-d"),
            String::from("-p"),
            String::from("/file"),
            String::from("--an-list"),
            String::from("Marcin"),
            String::from("-l"),
            String::from("Mazgaj"),
            String::from("dangling"),
        ];

        let mut args_list = ArgumentList::new();

        args_list.append_arg(Argument::new(Some('d'), None, ArgType::Flag).expect("append 1"));
        args_list.append_arg(Argument::new(Some('p'), None, ArgType::Value).expect("append 2"));
        args_list.append_arg(
            Argument::new(Some('l'), Some("an-list"), ArgType::ValueList).expect("append 3"),
        );

        args_list.parse_args(args).unwrap();

        let dangling = args_list.get_dangling_values();

        assert_eq!("dangling", dangling[0]);
    }

    #[test]
    fn values_with_spaces_work() {
        let args = vec![
            String::from("-n"),
            String::from("Marcin Mazgaj"),
            String::from("--hello"),
            String::from("Hello World!"),
            String::from("--hello"),
            String::from("Witaj Świecie!"),
        ];

        let mut args_list = ArgumentList::new();

        args_list.append_arg(Argument::new(Some('n'), None, ArgType::Value).unwrap());
        args_list.append_arg(Argument::new(None, Some("hello"), ArgType::ValueList).unwrap());

        args_list.parse_args(args).unwrap();

        assert_eq!(
            args_list
                .search_by_short_name('n')
                .unwrap()
                .get_value()
                .unwrap(),
            "Marcin Mazgaj"
        );
        assert_eq!(
            args_list
                .search_by_long_name("hello")
                .unwrap()
                .get_values()
                .unwrap(),
            &vec![String::from("Hello World!"), String::from("Witaj Świecie!")]
        );
    }

    #[test]
    fn parsable_works() {
        let args = vec![
            String::from("-n"),
            String::from("5"),
            String::from("--hello"),
            String::from("Hello World!"),
            String::from("--hello"),
            String::from("Witaj Świecie!"),
        ];

        let mut args_list = ArgumentList::new();
        let mut argument_int = ParsableValueArgument::new_integer(
            parsable_argument::ArgumentIdentification::Short('n'),
        );
        let mut argument_str = ParsableValueArgument::new_string(
            parsable_argument::ArgumentIdentification::Long(String::from("hello")),
        );
        args_list.register_parsable(&mut argument_int);
        args_list.register_parsable(&mut argument_str);
        args_list
            .parse_args(args)
            .expect("Failed while parsing arguments");
        assert_eq!(argument_int.first_value().unwrap(), &5);
        assert_eq!(argument_str.first_value().unwrap(), "Hello World!");
        assert_eq!(argument_str.values().get(1).unwrap(), "Witaj Świecie!");
    }
}
