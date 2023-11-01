use itertools::Itertools;

use crate::model::Part;
use crate::model::Part::{Literal, Specification};
use crate::model::Text::{Indexed, Positional};

pub fn spec_to_ansi(inputs: &[String], specs: Vec<Part>) -> Result<String, String> {
    let mut position = 0;
    let result = specs.iter().map(|spec|
        match spec {
            Literal(literal) => { literal.to_string() }
            Specification { text: selector, color, style } => {
                let mut pre = String::new();
                let mut post = String::new();

                // prepare to add color or style
                if !style.is_empty() || color.foreground.is_some() {
                    pre.push_str("\x1b[");
                }

                // reset the color and style
                if !style.is_empty() || color.foreground.is_some() || color.background.is_some() {
                    post.push_str("\x1b[0m")
                }

                style.iter().for_each(|s|
                    pre.push_str((*s as i32).to_string().as_str())
                );

                color.foreground.as_ref().map(|fg| {
                    if !style.is_empty() {
                        pre.push(';');
                    }
                    pre.push_str(&fg.escape_code());
                });

                if !style.is_empty() || color.foreground.is_some() {
                    pre.push('m');
                }

                color.background.as_ref().map(|fg| {

                    pre.push_str("\x1b[");
                    let c = fg.as_ansi_background_escape_code();
                    pre.push_str(&c);
                    pre.push_str("m");
                });

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
    use crate::model::Style::{Blink, Bold};
    use crate::vecs;
    use crate::writer::spec_to_ansi;

    fn test_ok_spec_to_ansi(mut inputs: Vec<String>, parts: Vec<Part>, expected: &str) {
        inputs.insert(0, "unused but necessary because this is the place of the formatter".to_string());
        let result = spec_to_ansi(&inputs, parts);
        let ok = result.unwrap();
        assert_eq!(ok, expected);
    }

    #[test]
    fn output_escape_sequence_for_red_string_surrounded_by_hashes() {
        test_ok_spec_to_ansi(
            vecs!("red"),
            vec!(
                Part::literal("##"),
                Part::indexed_color(1, Colors::new_fg(Color::red())),
                Part::literal("##"),
            ),
            "##\x1b[31mred\x1b[0m##",
        )
    }

    #[test]
    fn output_escape_sequence_for_green_string_surrounded_by_hashes() {
        test_ok_spec_to_ansi(
            vecs!("green"),
            vec!(
                Part::literal("##"),
                Part::indexed_color(1, Colors::new_fg(Color::green())),
                Part::literal("##"),
            ),
            "##\x1b[32mgreen\x1b[0m##",
        )
    }

    #[test]
    fn output_escape_sequence_for_yellow_on_red() {
        test_ok_spec_to_ansi(
            vecs!("DANGER"),
            vec!(
                Part::literal("##"),
                Part::indexed_color(
                    1,
                    Colors::new(
                        Color::yellow(),
                        Color::red(),
                    ),
                ),
                Part::literal("##"),
            ),
            "##\x1b[33m\x1b[41mDANGER\x1b[0m##");
    }

    #[test]
    fn output_bold_style_option_in_ansi_sequence() {
        test_ok_spec_to_ansi(
            vecs!("Bald"),
            vec!(
                Part::literal("##"),
                Part::positional_style(Bold),
                Part::literal("##"),
            ),
            "##\x1b[1mBald\x1b[0m##");
    }

    #[test]
    fn output_blink_style_option_in_ansi_sequence() {
        test_ok_spec_to_ansi(
            vecs!("TikTok"),
            vec!(
                Part::literal("##"),
                Part::positional_style(Blink),
                Part::literal("##"),
            ),
            "##\x1b[5mTikTok\x1b[0m##");
    }

    #[test]
    fn output_rgb_color_brown() {
        test_ok_spec_to_ansi(
            vecs!("Poop"),
            vec!(
                Part::literal("##"),
                Part::positional_color(Color::rgb(84,55,15)),
                Part::literal("##"),
            ),
            "##\x1b[38;2;84;55;15mPoop\x1b[0m##");
    }

    #[test]
    fn output_rgb_background_color_brown() {
        test_ok_spec_to_ansi(
            vecs!("Poop"),
            vec!(
                Part::literal("##"),
                Part::positional_background_color(Color::rgb(84,55,15)),
                Part::literal("##"),
            ),
            "##\x1b[48;2;84;55;15mPoop\x1b[0m##");
    }

}
