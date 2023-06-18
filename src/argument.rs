pub enum ArgumentIdentification {
    Short(char),
    Long(String),
    Both(char, String),
}

pub struct ParsableValueArgument<V> {
    identification: ArgumentIdentification,
    validator: Box<dyn Fn(&str) -> Option<String>>,
    handler: Box<dyn Fn(&str, &mut Vec<V>) -> Result<V, String>>,
    values: Vec<V>,
}

impl<V> ParsableValueArgument<V> {
    pub fn new<C, F>(
        identification: ArgumentIdentification,
        validator: F,
        handler: C,
    ) -> ParsableValueArgument<V>
    where
        C: Fn(&str, &mut Vec<V>) -> Result<V, String> + 'static,
        F: Fn(&str) -> Option<String> + 'static,
    {
        ParsableValueArgument::<V> {
            identification,
            validator: Box::new(validator),
            handler: Box::new(handler),
            values: Vec::new(),
        }
    }

    pub fn new_integer(identification: ArgumentIdentification) -> ParsableValueArgument<i64> {
        let validator = |v: &str| {
            for c in v.chars() {
                if !c.is_digit(10) {
                    return Option::Some(format!("Input is not a number"));
                }
            }
            Option::None
        };
        let handler = |val: &str, _values: &mut Vec<i64>| match val.parse() {
            Ok(v) => Result::Ok(v),
            Err(err) => Result::Err(format!("{}", err)),
        };
        ParsableValueArgument::new(identification, validator, handler)
    }

    fn validate(&self, value: &str) -> Option<String> {
        (self.validator)(value)
    }

    pub fn handle(&mut self, value: &str) -> Result<V, String> {
        if let Option::Some(err) = self.validate(value) {
            return Result::Err(err);
        }
        (self.handler)(value, &mut self.values)
    }

    pub fn is_by_short(&self, name: char) -> bool {
        if let ArgumentIdentification::Short(c) = self.identification {
            return c == name;
        }
        if let ArgumentIdentification::Both(c, _) = &self.identification {
            return c == &name;
        }
        false
    }

    pub fn is_by_long(&self, name: &str) -> bool {
        if let ArgumentIdentification::Long(s) = &self.identification {
            return s == name;
        }
        if let ArgumentIdentification::Both(_, s) = &self.identification {
            return s == name;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::ParsableValueArgument;

    #[test]
    fn new_parsable_value_argument_works() {
        let _arg = ParsableValueArgument::<i64>::new(
            super::ArgumentIdentification::Short('x'),
            |_| Option::None,
            |_, _| Result::Ok(2),
        );
    }

    #[test]
    fn is_by_short_works() {
        let arg = ParsableValueArgument::<i64>::new(
            super::ArgumentIdentification::Short('x'),
            |_| Option::None,
            |_, _| Result::Ok(2),
        );
        assert!(arg.is_by_short('x'));
        assert!(!arg.is_by_short('c'));
    }

    #[test]
    fn is_by_long_works() {
        let arg = ParsableValueArgument::<i64>::new(
            super::ArgumentIdentification::Long(String::from("path")),
            |_| Option::None,
            |_, _| Result::Ok(2),
        );
        assert!(arg.is_by_long("path"));
        assert!(!arg.is_by_long("directory"));
    }

    #[test]
    fn basic_integer_argument_works() {
        let mut arg =
            ParsableValueArgument::<i64>::new_integer(super::ArgumentIdentification::Short('i'));
        assert!(arg.handle("123").is_ok());
        assert_eq!(arg.values.get(0).unwrap(), &123);
    }
}
