use crate::model::Color::Byte;
use crate::model::Part::{Literal, Specification};
use crate::model::Text::{Indexed, Positional};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Part {
    Literal(String),
    Specification {
        text: Text,
        color: Colors,
    },
}

impl Part {
    pub fn literal(text: &str) -> Self {
        Literal(text.to_string())
    }
    pub fn positional() -> Self {
        Specification { text: Positional, color: Colors::none() }
    }
    pub const fn indexed(index: usize) -> Self {
        Specification { text: Indexed(index), color: Colors::none() }
    }
    pub const fn indexed_color(index: usize, color: Colors) -> Self {
        Specification { text: Indexed(index), color }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Text {
    Positional,
    Indexed(usize),
}

#[derive(PartialEq)]
#[derive(Debug)]
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
    Byte(u8) // ANSI color set
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

    pub fn as_ansi_foreground_escape_code(&self) -> String {
        let mut code = String::new();

        match self {
            Byte(b) => {
                code.push_str("\x1b[");
                if b < &8 {
                    code.push_str(&(30 + b).to_string());
                }else{
                    code.push_str(&(82 + b).to_string());
                }

                code.push_str("m");
            }
        }

        code
    }

    pub fn as_ansi_background_escape_code(&self) -> String {
        let mut code = String::new();

        match self {
            Byte(b) => {
                code.push_str("\x1b[");
                if b < &8 {
                    code.push_str(&(40 + b).to_string());
                }else{
                    code.push_str(&(92 + b).to_string());
                }

                code.push_str("m");
            }
        }

        code
    }
}
