use std::str::Chars;

use lazy_static::lazy_static;
use regex::{Match, Regex};

use crate::model::{Color, Colors, Part, Style, Text};
use crate::model::Part::{Literal, Specification};
use crate::model::Style::{Blink, Bold, Dim, Hidden, Invert, Italic, Strikethrough, Underline};
use crate::model::Text::{Indexed, Positional};
use crate::parser::ParserMode::{ColorMode, IndexMode, StyleMode};

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
                    if !so_far.is_empty() {
                        specs.push(Literal(so_far.to_string()));
                    }
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
                        Ok(Part::positional())
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

#[derive(Copy, Clone)]
enum ParserMode {
    IndexMode,
    ColorMode,
    StyleMode,
}

fn parse_spec(spec: &str) -> Result<Part, String> {
    if spec.is_empty() {
        return Ok(Part::positional());
    }

    let mut mode = None;

    let mut text = String::new();
    let mut color = String::new();
    let mut style = String::new();
    let mut last_word = String::new();

    for c in spec.chars() {
        match c {
            '#' => mode = Some(ColorMode),
            '%' => mode = Some(IndexMode),
            '!' => mode = Some(StyleMode),
            ' ' | '\t' => {
                last_word.clear();
                mode = None;
            }
            '=' => {
                match last_word.as_str() {
                    "color" => {
                        mode = Some(ColorMode)
                    }
                    "index" => {
                        mode = Some(IndexMode)
                    }
                    "style" => {
                        mode = Some(StyleMode)
                    }
                    _ => panic!("Don't know how to interpret the keyword '{}' as a mode", last_word),
                }
            }
            _ => {
                last_word.push(c);

                match mode {
                    Some(m) => match m {
                        IndexMode => text.push(c),
                        ColorMode => color.push(c),
                        StyleMode => style.push(c),
                    },
                    None => { /* TODO this could be used to parse raw strings in the format */ }
                }
            }
        }
    }

    let color_spec: Option<Colors> = parse_color(&color.as_str());
    let style_spec = parse_style(style);
    let text_spec = match text.trim() {
        "" => Positional,
        _ => {
            text.as_str().trim().parse::<usize>().and_then(|it| Ok(Indexed(it))).unwrap_or_else(|it|
                panic!("Don't know how to interpret the text specification '{}'", text)
            )
        }
    };

    Ok(
        Specification {
            text: text_spec,
            color: color_spec.unwrap_or_else(|| Colors::none()),
            style: style_spec,
        }
    )
}

lazy_static! {
    static ref COLOR_REGEX : Regex = Regex::new("^\\s*(?<fg>[^/]+)?\\s*(/\\s*(?<bg>.+))?\\s*$").unwrap();
}

fn parse_color(so_far: &str) -> Option<Colors> {
    COLOR_REGEX.captures(so_far.trim()).map(|color| {
        let foreground = color.name("fg").map(|s| { interpret_color(s) });
        let background = color.name("bg").map(|s| { interpret_color(s) });
        Colors { foreground, background }
    })
}

fn interpret_color(s: Match) -> Color {
    match s.as_str().trim() {
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
        &_ => { todo!("Don't know how to interpret the foreground color '{}'", s.as_str()) }
    }
}

fn parse_style(style: String) -> Option<Style> {
    // TODO: make it not case sensitive
    match style.as_str().trim() {
        "bold" => Some(Bold),
        "dim" | "faint" => Some(Dim),
        "italic" => Some(Italic),
        "underline" => Some(Underline),
        "blink" | "blinking" => Some(Blink),
        "invert" | "inverted" | "inverse" | "reversed" | "reverse" => Some(Invert),
        "hidden" | "invisible" => Some(Hidden),
        "strikethrough" | "strike" => Some(Strikethrough),
        "" => None,
        _ => panic!("Don't know how to interpret the style '{}'", style),
    }
}


#[cfg(test)]
mod tests {
    use crate::model::{Color, Colors, Part, Text};
    use crate::model::Part::{Literal, Specification};
    use crate::model::Style::{Blink, Bold, Dim, Hidden, Invert, Italic, Strikethrough, Underline};
    use crate::parser::{parse_color, parse_format, parse_format_in_default_mode, parse_spec};

    // TODO detect invalid cases:
    // {garbage value}
    // TODO refuse to mix positional, indexed and named, only 1 of each

