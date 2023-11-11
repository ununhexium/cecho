use crate::model::Color::{Byte, RGB};
use crate::model::Part::{Literal, Specification};
use crate::model::Text::{AllArgs, Indexed, Positional};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Part {
    Literal(String),
    Specification {
        text: Text,
        color: Colors,
        styles: Vec<Style>,
    },
}

impl Part {
    pub fn literal(text: &str) -> Self {
        Literal(text.to_string())
    }
    pub fn positional() -> Self {
        Specification { text: Positional, color: Colors::none(), styles: vec!() }
    }
    pub fn all_args() -> Self {
        Specification { text: AllArgs(" ".to_string()), color: Colors::none(), styles: vec!() }
    }
    pub fn all_args_custom_separator(separator: &str) -> Self {
        Specification { text: AllArgs(separator.to_string()), color: Colors::none(), styles: vec!() }
    }
    pub fn positional_color(color: Color) -> Self {
        Specification { text: Positional, color: Colors::new_fg(color), styles: vec!() }
    }
    pub fn positional_background_color(color: Color) -> Self {
        Specification { text: Positional, color: Colors::new_bg(color), styles: vec!() }
    }
    pub fn positional_style(style: Style) -> Self {
        Specification { text: Positional, color: Colors::none(), styles: vec!(style) }
    }
    pub fn positional_styles(styles: Vec<Style>) -> Self {
        Specification { text: Positional, color: Colors::none(), styles: styles }
    }
    pub const fn indexed(index: usize) -> Self {
        Specification { text: Indexed(index), color: Colors::none(), styles: vec!() }
    }
    pub const fn indexed_color(index: usize, color: Colors) -> Self {
        Specification { text: Indexed(index), color, styles: vec!() }
    }
}

#[derive(PartialEq, Debug)]
pub enum Text {
    Positional,
    Indexed(usize),
    AllArgs(String),
}

#[derive(PartialEq, Debug)]
pub struct Colors {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl Colors {
    pub const fn none() -> Self {
        Colors { foreground: None, background: None }
    }
    pub const fn new(foreground: Color, background: Color) -> Self {
        Colors { foreground: Some(foreground), background: Some(background) }
    }
    pub const fn new_fg(foreground: Color) -> Self {
        Colors { foreground: Some(foreground), background: None }
    }
    pub const fn new_bg(background: Color) -> Self {
        Colors { foreground: None, background: Some(background) }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Color {
    Byte(u8),
    // ANSI color set
    RGB {
        red: u8,
        green: u8,
        blue: u8,
    },
}


// bit notation or bytes: 0b4321
// bit 1 is red, bit 2 is green, bit 3 is blue, 4 is bright
const RED: u8 = 0b0001;
const GREEN: u8 = 0b0010;
const BLUE: u8 = 0b0100;
const BRIGHT: u8 = 0b1000;

impl Color {
    pub const fn black() -> Self { Byte(0) }
    pub const fn red() -> Self { Byte(RED) }
    pub const fn green() -> Self { Byte(GREEN) }
    pub const fn yellow() -> Self { Byte(RED | GREEN) }
    pub const fn blue() -> Self { Byte(BLUE) }
    pub const fn magenta() -> Self { Byte(RED | BLUE) }
    pub const fn cyan() -> Self { Byte(GREEN | BLUE) }
    pub const fn white() -> Self { Byte(RED | GREEN | BLUE) }

    // because physics, lol XD
    pub const fn bright_black() -> Self { Byte(BRIGHT | 0) }
    pub const fn bright_red() -> Self { Byte(BRIGHT | RED) }
    pub const fn bright_green() -> Self { Byte(BRIGHT | GREEN) }
    pub const fn bright_yellow() -> Self { Byte(BRIGHT | RED | GREEN) }
    pub const fn bright_blue() -> Self { Byte(BRIGHT | BLUE) }
    pub const fn bright_magenta() -> Self { Byte(BRIGHT | RED | BLUE) }
    pub const fn bright_cyan() -> Self { Byte(BRIGHT | GREEN | BLUE) }
    pub const fn bright_white() -> Self { Byte(BRIGHT | RED | GREEN | BLUE) }

    pub fn u32_rgb(value: u32) -> Self {
        RGB {
            red: ((value >> 16) & 0xff) as u8,
            green: ((value >> 8) & 0xff) as u8,
            blue: (value & 0xff) as u8,
        }
    }
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self { RGB { red, green, blue } }

    pub fn escape_code(&self) -> String {
        let mut code = String::new();

        match self {
            Byte(b) => {
                if b < &8 {
                    code.push_str(&(30 + b).to_string());
                } else {
                    code.push_str(&(82 + b).to_string());
                }
            }
            RGB { red, green, blue } => {
                code.push_str("38;2;");
                code.push_str(&red.to_string());
                code.push(';');
                code.push_str(&green.to_string());
                code.push(';');
                code.push_str(&blue.to_string());
            }
        }

        code
    }

    pub fn as_ansi_background_escape_code(&self) -> String {
        let mut code = String::new();

        match self {
            Byte(b) => {
                if b < &8 {
                    code.push_str(&(40 + b).to_string());
                } else {
                    code.push_str(&(92 + b).to_string());
                }
            }
            RGB { red, green, blue } => {
                code.push_str("48;2;");
                code.push_str(&red.to_string());
                code.push(';');
                code.push_str(&green.to_string());
                code.push(';');
                code.push_str(&blue.to_string());
            }
        }

        code
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Style {
    Absent = 0,
    Strong = 1,
    Dim = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    Reversed = 7,
    Hidden = 8,
    CrossedOut = 9,
}
