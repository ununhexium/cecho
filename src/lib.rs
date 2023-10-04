use std::str::Chars;
use itertools;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Captures, Match, Regex};
use crate::Part::{Literal, Specification};
use colored::Colorize;
use crate::Color::Byte;
use crate::Text::Positional;

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
            Ok(specs) => {
                let mut position = 0;
                let result = specs.iter().map(|s|
                    match s {
                        Literal(l) => { l.white() }
                        Specification { selector, color } => {
                            let text = match selector {
                                Text::Indexed(i) => {
                                    &inputs[*i]
                                }
                                Text::Positional => {
                                    position += 1;
                                    &inputs[position]
                                }
                            };

                            match color {
                                Some(ColorPair { fg, bg }) => {
                                    match fg {
                                        Some(Byte(1)) => text.red(),
                                        _ => { todo!() }
                                    }
                                }
                                _ => { text.white() }
                            }
                        }
                    }
                ).join("");
                Ok(result)
            }
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Part {
    Literal(String),
    Specification {
        selector: Text,
        color: Option<ColorPair>,
    },
}

impl Part {
    pub const fn new(selector: Text) -> Self {
        Specification { selector, color: None }
    }
    pub const fn indexed(index: usize) -> Self {
        Specification { selector: Text::Indexed(index), color: None }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Text {
    Positional,
    Indexed(usize),
}

#[derive(PartialEq)]
#[derive(Debug)]
struct ColorPair {
    fg: Option<Color>,
    bg: Option<Color>,
}

impl ColorPair {
    pub const fn new(foreground: Color, background: Color) -> Self {
        ColorPair { fg: Some(foreground), bg: Some(background) }
    }
    pub const fn new_fg(foreground: Color) -> Self {
        ColorPair { fg: Some(foreground), bg: None }
    }
    pub const fn new_bg(foreground: Color, background: Color) -> Self {
        ColorPair { fg: None, bg: Some(background) }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Color {
    Byte(u8) // ANSI color set
}

impl Color {
    pub const fn black() -> Self {
        Color::Byte(0)
    }
    pub const fn red() -> Self {
        Color::Byte(1)
    }
    pub const fn green() -> Self {
        Color::Byte(2)
    }
    pub const fn yellow() -> Self {
        Color::Byte(3)
    }
    pub const fn blue() -> Self {
        Color::Byte(4)
    }
    pub const fn magenta() -> Self {
        Color::Byte(5)
    }
    pub const fn cyan() -> Self {
        Color::Byte(6)
    }
    pub const fn white() -> Self {
        Color::Byte(7)
    }
}

fn parse_format(format: &String) -> Result<Vec<Part>, String> {
    return parse_in_default_mode(&mut format.chars());
}

fn parse_in_default_mode<'a, 'b>(chars: &'a mut Chars<'a>) -> Result<Vec<Part>, String> {
    let mut specs: Vec<Part> = Vec::new();
    let mut escaped = false;
    let mut so_far = String::new();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                if escaped {
                    so_far.push(c);
                } else {
                    specs.push(Literal(so_far.to_string()));
                    so_far = String::new();
                    let next = parse_in_spec_mode(chars);
                    match next {
                        Err(e) => return Err(e),
                        Ok(it) => specs.push(it),
                    }
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
        specs.push(Literal(so_far.to_string()));
    }

    Ok(specs)
}

// TODO: extend regex to stop and the next specifiers
lazy_static! {
    static ref index_regex : Regex = Regex::new("(?<index>[0-9]+)[#%!?]?").unwrap();
}

fn parse_in_spec_mode<'a, 'b>(chars: &mut Chars) -> Result<Part, String> {
    let mut so_far = String::new();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                return Err("Can't nest specifiers".to_string());
            }
            '}' => {
                return match so_far.as_ref() {
                    "" => {
                        Ok(Specification { selector: Text::Positional, color: None })
                    }
                    _ => {
                        let color_matcher = color_regex.captures(so_far.as_str());
                        let color_spec: Option<ColorPair> = parse_color(color_matcher);

                        let index_match = index_regex.captures(so_far.as_str());
                        let index_spec: Option<Text> = match index_match {
                            None => None,
                            Some(index) => {
                                match index.name("index") {
                                    None => { todo!() }
                                    Some(s) => {
                                        Some(Text::Indexed(s.as_str().parse::<i32>().unwrap() as usize))
                                    }
                                }
                            }
                        };

                        Ok(Specification { selector: index_spec.unwrap_or_else(|| Positional), color: color_spec })
                    }
                };
            }
            _ => {
                so_far.push(c);
            }
        }
    }

    Err("The specifiers are imbalanced: missing }".to_string())
}

lazy_static! {
    static ref color_regex : Regex = Regex::new(".*#(?<value>.+)[#%!?]?").unwrap();
}

fn parse_color(color_spec: Option<Captures>) -> Option<ColorPair> {
    match color_spec {
        None => None,
        Some(color) => {
            match color.name("value") {
                Some(s) => {
                    match s.as_str() {
                        "r" => { Some(ColorPair::new_fg(Color::red())) }
                        &_ => { todo!("{}", s.as_str()) }
                    }
                }
                _ => { todo!() }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cecho, parse_in_spec_mode, parse_in_default_mode, Text, ColorPair, Color, Part};
    use crate::Part::{Literal, Specification};

    macro_rules! vecs {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

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

    #[test]
    fn parse_a_string_that_contains_no_spec_in_default_mode() {
        let specs = parse_in_default_mode(&mut "Hello, format!".to_string().chars());
        let ok = specs.ok().unwrap();
        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0], Literal("Hello, format!".to_string()));
    }

    #[test]
    fn parse_a_string_that_contains_no_spec_but_special_chars_in_default_mode() {
        let specs = parse_in_default_mode(&mut r#"Look at those dirty chars: \{ \\ \}"#.to_string().chars());
        let ok = specs.ok().unwrap();
        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0], Literal(r#"Look at those dirty chars: { \ }"#.to_string()));
    }

    #[test]
    fn parse_a_string_that_contains_1_spec_in_default_mode() {
        let specs = parse_in_default_mode(&mut "Spec={}".to_string().chars());
        let ok = specs.ok().unwrap();
        assert_eq!(ok.len(), 2);
        assert_eq!(ok[0], Literal("Spec=".to_string()));
        assert_eq!(ok[1], Specification { selector: Text::Positional, color: None });
    }

    #[test]
    fn parse_a_nested_format() {
        let specs = parse_in_default_mode(&mut "Whatever {{}".to_string().chars());
        let err = specs.err().unwrap();
        // TODO: improvement: tell the char that caused the issue
        assert_eq!(err, "Can't nest specifiers".to_string());
    }

    #[test]
    fn parse_an_imbalanced_format() {
        let specs = parse_in_default_mode(&mut "Imbalanced {".to_string().chars());
        let err = specs.err().unwrap();
        // TODO: improvement: tell the char that caused the issue
        assert_eq!(err, "The specifiers are imbalanced: missing }".to_string());
    }

    // TODO: warn about extra arguments? pedantic mode?

    #[test]
    fn parse_a_single_digit_index_spec() {
        let specs = parse_in_spec_mode(&mut "8}".to_string().chars());
        let ok = specs.ok().unwrap();
        assert_eq!(ok, Part::indexed(8));
    }

    #[test]
    fn parse_a_large_index_spec() {
        let specs = parse_in_spec_mode(&mut "116}".to_string().chars());
        let ok = specs.ok().unwrap();
        assert_eq!(ok, Part::indexed(116));
    }

    // TODO: spec special chars escape \% \# \\ \? \! ...
    #[test]
    fn parse_red_specs() {
        let specs = parse_in_spec_mode(&mut "#r}".to_string().chars());
        let ok = specs.ok().unwrap();
        assert_eq!(ok, Specification { selector: Text::Positional, color: Some(ColorPair::new_fg(Color::red())) });
    }
}
