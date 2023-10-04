pub fn cecho(inputs: Vec<String>) -> Result<String, &'static str> {
    if inputs.len() < 2 {
        Err("The minimum number of arguments is 2. The first argument is the format. If no formatting is necessary, use an empty string.")
    } else {
        let mut result = inputs[1].to_string();
        inputs.iter().skip(2).for_each(|s| result.push_str(s));
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::cecho;

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
}
