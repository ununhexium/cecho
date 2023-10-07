use crate::parser::parse_format;
use crate::writer::spec_to_ansi;

pub fn cecho(inputs: Vec<String>) -> Result<String, String> {
    // TODO matcher
    if inputs.len() < 2 {
        Err("The minimum number of arguments is 2. The first argument is the format. If no formatting is necessary, use an empty string.".to_string())
    } else if inputs[0].is_empty() {
        let mut result = inputs[1].to_string();
        inputs.iter().skip(2).for_each(|s| result.push_str(s));
        Ok(result)
    } else {
        let parsed = parse_format(&inputs[0]);

        match parsed {
            Err(m) => Err(m.to_string()),
            Ok(specs) => { spec_to_ansi(&inputs, specs) }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::vecs;
    use crate::cecho::cecho;

    #[test]
    fn check_that_there_is_at_least_2_arguments() {
        let i = vecs!("");
        let actual = cecho(i);

        assert_eq!(
            actual.err(),
            Some("The minimum number of arguments is 2. The first argument is the format. If no formatting is necessary, use an empty string.".to_string())
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
    // {garbage value}
    // TODO refuse to mix positional, indexed and named, only 1 of each
}
