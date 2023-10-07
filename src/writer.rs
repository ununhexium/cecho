use itertools::Itertools;
use crate::model::Color::Byte;
use crate::model::{ColorPair, Part};
use crate::model::Part::{Literal, Specification};
use crate::model::Text::{Indexed, Positional};

pub fn spec_to_ansi(inputs: &Vec<String>, specs: Vec<Part>) -> Result<String, String> {
    let mut position = 0;
    let result = specs.iter().map(|spec|
        match spec {
            Literal(literal) => { literal.to_string() }
            Specification { text: selector, color } => {
                let surroundings = match color {
                    Some(ColorPair { foreground: fg, background: _ }) => {
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

#[cfg(test)]
mod tests {
    use crate::model::{Color, ColorPair, Part};
    use crate::vecs;
    use crate::writer::spec_to_ansi;

    #[test]
    fn output_escape_sequence_for_red_string_surrounded_by_hashes() {
        let result = spec_to_ansi(
            &vecs!("string"),
            vec!(
                Part::literal("##"),
                Part::indexed_color(0, ColorPair::new_fg(Color::red())),
                Part::literal("##"),
            ),
        );

        let ok = result.unwrap();

        assert_eq!(ok, "##\x1b[0;31mstring\x1b[0m##");
    }
}
