use itertools::Itertools;
use crate::model::Color::Byte;
use crate::model::{Colors, Part};
use crate::model::Part::{Literal, Specification};
use crate::model::Text::{Indexed, Positional};

pub fn spec_to_ansi(inputs: &Vec<String>, specs: Vec<Part>) -> Result<String, String> {
    let mut position = 0;
    let result = specs.iter().map(|spec|
        match spec {
            Literal(literal) => { literal.to_string() }
            Specification { text: selector, color } => {
                let mut pre = String::new();
                let mut post = String::new();

                color.foreground.as_ref().map(|fg| {
                    let c = fg.as_ansi_foreground_escape_code();
                    pre.push_str(&c);
                });

                color.background.as_ref().map(|fg| {
                    let c = fg.as_ansi_background_escape_code();
                    pre.push_str(&c);
                });

                if color.foreground.is_some() || color.background.is_some() {
                    post.push_str("\x1b[0m")
                }

                let text = match selector {
                    Indexed(i) => {
                        &inputs[*i]
                    }
                    Positional => {
                        position += 1;
                        &inputs[position]
                    }
                };

                let mut full = String::new();
                full.push_str(&pre);
                full.push_str(text);
                full.push_str(&post);

                full
            }
        }
    ).join("");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::model::{Color, Colors, Part};
    use crate::vecs;
    use crate::writer::spec_to_ansi;

    #[test]
    fn output_escape_sequence_for_red_string_surrounded_by_hashes() {
        let result = spec_to_ansi(
            &vecs!("red"),
            vec!(
                Part::literal("##"),
                Part::indexed_color(0, Colors::new_fg(Color::red())),
                Part::literal("##"),
            ),
        );

        let ok = result.unwrap();

        assert_eq!(ok, "##\x1b[0;31mred\x1b[0m##");
    }

    #[test]
    fn output_escape_sequence_for_green_string_surrounded_by_hashes() {
        let result = spec_to_ansi(
            &vecs!("green"),
            vec!(
                Part::literal("##"),
                Part::indexed_color(0, Colors::new_fg(Color::green())),
                Part::literal("##"),
            ),
        );

        let ok = result.unwrap();

        assert_eq!(ok, "##\x1b[0;32mgreen\x1b[0m##");
    }

    #[test]
    fn output_escape_sequence_for_yellow_on_red() {
        let result = spec_to_ansi(
            &vecs!("DANGER"),
            vec!(
                Part::literal("##"),
                Part::indexed_color(
                    0,
                    Colors::new(
                        Color::yellow(),
                        Color::red()
                    )
                ),
                Part::literal("##"),
            ),
        );

        let ok = result.unwrap();

        assert_eq!(ok, "##\x1b[0;33m\x1b[0;41mDANGER\x1b[0m##");
    }
}
