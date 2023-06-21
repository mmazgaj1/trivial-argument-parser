pub mod argument;

use std::{borrow::BorrowMut, env, iter::Peekable};

use argument::{legacy_argument::Argument, parsable_argument::HandleableArgument};

///
/// Acumulates arguments into list which then can be fed to parse.
///
/// # Examples
/// ```
/// use trivial_argument_parser::{ArgumentList, argument::legacy_argument::*};
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
    pub fn search_by_short_name(&self, name: char) -> Option<&Argument> {
        for x in &self.arguments {
            match x.short() {
                Some(symbol) => {
                    if symbol == &name {
                        return Some(x);
                    }
                }
                None => (),
            };
        }
        Option::None
    }

    /**
    Search arguments by short name.
    */
    pub fn search_by_short_name_mut(&mut self, name: char) -> Option<&mut Argument> {
        for x in &mut self.arguments {
            match x.short() {
                Some(symbol) => {
                    if symbol == &name {
                        return Some(x);
                    }
                }
                None => (),
            };
        }
        Option::None
    }

    fn handle_parsable_short_name(
        &mut self,
        name: char,
        input_iter: &mut Peekable<&mut std::slice::Iter<'_, String>>,
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
        input_iter: &mut Peekable<&mut std::slice::Iter<'_, String>>,
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
            match x.long() {
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

    /// Function that does all the parsing. You need to feed user input as an argument. Handles both
    /// legacy type arguments and parsable value arguments. When used with mixed type arguments, parsable
    /// arguments cannot be accessed before the last reference to ArgumentList or it being dropped.
    ///
    /// # Examples
    /// ```
    /// use trivial_argument_parser::{ArgumentList, args_to_string_vector, argument::legacy_argument::*};
    ///
    /// let mut args_list = ArgumentList::new();
    /// args_list.append_arg(Argument::new(Some('d'), None, ArgType::Flag).unwrap());
    /// let mut argument_str =ParsableValueArgument::new_string(ArgumentIdentification::Long(String::from("hello")));
    /// args_list.register_parsable(&mut argument_int);
    /// args_list.parse_args(args_to_string_vector(std::env::args())).unwrap();
    /// // First read legacy arguments.
    /// args_list.search_by_short_name('n');
    /// // Then access parsable value arguments since last reference was used.
    /// argument_str.first_value();
    /// ```
    pub fn parse_args(&mut self, input: Vec<String>) -> Result<(), String> {
        let mut iter = input.iter();
        let mut input_iter = iter.borrow_mut().peekable();
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
                    match self.search_by_short_name_mut(word.chars().nth(1).unwrap()) {
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
    use crate::argument::{
        legacy_argument::{ArgResult, ArgType},
        parsable_argument::ParsableValueArgument,
    };

    use super::{argument::ArgumentIdentification, *};

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
                .search_by_short_name_mut('d')
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
                .search_by_short_name_mut('n')
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
    fn parse_with_parsable_arguments_works() {
        let args = vec![
            String::from("-n"),
            String::from("5"),
            String::from("--hello"),
            String::from("Hello World!"),
            String::from("--hello"),
            String::from("Witaj Świecie!"),
        ];

        let mut args_list = ArgumentList::new();
        let mut argument_int =
            ParsableValueArgument::new_integer(ArgumentIdentification::Short('n'));
        let mut argument_str =
            ParsableValueArgument::new_string(ArgumentIdentification::Long(String::from("hello")));
        args_list.register_parsable(&mut argument_int);
        args_list.register_parsable(&mut argument_str);
        args_list
            .parse_args(args)
            .expect("Failed while parsing arguments");
        assert_eq!(argument_int.first_value().unwrap(), &5);
        assert_eq!(argument_str.first_value().unwrap(), "Hello World!");
        assert_eq!(argument_str.values().get(1).unwrap(), "Witaj Świecie!");
    }

    #[test]
    fn parse_drop_parser_works() {
        let args = vec![
            String::from("-n"),
            String::from("5"),
            String::from("--hello"),
            String::from("Hello World!"),
            String::from("--hello"),
            String::from("Witaj Świecie!"),
        ];
        let mut argument_int =
            ParsableValueArgument::new_integer(ArgumentIdentification::Short('n'));
        let mut argument_str =
            ParsableValueArgument::new_string(ArgumentIdentification::Long(String::from("hello")));
        {
            let mut args_list = ArgumentList::new();
            args_list.register_parsable(&mut argument_int);
            args_list.register_parsable(&mut argument_str);
            args_list
                .parse_args(args)
                .expect("Failed while parsing arguments");
        }
        assert_eq!(argument_int.first_value().unwrap(), &5);
        assert_eq!(argument_str.first_value().unwrap(), "Hello World!");
        assert_eq!(argument_str.values().get(1).unwrap(), "Witaj Świecie!");
    }

    #[test]
    fn parse_with_mixed_arguments_works() {
        let args = vec![
            String::from("-n"),
            String::from("5"),
            String::from("--hello"),
            String::from("Hello World!"),
            String::from("--hello"),
            String::from("Witaj Świecie!"),
        ];

        let mut args_list = ArgumentList::new();
        let mut argument_str =
            ParsableValueArgument::new_string(ArgumentIdentification::Long(String::from("hello")));
        args_list.register_parsable(&mut argument_str);
        args_list.append_arg(Argument::new(Some('n'), None, ArgType::Value).unwrap());
        args_list
            .parse_args(args)
            .expect("Failed while parsing arguments");
        assert_eq!(
            args_list
                .search_by_short_name('n')
                .unwrap()
                .get_value()
                .unwrap(),
            "5"
        );
        assert_eq!(argument_str.first_value().unwrap(), "Hello World!");
        assert_eq!(argument_str.values().get(1).unwrap(), "Witaj Świecie!");
    }
}
