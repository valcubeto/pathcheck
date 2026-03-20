use owo_colors::{ OwoColorize, Stream as OwoStream };
use crate::args::WhenColors;

pub fn green(text: &str, color: WhenColors) -> String {
    match color {
        WhenColors::Never => text.to_string(),
        WhenColors::Always => text.green().to_string(),
        WhenColors::Auto => {
            text.if_supports_color(
                OwoStream::Stdout,
                OwoColorize::green
            ).to_string()
        }
    }
}

pub fn red(text: &str, color: WhenColors) -> String {
    match color {
        WhenColors::Never => text.to_string(),
        WhenColors::Always => text.red().to_string(),
        WhenColors::Auto => {
            text.if_supports_color(
                OwoStream::Stdout,
                OwoColorize::red
            ).to_string()
        }
    }
}