    fn test_ok_format(format: &str, parts: Vec<Part>) {
        let specs = parse_format(&format.to_string());
        let ok = specs.ok().unwrap();
        assert_eq!(ok.len(), parts.len());

        for i in 0..=parts.len() - 1 {
            assert_eq!(ok[i], parts[i]);
        }
    }

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
        assert_eq!(ok[1], Part::positional());
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

    #[test]
    fn the_symbol_for_the_index_is_percent() {
        test_ok_format("{%1}", vec!(Part::indexed(1)));
    }

    #[test]
    fn the_name_for_the_index_is_index() {
        test_ok_format("{index=1}", vec!(Part::indexed(1)));
    }

    #[test]
    fn the_symbol_for_the_color_is_hash() {
        test_ok_format("{#red}", vec!(Part::positional_color(Color::red())));
    }

    #[test]
    fn the_name_for_the_color_is_color() {
        test_ok_format("{color=red}", vec!(Part::positional_color(Color::red())));
    }

    #[test]
    fn the_symbol_for_the_font_style_is_exclamation_mark() {
        test_ok_format("{!bold}", vec!(Part::positional_style(Bold)));
    }

    #[test]
    fn the_name_for_the_font_style_is_style() {
        test_ok_format("{style=bold}", vec!(Part::positional_style(Bold)));
    }

    fn parse_ok_spec(spec: &str, expected: Part) {
        let specs = parse_spec(spec);
        let ok = specs.ok().unwrap();
        assert_eq!(ok, expected);
    }

    // TODO: warn about extra arguments? pedantic mode?
    #[test]
    fn parse_an_empty_spec() {
        let specs = parse_spec("");
        let ok = specs.ok().unwrap();
        assert_eq!(ok, Part::positional());

        parse_ok_spec("", Part::positional())
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

    #[test]
    fn the_specifiers_can_be_given_in_any_order() {
        let specs1 = parse_spec("%1#red");
        let specs2 = parse_spec("#red%1");

        let ok1 = specs1.ok().unwrap();
        let ok2 = specs2.ok().unwrap();

        let expected = Part::indexed_color(1, Colors::new_fg(Color::red()));
        assert_eq!(ok1, expected);
        assert_eq!(ok2, expected);
    }

    #[test]
    fn the_specifiers_may_be_surrounded_by_spaces() {
        let specs = parse_spec(" %1 \t #red/magenta ");

        let ok = specs.ok().unwrap();

        let expected = Part::indexed_color(1, Colors::new(Color::red(), Color::magenta()));
        assert_eq!(ok, expected);
    }

    #[test]
    fn the_specifiers_can_be_named() {
        let specs = parse_spec("index=1 color=red/magenta");

        let ok = specs.ok().unwrap();

        let expected = Part::indexed_color(1, Colors::new(Color::red(), Color::magenta()));
        assert_eq!(ok, expected);
    }

    #[test]
    fn the_specifiers_styles_can_be_mixed() {
        let specs = parse_spec("%1 color=red/magenta");

        let ok = specs.ok().unwrap();

        let expected = Part::indexed_color(1, Colors::new(Color::red(), Color::magenta()));
        assert_eq!(ok, expected);
    }

    // color specifiers

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
        test_color_spec("0", Color::black());
        test_color_spec("k", Color::black());
        test_color_spec("black", Color::black());

        test_color_spec("8", Color::bright_black());
        test_color_spec("K", Color::bright_black());
        test_color_spec("BLACK", Color::bright_black());
    }

    #[test]
    fn parse_red_specs() {
        test_color_spec("1", Color::red());
        test_color_spec("r", Color::red());
        test_color_spec("red", Color::red());

        test_color_spec("9", Color::bright_red());
        test_color_spec("R", Color::bright_red());
        test_color_spec("RED", Color::bright_red());
    }

    #[test]
    fn parse_green_specs() {
        test_color_spec("2", Color::green());
        test_color_spec("g", Color::green());
        test_color_spec("green", Color::green());

        test_color_spec("10", Color::bright_green());
        test_color_spec("G", Color::bright_green());
        test_color_spec("GREEN", Color::bright_green());
    }

    #[test]
    fn parse_yellow_specs() {
        test_color_spec("3", Color::yellow());
        test_color_spec("y", Color::yellow());
        test_color_spec("yellow", Color::yellow());

        test_color_spec("11", Color::bright_yellow());
        test_color_spec("Y", Color::bright_yellow());
        test_color_spec("YELLOW", Color::bright_yellow());
    }

