use super::ArgumentIdentification;
use std::iter::Peekable;
/**
 * Structure which defines how given argument should be handled. Allows for automatic parsing and validation.
 * Mutable borrow to parsable argument definition has to be registered in ArgumentList. Because of that
 * registered arguments cannot be used while those borrows exist. Either ArgumentList instance has to be dropped
 * or there are no further usages of it. This method of defining arguments is preferred as oposed to using
 * the legacy API.
 */
pub struct ParsableValueArgument<V> {
    identification: ArgumentIdentification,
    handler: Box<
        dyn Fn(&mut Peekable<&mut std::slice::Iter<'_, String>>, &mut Vec<V>) -> Result<V, String>,
    >,
    values: Vec<V>,
}

/// Unifies how parsable arguments are parsed.
pub trait HandleableArgument<'a> {
    /// Handles argument. Gets all needed values from input iterator.
    fn handle(
        &mut self,
        input_iter: &mut Peekable<&mut std::slice::Iter<'_, String>>,
    ) -> Result<(), String>;
    /// Check if this argument is identified by specified short name.
    fn is_by_short(&self, name: char) -> bool;
    /// Check if this argument is identified by specified long name.
    fn is_by_long(&self, name: &str) -> bool;
    /// Get this arguments identification.
    fn identification(&self) -> &ArgumentIdentification;
}

impl<V> ParsableValueArgument<V> {
    pub fn new<C>(identification: ArgumentIdentification, handler: C) -> ParsableValueArgument<V>
    where
        C: Fn(&mut Peekable<&mut std::slice::Iter<'_, String>>, &mut Vec<V>) -> Result<V, String>
            + 'static,
    {
        ParsableValueArgument::<V> {
            identification,
            handler: Box::new(handler),
            values: Vec::new(),
        }
    }

    pub fn first_value(&self) -> Option<&V> {
        self.values().get(0)
    }

    pub fn values(&self) -> &Vec<V> {
        &self.values
    }
}

impl ParsableValueArgument<i64> {
    fn validate_integer(v: &str) -> Option<String> {
        let mut chars_iter = v.chars().peekable();
        if let Some(c) = chars_iter.next() {
            if (c != '-' || chars_iter.peek().is_none()) && !c.is_digit(10) {
                return Option::Some(format!("Input is not a number"));
            }
        }
        for c in chars_iter {
            if !c.is_digit(10) {
                return Option::Some(format!("Input is not a number"));
            }
        }
        Option::None
    }
    /**
     * Default integer type argument value handler. Checks whether value contains only digits or starts with minus sign.
     */
    pub fn new_integer(identification: ArgumentIdentification) -> ParsableValueArgument<i64> {
        let handler = |input_iter: &mut Peekable<&mut std::slice::Iter<'_, String>>,
                       _values: &mut Vec<i64>| {
            if let Option::Some(v) = input_iter.next() {
                let validation = ParsableValueArgument::validate_integer(v);
                if let Option::Some(err) = validation {
                    return Result::Err(err);
                }
                match v.parse() {
                    Result::Ok(v) => Result::Ok(v),
                    Result::Err(err) => Result::Err(format!("{}", err)),
                }
            } else {
                Result::Err(String::from("No remaining input values."))
            }
        };
        ParsableValueArgument::new(identification, handler)
    }
}

impl ParsableValueArgument<String> {
    /**
     * Default string type argument value handler.
     */
    pub fn new_string(identification: ArgumentIdentification) -> ParsableValueArgument<String> {
        let handler = |input_iter: &mut Peekable<&mut std::slice::Iter<'_, String>>,
                       _values: &mut Vec<String>| {
            if let Some(v) = input_iter.next() {
                Result::Ok(String::from(v))
            } else {
                Result::Err(String::from("No remaining input values."))
            }
        };
        ParsableValueArgument::new(identification, handler)
    }
}

impl<'a, V> HandleableArgument<'a> for ParsableValueArgument<V> {
    fn handle(
        &mut self,
        input_iter: &mut Peekable<&mut std::slice::Iter<'_, String>>,
    ) -> Result<(), String> {
        let result = (self.handler)(input_iter, &mut self.values)?;
        self.values.push(result);
        Result::Ok(())
    }

    fn is_by_short(&self, name: char) -> bool {
        self.identification().is_by_short(name)
    }

    fn is_by_long(&self, name: &str) -> bool {
        self.identification().is_by_long(name)
    }

    fn identification(&self) -> &ArgumentIdentification {
        &self.identification
    }
}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;

    use super::{HandleableArgument, ParsableValueArgument};

    #[test]
    fn new_parsable_value_argument_works() {
        let _arg =
            ParsableValueArgument::<i64>::new(super::ArgumentIdentification::Short('x'), |_, _| {
                Result::Ok(2)
            });
    }

    #[test]
    fn is_by_short_works() {
        let arg =
            ParsableValueArgument::<i64>::new(super::ArgumentIdentification::Short('x'), |_, _| {
                Result::Ok(2)
            });
        assert!(arg.is_by_short('x'));
        assert!(!arg.is_by_short('c'));
    }

    #[test]
    fn is_by_long_works() {
        let arg = ParsableValueArgument::<i64>::new(
            super::ArgumentIdentification::Long(String::from("path")),
            |_, _| Result::Ok(2),
        );
        assert!(arg.is_by_long("path"));
        assert!(!arg.is_by_long("directory"));
    }

    #[test]
    fn basic_integer_argument_works() {
        let mut arg =
            ParsableValueArgument::<i64>::new_integer(super::ArgumentIdentification::Short('i'));
        assert!(arg
            .handle(&mut vec![String::from("123")].iter().borrow_mut().peekable())
            .is_ok());
        assert_eq!(arg.values.get(0).unwrap(), &123);
        assert!(arg
            .handle(&mut vec![String::from("333")].iter().borrow_mut().peekable())
            .is_ok());
        assert_eq!(2, arg.values.len());
        assert_eq!(arg.values.get(0).unwrap(), &123);
        assert_eq!(arg.values.get(1).unwrap(), &333);
        assert!(arg
            .handle(&mut vec![String::from("-333")].iter().borrow_mut().peekable())
            .is_ok());
    }

    #[test]
    fn basic_integer_argument_handler_fails_invalid_number() {
        let mut arg =
            ParsableValueArgument::<i64>::new_integer(super::ArgumentIdentification::Short('i'));
        assert!(arg
            .handle(&mut vec![String::from("-")].iter().borrow_mut().peekable())
            .is_err());
        assert!(arg
            .handle(&mut vec![String::from("12a")].iter().borrow_mut().peekable())
            .is_err());
        assert!(arg
            .handle(&mut vec![String::from("123.12")].iter().borrow_mut().peekable())
            .is_err());
    }

    #[test]
    fn first_value_works() {
        let mut arg = ParsableValueArgument::new_integer(super::ArgumentIdentification::Short('i'));
        assert!(arg.first_value().is_none());
        assert!(arg
            .handle(&mut vec![String::from("123")].iter().borrow_mut().peekable())
            .is_ok());
        assert_eq!(arg.first_value().unwrap(), &123);
    }
}
