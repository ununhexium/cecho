use std::str::Chars;
use itertools;
use itertools::Itertools;

pub fn cecho(inputs: Vec<String>) -> Result<String, &'static str> {
    // TODO matcher
    if inputs.len() < 2 {
        Err("The minimum number of arguments is 2. The first argument is the format. If no formatting is necessary, use an empty string.")
    } else if inputs[0].is_empty() {
        let mut result = inputs[1].to_string();
        inputs.iter().skip(2).for_each(|s| result.push_str(s));
        Ok(result)
    } else {
        let parsed = parse_format(&inputs[0]);
        let mut position = 0;
        let result = parsed.iter().map(|s|
            match s {
                Spec::Litteral(l) => { l }
                Spec::Positional => {
                    position += 1;
                    &inputs[position]
                }
            }
        ).join("");
        Ok(result)
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Spec {
    Litteral(String),
    Positional,
}

fn parse_format(format: &String) -> Vec<Spec> {
    let mut specs: Vec<Spec> = Vec::new();

    specs.extend(
        parse_in_default_mode(&mut format.chars())
    );

    specs
}

fn parse_in_default_mode<'a, 'b>(chars: &'a mut Chars<'a>) -> Vec<Spec> {
    let mut specs: Vec<Spec> = Vec::new();
    let mut escaped = false;
    let mut so_far = String::new();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                if escaped {
                    so_far.push(c);
                } else {
                    specs.push(Spec::Litteral(so_far.to_string()));
                    so_far = String::new();
                    specs.extend(parse_as_spec(chars));
                }
            }
            '\\' => {
                if escaped {
                    so_far.push(c);
                    escaped = false;
                } else {
                    escaped = true;
                }
            }
            _ => {
                so_far.push(c);
            }
        }

        if c != '\\' {
            escaped = false;
        }
    }

    if !so_far.is_empty() {
        specs.push(Spec::Litteral(so_far.to_string()));
    }

    specs
}

fn parse_as_spec<'a, 'b>(chars: &mut Chars) -> Vec<Spec> {
    let mut specs: Vec<Spec> = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                panic!("Nested specifiers are not supported")
            }
            '}' => {
                specs.push(Spec::Positional);
                break;
            }
            _ => {
                todo!()
            }
        }
    }

    specs
}

#[cfg(test)]
mod tests {
    use crate::{cecho, parse_in_default_mode, Spec};

    macro_rules! vecs {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn check_that_there_is_at_least_2_arguments() {
        let i = vecs!("");
        let actual = cecho(i);

        assert_eq!(
            actual.err(),
            Some("The minimum number of arguments is 2. The first argument is the format. If no formatting is necessary, use an empty string.")
        )
    }

    #[test]
    fn when_the_first_string_is_empty_and_there_are_2_arguments_just_return_the_second_argument() {
        let i = vecs!("", "{foo}");
        let actual = cecho(i);
        assert_eq!(actual.ok(), Some("{foo}".to_string()));
    }

    #[test]
    fn when_the_first_string_is_empty_and_there_are_n_arguments_just_return_their_concatenation() {
        let i = vecs!("", "1", ", 2, ", "N...");
        let actual = cecho(i);
        assert_eq!(actual.ok(), Some("1, 2, N...".to_string()));
    }

    #[test]
    fn when_a_format_is_specified_then_use_it_0_spec() {
        let i = vecs!(
            r#"Just raw text, nothing special, no placeholder like \{\}"#,
            "this will be ignored because the format contains no formatting specifier"
        );
        let actual = cecho(i);
        assert_eq!(actual.ok(), Some(r#"Just raw text, nothing special, no placeholder like {}"#.to_string()));
    }

    #[test]
    fn when_a_format_is_specified_then_use_it_2_specs() {
        let i = vecs!("{} and {}", "A", "B");
        let actual = cecho(i);
        assert_eq!(actual.ok(), Some("A and B".to_string()));
    }

    // TODO detect invalid cases:
    // {{}}
    // {garbage value}
    // unbalanced {
    // TODO refuse to mix positional, indexed and named, only 1 of each

    #[test]
    fn parse_a_string_that_contains_no_spec_in_default_mode() {
        let specs = parse_in_default_mode(&mut "Hello, format!".to_string().chars());
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0], Spec::Litteral("Hello, format!".to_string()));
    }

    #[test]
    fn parse_a_string_that_contains_no_spec_but_special_chars_in_default_mode() {
        let specs = parse_in_default_mode(&mut r#"Look at those dirty chars: \{ \\ \}"#.to_string().chars());
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0], Spec::Litteral(r#"Look at those dirty chars: { \ }"#.to_string()));
    }

    #[test]
    fn parse_a_string_that_contains_1_spec_in_default_mode() {
        let specs = parse_in_default_mode(&mut "Spec={}".to_string().chars());
        assert_eq!(specs.len(), 2);
        assert_eq!(specs[0], Spec::Litteral("Spec=".to_string()));
        assert_eq!(specs[1], Spec::Positional);
    }
}
