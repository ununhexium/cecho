use std::str::Chars;
use colorz::Colorize;
use itertools;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use crate::Part::{Literal, Specification};
use crate::Color::Byte;
use crate::Text::{Indexed, Positional};

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
                        Literal(l) => { l.to_string() }
                        Specification { text: selector, color } => {
                            let surroundings = match color {
                                Some(ColorPair { fg, bg: _ }) => {
                                    match fg {
                                        Some(Byte(b)) => if b <= &b'\x08' {
                                            (format!("\x1b[0;{}m", 30 + b), "\x1b[0m")
                                        } else {
                                            (format!("\x1b[0;{}m", 82 + b), "\x1b[0m")
                                        }
                                        _ => { ("".to_string(), "") }
                                    }
                                }
                                _ => { ("".to_string(), "") }
                            };

                            let full = match selector {
                                Indexed(i) => {
                                    surroundings.0 + &inputs[*i] + surroundings.1
                                }
                                Positional => {
                                    position += 1;
                                    surroundings.0 + &inputs[position] + surroundings.1
                                }
                            };

                            full.to_string()
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
        text: Text,
        color: Option<ColorPair>,
    },
}

impl Part {
    pub const fn indexed(index: usize) -> Self {
        Specification { text: Text::Indexed(index), color: None }
    }
    pub const fn positional_color(color: ColorPair) -> Self {
        Specification { text: Positional, color: Some(color) }
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
    pub const fn new_fg(foreground: Color) -> Self {
        ColorPair { fg: Some(foreground), bg: None }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Color {
    Byte(u8) // ANSI color set
}

impl Color {
    // bit notation: 0b4321
    // bit 1 is red, bit 2 is green, bit 3 is blue, 4 is bright
    pub const fn black() -> Self { Byte(0b000) }
    pub const fn red() -> Self { Byte(0b001) }
    pub const fn green() -> Self { Byte(0b010) }
    pub const fn yellow() -> Self { Byte(0b011) }
    pub const fn blue() -> Self { Byte(0b100) }
    pub const fn magenta() -> Self { Byte(0b101) }
    pub const fn cyan() -> Self { Byte(0b110) }
    pub const fn white() -> Self { Byte(0b111) }

    // because physics, lol XD
    pub const fn bright_black() -> Self { Byte(0b1000) }
    pub const fn bright_red() -> Self { Byte(0b1001) }
    pub const fn bright_green() -> Self { Byte(0b1010) }
    pub const fn bright_yellow() -> Self { Byte(0b1011) }
    pub const fn bright_blue() -> Self { Byte(0b100) }
    pub const fn bright_magenta() -> Self { Byte(0101) }
    pub const fn bright_cyan() -> Self { Byte(0b1111) }
    pub const fn bright_white() -> Self { Byte(0b1111) }
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
    static ref INDEX_REGEX : Regex = Regex::new("^(?<index>[0-9]+)[#%!?]?").unwrap();
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
                        Ok(Specification { text: Positional, color: None })
                    }
                    _ => {
                        let color_matcher = COLOR_REGEX.captures(so_far.as_str());
                        let color_spec: Option<ColorPair> = parse_color(color_matcher);

                        let index_match = INDEX_REGEX.captures(so_far.as_str());
                        let index_spec: Option<Text> = match index_match {
                            None => None,
                            Some(index) => {
                                match index.name("index") {
                                    None => { todo!() }
                                    Some(s) => {
                                        Some(Indexed(s.as_str().parse::<i32>().unwrap() as usize))
                                    }
                                }
                            }
                        };

                        Ok(Specification { text: index_spec.unwrap_or_else(|| Positional), color: color_spec })
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
    static ref COLOR_REGEX : Regex = Regex::new(".*#(?<value>.+)[#%!?]?").unwrap();
}

fn parse_color(color_spec: Option<Captures>) -> Option<ColorPair> {
    match color_spec {
        None => None,
        Some(color) => {
            match color.name("value") {
                Some(s) => {
                    match s.as_str() {
                        "0" | "k" | "black" => { Some(ColorPair::new_fg(Color::black())) }
                        "1" | "r" | "red" => { Some(ColorPair::new_fg(Color::red())) }
                        "2" | "g" | "green" => { Some(ColorPair::new_fg(Color::green())) }
                        "3" | "y" | "yellow" => { Some(ColorPair::new_fg(Color::yellow())) }
                        "4" | "b" | "blue" => { Some(ColorPair::new_fg(Color::blue())) }
                        "5" | "m" | "magenta" => { Some(ColorPair::new_fg(Color::magenta())) }
                        "6" | "c" | "cyan" => { Some(ColorPair::new_fg(Color::cyan())) }
                        "7" | "w" | "white" => { Some(ColorPair::new_fg(Color::white())) }

                        "8" | "K" | "BLACK" => { Some(ColorPair::new_fg(Color::bright_black())) }
                        "9" | "R" | "RED" => { Some(ColorPair::new_fg(Color::bright_red())) }
                        "10" | "G" | "GREEN" => { Some(ColorPair::new_fg(Color::bright_green())) }
                        "11" | "Y" | "YELLOW" => { Some(ColorPair::new_fg(Color::bright_yellow())) }
                        "12" | "B" | "BLUE" => { Some(ColorPair::new_fg(Color::bright_blue())) }
                        "13" | "M" | "MAGENTA" => { Some(ColorPair::new_fg(Color::bright_magenta())) }
                        "14" | "C" | "CYAN" => { Some(ColorPair::new_fg(Color::bright_cyan())) }
                        "15" | "W" | "WHITE" => { Some(ColorPair::new_fg(Color::bright_white())) }
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
        assert_eq!(ok[1], Specification { text: Text::Positional, color: None });
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

    fn test_color_spec(spec: &str, color: Color) {
        let specs = parse_in_spec_mode(&mut spec.to_string().chars());
        let ok = specs.ok().unwrap();
        assert_eq!(ok, Part::positional_color(ColorPair::new_fg(color)));
    }

    // TODO: spec special chars escape \% \# \\ \? \! ...
    #[test]
    fn parse_black_specs() {
        test_color_spec("#0}", Color::black());
        test_color_spec("#k}", Color::black());
        test_color_spec("#black}", Color::black());

        test_color_spec("#8}", Color::bright_black());
        test_color_spec("#K}", Color::bright_black());
        test_color_spec("#BLACK}", Color::bright_black());
    }

    #[test]
    fn parse_red_specs() {
        test_color_spec("#1}", Color::red());
        test_color_spec("#r}", Color::red());
        test_color_spec("#red}", Color::red());

        test_color_spec("#9}", Color::bright_red());
        test_color_spec("#R}", Color::bright_red());
        test_color_spec("#RED}", Color::bright_red());
    }

    #[test]
    fn parse_green_specs() {
        test_color_spec("#2}", Color::green());
        test_color_spec("#g}", Color::green());
        test_color_spec("#green}", Color::green());

        test_color_spec("#10}", Color::bright_green());
        test_color_spec("#G}", Color::bright_green());
        test_color_spec("#GREEN}", Color::bright_green());
    }

    #[test]
    fn parse_yellow_specs() {
        test_color_spec("#3}", Color::yellow());
        test_color_spec("#y}", Color::yellow());
        test_color_spec("#yellow}", Color::yellow());

        test_color_spec("#11}", Color::bright_yellow());
        test_color_spec("#Y}", Color::bright_yellow());
        test_color_spec("#YELLOW}", Color::bright_yellow());
    }

    #[test]
    fn parse_blue_specs() {
        test_color_spec("#4}", Color::blue());
        test_color_spec("#b}", Color::blue());
        test_color_spec("#blue}", Color::blue());

        test_color_spec("#12}", Color::bright_blue());
        test_color_spec("#B}", Color::bright_blue());
        test_color_spec("#BLUE}", Color::bright_blue());
    }

    #[test]
    fn parse_magenta_specs() {
        test_color_spec("#5}", Color::magenta());
        test_color_spec("#m}", Color::magenta());
        test_color_spec("#magenta}", Color::magenta());

        test_color_spec("#13}", Color::bright_magenta());
        test_color_spec("#M}", Color::bright_magenta());
        test_color_spec("#MAGENTA}", Color::bright_magenta());
    }

    #[test]
    fn parse_cyan_specs() {
        test_color_spec("#6}", Color::cyan());
        test_color_spec("#c}", Color::cyan());
        test_color_spec("#cyan}", Color::cyan());

        test_color_spec("#14}", Color::bright_cyan());
        test_color_spec("#C}", Color::bright_cyan());
        test_color_spec("#CYAN}", Color::bright_cyan());
    }

    #[test]
    fn parse_white_specs() {
        test_color_spec("#7}", Color::white());
        test_color_spec("#w}", Color::white());
        test_color_spec("#white}", Color::white());

        test_color_spec("#15}", Color::bright_white());
        test_color_spec("#W}", Color::bright_white());
        test_color_spec("#WHITE}", Color::bright_white());
    }
}
