use std::str::Chars;

use lazy_static::lazy_static;
use regex::{Match, Regex};

use crate::model::{Color, Colors, Part, Text};
use crate::model::Part::{Literal, Specification};
use crate::model::Text::{Indexed, Positional};

pub fn parse_format(format: &String) -> Result<Vec<Part>, String> {
    return parse_format_in_default_mode(&mut format.chars());
}

fn parse_format_in_default_mode<'a, 'b>(chars: &'a mut Chars<'a>) -> Result<Vec<Part>, String> {
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
                    let next = parse_format_in_spec_mode(chars);
                    match next {
                        Err(e) => return Err(e),
                        Ok(it) => specs.push(it),
                    }
                }
            }
            '\\' => {
                if escaped {
                    so_far.push('\\');
                    escaped = false;
                } else {
                    escaped = true;
                }
            }
            _ => {
                if escaped {
                    match c {
                        'a' => {
                            so_far.push('\x07');
                        }
                        'b' => {
                            so_far.push('\x08');
                        }
                        't' => {
                            so_far.push('\x09');
                        }
                        'n' => {
                            so_far.push('\n');
                        }
                        'v' => {
                            so_far.push('\x0b');
                        }
                        'f' => {
                            so_far.push('\x0c');
                        }
                        'r' => {
                            so_far.push('\x0d');
                        }
                        'e' => {
                            so_far.push('\x1b');
                        }
                        '}' => {
                            so_far.push('}');
                        }
                        '\\' => {
                            panic!("This should never happen, \\ is handled above.")
                        }
                        _ => {
                            panic!("Invalid escape sequence: \\{}", c)
                        }
                    }
                } else {
                    so_far.push(c);
                }
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

fn parse_format_in_spec_mode<'a, 'b>(chars: &mut Chars) -> Result<Part, String> {
    let mut so_far = String::new();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                return Err("Can't nest specifiers".to_string());
            }
            '}' => {
                return match so_far.as_ref() {
                    "" => {
                        Ok(Specification { text: Positional, color: Colors::none() })
                    }
                    _ => {
                        parse_spec(so_far.as_str())
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
    static ref PARTS_REGEX : Regex = Regex::new("^(%(?<text>[0-9]+))?(?<color>#.+)?$").unwrap();
}

fn parse_spec(spec: &str) -> Result<Part, String> {
    if spec.is_empty() {
        return Ok(Part::positional());
    }

    PARTS_REGEX.captures(spec).map(|capture| {
        let color_spec: Option<Colors> = capture.name("color")
            .and_then(|it| parse_color(it.as_str()));

        let text_capture = capture.name("text");
        let text_spec: Text = text_capture.map(|text| {
            Indexed(text.as_str().parse::<i32>().unwrap() as usize)
        }).unwrap_or_else(|| Positional);

        Specification {
            text: text_spec,
            color: color_spec.unwrap_or_else(|| Colors::none()),
        }
    }).ok_or_else(|| format!("The specifier {} is invalid", spec))
}

lazy_static! {
    static ref COLOR_REGEX : Regex = Regex::new("^#(?<fg>[^/]+)?(/(?<bg>.+))?$").unwrap();
}

fn parse_color(so_far: &str) -> Option<Colors> {
    COLOR_REGEX.captures(so_far).map(|color| {
        let foreground = color.name("fg").map(|s| { interpret_color(s) });
        let background = color.name("bg").map(|s| { interpret_color(s) });
        Colors { foreground, background }
    })
}

fn interpret_color(s: Match) -> Color {
    match s.as_str() {
        "0" | "k" | "black" => { Color::black() }
        "1" | "r" | "red" => { Color::red() }
        "2" | "g" | "green" => { Color::green() }
        "3" | "y" | "yellow" => { Color::yellow() }
        "4" | "b" | "blue" => { Color::blue() }
        "5" | "m" | "magenta" => { Color::magenta() }
        "6" | "c" | "cyan" => { Color::cyan() }
        "7" | "w" | "white" => { Color::white() }

        "8" | "K" | "BLACK" => { Color::bright_black() }
        "9" | "R" | "RED" => { Color::bright_red() }
        "10" | "G" | "GREEN" => { Color::bright_green() }
        "11" | "Y" | "YELLOW" => { Color::bright_yellow() }
        "12" | "B" | "BLUE" => { Color::bright_blue() }
        "13" | "M" | "MAGENTA" => { Color::bright_magenta() }
        "14" | "C" | "CYAN" => { Color::bright_cyan() }
        "15" | "W" | "WHITE" => { Color::bright_white() }
        &_ => { todo!("Don't know how to interpret the foreground color {}", s.as_str()) }
    }
}


#[cfg(test)]
mod tests {
    use crate::model::{Color, Colors, Part, Text};
    use crate::model::Part::{Literal, Specification};
    use crate::parser::{parse_color, parse_format, parse_format_in_default_mode, parse_spec};

    // TODO detect invalid cases:
    // {garbage value}
    // TODO refuse to mix positional, indexed and named, only 1 of each

    #[test]
    fn parse_a_string_that_contains_no_spec_in_default_mode() {
        let specs = parse_format(&"Hello, format!".to_string());
        let ok = specs.ok().unwrap();
        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0], Literal("Hello, format!".to_string()));
    }

    #[test]
    fn parse_a_string_that_contains_no_spec_but_special_chars_in_default_mode() {
        let specs = parse_format(&r#"Look at those dirty chars: \{ \\ \}"#.to_string());
        let ok = specs.ok().unwrap();
        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0], Literal(r#"Look at those dirty chars: { \ }"#.to_string()));
    }

    #[test]
    fn parse_a_string_that_contains_1_spec_in_default_mode() {
        let specs = parse_format(&"Spec={}".to_string());
        let ok = specs.ok().unwrap();
        assert_eq!(ok.len(), 2);
        assert_eq!(ok[0], Literal("Spec=".to_string()));
        assert_eq!(ok[1], Specification { text: Text::Positional, color: Colors::none() });
    }

    #[test]
    fn parse_a_nested_format() {
        let specs = parse_format(&"Whatever {{}".to_string());
        let err = specs.err().unwrap();
        // TODO: improvement: tell the char that caused the issue
        assert_eq!(err, "Can't nest specifiers".to_string());
    }

    #[test]
    fn parse_an_imbalanced_format() {
        let specs = parse_format(&"Imbalanced {".to_string());
        let err = specs.err().unwrap();
        assert_eq!(err, "The specifiers are imbalanced: missing }".to_string());
    }

    // TODO: warn about extra arguments? pedantic mode?
    #[test]
    fn parse_an_empty_spec() {
        let specs = parse_spec("");
        let ok = specs.ok().unwrap();
        assert_eq!(ok, Part::positional());
    }

    #[test]
    fn parse_a_single_digit_index_spec() {
        let specs = parse_spec("%8");
        let ok = specs.ok().unwrap();
        assert_eq!(ok, Part::indexed(8));
    }

    #[test]
    fn parse_a_large_index_spec() {
        let specs = parse_spec("%314159265");
        let ok = specs.ok().unwrap();
        assert_eq!(ok, Part::indexed(314159265));
    }

    fn test_color_spec(spec: &str, color: Color) {
        let specs = parse_color(&spec.to_string());
        let ok = specs.unwrap();
        assert_eq!(ok, Colors::new_fg(color));
    }

    fn test_background_color_spec(spec: &str, color: Color) {
        let specs = parse_color(&spec.to_string());
        let ok = specs.unwrap();
        assert_eq!(ok, Colors::new_bg(color));
    }

    #[test]
    fn parse_black_specs() {
        test_color_spec("#0", Color::black());
        test_color_spec("#k", Color::black());
        test_color_spec("#black", Color::black());

        test_color_spec("#8", Color::bright_black());
        test_color_spec("#K", Color::bright_black());
        test_color_spec("#BLACK", Color::bright_black());
    }

    #[test]
    fn parse_red_specs() {
        test_color_spec("#1", Color::red());
        test_color_spec("#r", Color::red());
        test_color_spec("#red", Color::red());

        test_color_spec("#9", Color::bright_red());
        test_color_spec("#R", Color::bright_red());
        test_color_spec("#RED", Color::bright_red());
    }

    #[test]
    fn parse_green_specs() {
        test_color_spec("#2", Color::green());
        test_color_spec("#g", Color::green());
        test_color_spec("#green", Color::green());

        test_color_spec("#10", Color::bright_green());
        test_color_spec("#G", Color::bright_green());
        test_color_spec("#GREEN", Color::bright_green());
    }

    #[test]
    fn parse_yellow_specs() {
        test_color_spec("#3", Color::yellow());
        test_color_spec("#y", Color::yellow());
        test_color_spec("#yellow", Color::yellow());

        test_color_spec("#11", Color::bright_yellow());
        test_color_spec("#Y", Color::bright_yellow());
        test_color_spec("#YELLOW", Color::bright_yellow());
    }

    #[test]
    fn parse_blue_specs() {
        test_color_spec("#4", Color::blue());
        test_color_spec("#b", Color::blue());
        test_color_spec("#blue", Color::blue());

        test_color_spec("#12", Color::bright_blue());
        test_color_spec("#B", Color::bright_blue());
        test_color_spec("#BLUE", Color::bright_blue());
    }

    #[test]
    fn parse_magenta_specs() {
        test_color_spec("#5", Color::magenta());
        test_color_spec("#m", Color::magenta());
        test_color_spec("#magenta", Color::magenta());

        test_color_spec("#13", Color::bright_magenta());
        test_color_spec("#M", Color::bright_magenta());
        test_color_spec("#MAGENTA", Color::bright_magenta());
    }

    #[test]
    fn parse_cyan_specs() {
        test_color_spec("#6", Color::cyan());
        test_color_spec("#c", Color::cyan());
        test_color_spec("#cyan", Color::cyan());

        test_color_spec("#14", Color::bright_cyan());
        test_color_spec("#C", Color::bright_cyan());
        test_color_spec("#CYAN", Color::bright_cyan());
    }

    #[test]
    fn parse_white_specs() {
        test_color_spec("#7", Color::white());
        test_color_spec("#w", Color::white());
        test_color_spec("#white", Color::white());

        test_color_spec("#15", Color::bright_white());
        test_color_spec("#W", Color::bright_white());
        test_color_spec("#WHITE", Color::bright_white());
    }

    #[test]
    fn parse_background_specs() {
        test_background_color_spec("#/k", Color::black());
    }

    #[test]
    fn parse_fg_and_bg_specs() {
        let specs = parse_color("#k/w");
        let ok = specs.unwrap();
        assert_eq!(ok, Colors::new(Color::black(), Color::white()));
    }

    #[test]
    fn fail_gracefully_on_invalid_color_spec() {
        let specs = parse_spec("#");
        assert!(specs.is_err());

        let specs = parse_spec("/");
        assert!(specs.is_err());
    }

    fn check_backslash_notation(notation: &str, code: &str) {
        let specs = parse_format(&notation.to_string());
        let ok = &specs.unwrap()[0];
        assert_eq!(ok, &Part::literal(code));
    }

    #[test]
    fn interpret_backslash_a_as_bell() {
        check_backslash_notation(r#"\a"#, "\x07");
    }

    #[test]
    fn interpret_backslash_b_as_backspace() {
        check_backslash_notation(&r#"\b"#.to_string(), "\x08");
    }

    #[test]
    fn interpret_backslash_t_as_horizontal_tab() {
        check_backslash_notation(&r#"\t"#.to_string(), "\x09");
    }

    #[test]
    fn interpret_backslash_n_as_line_feed() {
        check_backslash_notation(&r#"\n"#.to_string(), "\x0a");
    }

    #[test]
    fn interpret_backslash_v_as_vertical_tab() {
        check_backslash_notation(&r#"\v"#.to_string(), "\x0b");
    }

    #[test]
    fn interpret_backslash_f_as_form_feed() {
        check_backslash_notation(&r#"\f"#.to_string(), "\x0c");
    }

    #[test]
    fn interpret_backslash_r_as_carriage_return() {
        check_backslash_notation(&r#"\r"#.to_string(), "\x0d");
    }

    #[test]
    fn interpret_backslash_e_as_escape() {
        check_backslash_notation(&r#"\e"#.to_string(), "\x1b");
    }
}
