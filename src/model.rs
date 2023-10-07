use crate::model::Color::Byte;
use crate::model::Part::{Literal, Specification};
use crate::model::Text::{Indexed, Positional};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Part {
    Literal(String),
    Specification {
        text: Text,
        color: Option<ColorPair>,
    },
}

impl Part {
    pub fn literal(text: &str) -> Self {
        Literal(text.to_string())
    }
    pub const fn indexed(index: usize) -> Self {
        Specification { text: Indexed(index), color: None }
    }
    pub const fn indexed_color(index: usize, color: ColorPair) -> Self {
        Specification { text: Indexed(index), color: Some(color) }
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
pub struct ColorPair {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl ColorPair {
    pub const fn new_fg(foreground: Color) -> Self {
        ColorPair { foreground: Some(foreground), background: None }
    }
    pub const fn new_bg(background: Color) -> Self {
        ColorPair { foreground: None, background: Some(background) }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Color {
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