    #[test]
    fn parse_blue_specs() {
        test_color_spec("4", Color::blue());
        test_color_spec("b", Color::blue());
        test_color_spec("blue", Color::blue());

        test_color_spec("12", Color::bright_blue());
        test_color_spec("B", Color::bright_blue());
        test_color_spec("BLUE", Color::bright_blue());
    }

    #[test]
    fn parse_magenta_specs() {
        test_color_spec("5", Color::magenta());
        test_color_spec("m", Color::magenta());
        test_color_spec("magenta", Color::magenta());

        test_color_spec("13", Color::bright_magenta());
        test_color_spec("M", Color::bright_magenta());
        test_color_spec("MAGENTA", Color::bright_magenta());
    }

    #[test]
    fn parse_cyan_specs() {
        test_color_spec("6", Color::cyan());
        test_color_spec("c", Color::cyan());
        test_color_spec("cyan", Color::cyan());

        test_color_spec("14", Color::bright_cyan());
        test_color_spec("C", Color::bright_cyan());
        test_color_spec("CYAN", Color::bright_cyan());
    }

    #[test]
    fn parse_white_specs() {
        test_color_spec("7", Color::white());
        test_color_spec("w", Color::white());
        test_color_spec("white", Color::white());

        test_color_spec("15", Color::bright_white());
        test_color_spec("W", Color::bright_white());
        test_color_spec("WHITE", Color::bright_white());
    }

    #[test]
    fn parse_background_specs() {
        test_background_color_spec("/k", Color::black());
    }

    #[test]
    fn parse_foreground_and_background_specs() {
        let specs = parse_color("k/w");
        let ok = specs.unwrap();
        assert_eq!(ok, Colors::new(Color::black(), Color::white()));
    }

    // escape sequences

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

    // font style

    #[test]
    fn parse_bold_style() {
        parse_ok_spec("style=bold", Part::positional_style(Bold));
        parse_ok_spec("!bold", Part::positional_style(Bold));
    }

    #[test]
    fn parse_dim_style() {
        parse_ok_spec("style=dim", Part::positional_style(Dim));
        parse_ok_spec("style=faint", Part::positional_style(Dim));
        parse_ok_spec("!dim", Part::positional_style(Dim));
        parse_ok_spec("!faint", Part::positional_style(Dim));
    }

    #[test]
    fn parse_italic_style() {
        parse_ok_spec("style=italic", Part::positional_style(Italic));
        parse_ok_spec("!italic", Part::positional_style(Italic));
    }

    #[test]
    fn parse_underline_style() {
        parse_ok_spec("style=underline", Part::positional_style(Underline));
        parse_ok_spec("!underline", Part::positional_style(Underline));
    }

    #[test]
    fn parse_blink_style() {
        parse_ok_spec("style=blink", Part::positional_style(Blink));
        parse_ok_spec("style=blinking", Part::positional_style(Blink));
        parse_ok_spec("!blink", Part::positional_style(Blink));
        parse_ok_spec("!blinking", Part::positional_style(Blink));
    }

    #[test]
    fn parse_invert_style() {
        parse_ok_spec("style=invert", Part::positional_style(Invert));
        parse_ok_spec("style=inverted", Part::positional_style(Invert));
        parse_ok_spec("style=inverse", Part::positional_style(Invert));
        parse_ok_spec("style=reverse", Part::positional_style(Invert));
        parse_ok_spec("style=reversed", Part::positional_style(Invert));
        parse_ok_spec("!invert", Part::positional_style(Invert));
        parse_ok_spec("!inverted", Part::positional_style(Invert));
        parse_ok_spec("!inverse", Part::positional_style(Invert));
        parse_ok_spec("!reverse", Part::positional_style(Invert));
        parse_ok_spec("!reversed", Part::positional_style(Invert));
    }

    #[test]
    fn parse_hidden_style() {
        parse_ok_spec("style=hidden", Part::positional_style(Hidden));
        parse_ok_spec("style=invisible", Part::positional_style(Hidden));
        parse_ok_spec("!hidden", Part::positional_style(Hidden));
        parse_ok_spec("!invisible", Part::positional_style(Hidden));
    }

    #[test]
    fn parse_strikethrough_style() {
        parse_ok_spec("style=strikethrough", Part::positional_style(Strikethrough));
        parse_ok_spec("style=strike", Part::positional_style(Strikethrough));
        parse_ok_spec("!strikethrough", Part::positional_style(Strikethrough));
        parse_ok_spec("!strike", Part::positional_style(Strikethrough));
    }
}
