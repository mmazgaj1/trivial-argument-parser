[![crates.io](https://img.shields.io/crates/v/trivial_argument_parser.svg)](https://crates.io/crates/trivial-argument-parser)

# trivial-argument-parser
This crate was created as a part of a small private project. It started as a small library with the purpose of being a testing ground for the author. Since then nothing really changed because no valuable improvements were introduced. Recently I came back to this crate with introduction of version 0.3.0 where changes drastically shifted how it will be developed. Hopefully I will spend more time on this project so it won't look like an abandoned pile of mess.
## Usage
Simple argument parser written in Rust. It provides simple way of defining and parsing CLI arguments. Example application that uses this library in version 0.3.0.

```rust
use trivial_argument_parser::{
    args_to_string_vector,
    argument::{parsable_argument::ParsableValueArgument, ArgumentIdentification},
    ArgumentList,
};

fn main() {
    let mut args_list = ArgumentList::new();
    let mut argument_int = ParsableValueArgument::new_integer(
            ArgumentIdentification::Short('n')
        );
    let mut argument_str =
        ParsableValueArgument::new_string(
            ArgumentIdentification::Long(String::from("path"))
        );
    args_list.register_parsable(&mut argument_int);
    args_list.register_parsable(&mut argument_str);
    args_list
        .parse_args(args_to_string_vector(std::env::args()))
        .unwrap();
    println!("n - {}", argument_int.first_value().unwrap());
    println!("path - {}", argument_str.first_value().unwrap());
}
```
Running this code with arguments:
```sh
cargo run -- -n 131 --path abc
> n - 131
> path - abc
```

## Defining own argument handlers

You can define your own handlers by using associated function ParsableValueArgument::new. You need to specify how argument will handle values by going over input iterator (it can take one or more values by calling next() or it can be used to set a flag). Input iterator is peekable and can be used for more complex control. Example of defined argument handler - predefined integer argument handler:

``` Rust
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
```

## Future of this library
Even though the development of this crate proceeds slowly, there are some plans of adding more functionalities. The biggest target is introduction of macros to define arguments from structures. Apart from that there are parts of code that could be improved. Legacy API is considered to be useless at this point so I aim to get rid of it.