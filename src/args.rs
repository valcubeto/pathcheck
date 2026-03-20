use crate::strings;
use crate::utils::{ green, red };
use clap::{ Parser, ValueEnum };

#[derive(Parser, Debug)]
#[command(version, about, after_long_help = { strings::AFTER_HELP.trim_matches('\n') })]
pub struct Args {
    #[arg(long = "color", value_enum, default_value = "auto", alias = "colors", id = "WHEN")]
    pub colorize: WhenColors,

    /// Enumerate paths
    #[arg(short, long)]
    pub enumerate: bool,

    /// Pad numbers with zeroes instead of spaces.
    #[arg(short, long)]
    pub zero_padding: bool,

    #[arg(short, long, value_enum, default_value = "text", id = "STYLE")]
    pub status_style: StatusStyle,

    /// Self-explanatory
    #[arg(short = 'd', long)]
    pub show_status_description: bool,

    /// Specify a format for each path
    #[arg(
        long,
        default_value = "{line :0}{status :<}{path}",
        hide_default_value = true
    )]
    pub format: String,

    // pub hide_invalid_paths
    // pub no_duplicates: bool
}

#[derive(ValueEnum, Debug, Clone)]
pub enum StatusStyle {
    Icons,
    Emoji,
    Text,
    None,
}

impl StatusStyle {
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn get_status_str(&self, colorize: WhenColors) -> Option<(String, String)> {
        match self {
            StatusStyle::Icons => Some((
                green(strings::OK_ICON, colorize),
                red(strings::ERR_ICON, colorize)
            )),
            StatusStyle::Text => Some((
                green(strings::OK_TEXT, colorize),
                red(strings::ERR_TEXT, colorize)
            )),
            StatusStyle::Emoji => Some((
                strings::OK_EMOJI.to_string(),
                strings::ERR_EMOJI.to_string()
            )),
            StatusStyle::None => None
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum WhenColors {
    Never,
    Always,
    Auto,
}
